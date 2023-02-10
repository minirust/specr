use crate::*;

pub fn translate_rvalue<'tcx>(rv: &rs::Rvalue<'tcx>, fcx: &mut FnCtxt<'tcx>) -> Option<ValueExpr> {
    Some(match rv {
        rs::Rvalue::Use(operand) => translate_operand(operand, fcx),
        rs::Rvalue::CheckedBinaryOp(bin_op, box (l, r)) | rs::Rvalue::BinaryOp(bin_op, box (l, r)) => {
            let lty = l.ty(&fcx.body, fcx.tcx);
            let rty = r.ty(&fcx.body, fcx.tcx);

            assert_eq!(lty, rty);

            let l = translate_operand(l, fcx);
            let r = translate_operand(r, fcx);

            let l = GcCow::new(l);
            let r = GcCow::new(r);

            use rs::BinOp::*;
            let op = if *bin_op == Offset {
                BinOp::PtrOffset {
                    inbounds: true // TODO is rs::BinOp::Offset always `inbounds`?
                }
            } else { // everything else right-now is a int op!
                let op_int = match bin_op {
                    Add => BinOpInt::Add,
                    Sub => BinOpInt::Sub,
                    Mul => BinOpInt::Mul,
                    Div => BinOpInt::Div,
                    Lt => return None, // This is IGNORED. It's generated in bounds checking.
                    Eq => return None, // This is IGNORED. It's generated in div-zero checking.
                    BitAnd => return None,
                    x => {
                        dbg!(x);
                        todo!("unsupported BinOp")
                    },
                };

                let Type::Int(int_ty) = translate_ty(lty, fcx.tcx) else {
                    panic!("arithmetic operation with non-int type unsupported!");
                };

                BinOp::Int(op_int, int_ty)
            };

            ValueExpr::BinOp {
                operator: op,
                left: l,
                right: r,
            }
        },
        rs::Rvalue::UnaryOp(unop, operand) => {
            match unop {
                rs::UnOp::Neg => {
                    let ty = operand.ty(&fcx.body, fcx.tcx);
                    let ty = translate_ty(ty, fcx.tcx);
                    let Type::Int(int_ty) = ty else {
                        panic!("Neg operation with non-int type!");
                    };

                    let operand = translate_operand(operand, fcx);

                    ValueExpr::UnOp {
                        operator: UnOp::Int(UnOpInt::Neg, int_ty),
                        operand: GcCow::new(operand),
                    }
                },
                _ => panic!("unsupported UnOp!"),
            }
        }
        rs::Rvalue::Ref(_, bkind, place) => {
            let ty = place.ty(&fcx.body, fcx.tcx).ty;
            let pointee = layout_of(ty, fcx.tcx);

            let place = translate_place(place, fcx);
            let target = GcCow::new(place);
            let mutbl = translate_mutbl(bkind.to_mutbl_lossy());

            let ptr_ty = PtrType::Ref { mutbl, pointee };

            ValueExpr::AddrOf { target, ptr_ty }
        },
        rs::Rvalue::AddressOf(_mutbl, place) => {
            let ty = place.ty(&fcx.body, fcx.tcx).ty;
            let pointee = layout_of(ty, fcx.tcx);

            let place = translate_place(place, fcx);
            let target = GcCow::new(place);

            let ptr_ty = PtrType::Raw { pointee };

            ValueExpr::AddrOf { target, ptr_ty }
        },
        rs::Rvalue::Aggregate(box rs::AggregateKind::Array(ty), operands) => {
            let count = Int::from(operands.len());
            let ty = translate_ty(*ty, fcx.tcx);
            let ty = Type::Array { elem: GcCow::new(ty), count };
            let ops: List<_> = operands.iter().map(|x| translate_operand(x, fcx)).collect();
            ValueExpr::Tuple(ops, ty)
        },
        rs::Rvalue::CopyForDeref(place) => {
            ValueExpr::Load {
                destructive: false,
                source: GcCow::new(translate_place(place, fcx)),
            }
        },
        rs::Rvalue::Len(..) => return None, // This is IGNORED. It's generated due to bounds checking.
        rs::Rvalue::Cast(rs::CastKind::IntToInt, operand, ty) => {
            let operand = translate_operand(operand, fcx);
            let Type::Int(int_ty) = translate_ty(*ty, fcx.tcx) else {
                panic!("attempting to IntToInt-Cast to non-int type!");
            };

            ValueExpr::UnOp {
                operator: UnOp::Int(UnOpInt::Cast, int_ty),
                operand: GcCow::new(operand),
            }
        },
        rs::Rvalue::Cast(rs::CastKind::PointerExposeAddress, operand, _) => {
            let operand = translate_operand(operand, fcx);

            ValueExpr::UnOp {
                operator: UnOp::Ptr2Int,
                operand: GcCow::new(operand),
            }
        },
        rs::Rvalue::Cast(rs::CastKind::PointerFromExposedAddress, operand, ty) => {
            // TODO untested so far! (Can't test because of `predict`)
            let operand = translate_operand(operand, fcx);
            let Type::Ptr(ptr_ty) = translate_ty(*ty, fcx.tcx) else { panic!() };

            ValueExpr::UnOp {
                operator: UnOp::Int2Ptr(ptr_ty),
                operand: GcCow::new(operand),
            }
        },
        x => {
            dbg!(x);
            todo!()
        }
    })
}

pub fn translate_const<'tcx>(c: &rs::Constant<'tcx>, fcx: &mut FnCtxt<'tcx>) -> ValueExpr {
    let kind = c.literal.eval(fcx.tcx, rs::ParamEnv::empty());
    match kind {
        rs::ConstantKind::Val(val, ty) => {
            let ty = translate_ty(ty, fcx.tcx);
            let constant = match ty {
                Type::Int(int_ty) => {
                    let val = val.try_to_scalar_int().unwrap();
                    let int: Int = match int_ty.signed {
                        Signed => val.try_to_int(val.size()).unwrap().into(),
                        Unsigned => val.try_to_uint(val.size()).unwrap().into(),
                    };
                    Constant::Int(int)
                },
                // unit type `()`
                Type::Tuple { fields, .. } if fields.is_empty() => { // TODO are other tuples supported correctly?
                    return ValueExpr::Tuple(List::new(), Type::Tuple { fields: List::new(), size: Size::ZERO })
                }
                Type::Bool => {
                    Constant::Bool(val.try_to_bool().unwrap())
                }
                Type::Ptr(_) => {
                    let _val = val.try_to_scalar()
                                 .unwrap()
                                 .to_pointer(&fcx.tcx)
                                 .unwrap();
                    panic!("minirust doesn't yet support constant pointers!")
                },
                x => {
                    dbg!(x);
                    todo!()
                }
            };
            ValueExpr::Constant(constant, ty)
        }
        x => {
            dbg!(x);
            todo!()
        }
    }
}

pub fn translate_operand<'tcx>(operand: &rs::Operand<'tcx>, fcx: &mut FnCtxt<'tcx>) -> ValueExpr {
    match operand {
        rs::Operand::Constant(box c) => translate_const(c, fcx),
        rs::Operand::Copy(place) => {
            ValueExpr::Load {
                destructive: false,
                source: GcCow::new(translate_place(place, fcx)),
            }
        },
        rs::Operand::Move(place) => {
            ValueExpr::Load {
                destructive: true,
                source: GcCow::new(translate_place(place, fcx)),
            }
        },
    }
}

fn place_type_of<'tcx>(ty: rs::Ty<'tcx>, fcx: &mut FnCtxt<'tcx>) -> PlaceType {
    let align = layout_of(ty, fcx.tcx).align;
    let ty = translate_ty(ty, fcx.tcx);

    PlaceType { ty, align }
}

pub fn translate_place<'tcx>(place: &rs::Place<'tcx>, fcx: &mut FnCtxt<'tcx>) -> PlaceExpr {
    let mut expr = PlaceExpr::Local(fcx.local_name_map[&place.local]);
    for (i, proj) in place.projection.iter().enumerate() {
        match proj {
            rs::ProjectionElem::Field(f, _ty) => {
                let f = f.index();
                let indirected = GcCow::new(expr);
                expr = PlaceExpr::Field {
                    root: indirected,
                    field: f.into(),
                };
            },
            rs::ProjectionElem::Deref => {
                let x = GcCow::new(expr);
                let x = ValueExpr::Load {
                    destructive: false,
                    source: x
                };
                let x = GcCow::new(x);

                let ty = rs::Place::ty_from(place.local, &place.projection[..(i+1)], &fcx.body, fcx.tcx).ty;
                let ptype = place_type_of(ty, fcx);

                expr = PlaceExpr::Deref {
                    operand: x,
                    ptype,
                };
            },
            rs::ProjectionElem::Index(loc) => {
                let i = PlaceExpr::Local(fcx.local_name_map[&loc]);
                let i = GcCow::new(i);
                let i = ValueExpr::Load {
                    destructive: false,
                    source: i,
                };
                let i = GcCow::new(i);
                let root = GcCow::new(expr);
                expr = PlaceExpr::Index { root, index: i };
            },
            x => todo!("{:?}", x),
        }
    }
    expr
}

