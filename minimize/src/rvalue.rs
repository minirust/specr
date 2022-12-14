use crate::*;

pub fn translate_rvalue<'tcx>(rv: &rs::Rvalue<'tcx>, fcx: &mut FnCtxt<'tcx>) -> Option<ValueExpr> {
    Some(match rv {
        rs::Rvalue::Use(operand) => translate_operand(operand, fcx),
        rs::Rvalue::CheckedBinaryOp(bin_op, box (l, r)) | rs::Rvalue::BinaryOp(bin_op, box (l, r)) => {
            let lty = l.ty(&fcx.body, fcx.tcx);
            let rty = r.ty(&fcx.body, fcx.tcx);

            assert_eq!(lty, rty);

            let Type::Int(int_ty) = translate_ty(lty, fcx.tcx) else {
                panic!("arithmetic operation with non-int type unsupported!");
            };

            let l = translate_operand(l, fcx);
            let r = translate_operand(r, fcx);

            let l = GcCow::new(l);
            let r = GcCow::new(r);

            use rs::BinOp::*;
            let op = if *bin_op == Offset {
                BinOp::PtrOffset {
                    inbounds: true // FIXME where to find this bool `inbounds` in mir?
                }
            } else { // everything else right-now is a int op!
                let op_int = match bin_op {
                    Add => BinOpInt::Add,
                    Sub => BinOpInt::Sub,
                    Mul => BinOpInt::Mul,
                    Div => BinOpInt::Div,
                    Lt => return None, // This is IGNORED. It's generated in bounds checking.
                    x => {
                        dbg!(x);
                        todo!("unsupported BinOp")
                    },
                };
                BinOp::Int(op_int, int_ty)
            };

            ValueExpr::BinOp {
                operator: op,
                left: l,
                right: r,
            }
        },
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
            let ops: List<_> = operands.iter().map(|x| {
                let op = translate_operand(x, fcx);
                let ValueExpr::Constant(c, _) = op else {
                    panic!("non-constants in array-expr not supported!");
                };

                c
            }).collect();
            let c = Constant::Tuple(ops);
            ValueExpr::Constant(c, ty)
        },
        rs::Rvalue::CopyForDeref(place) => {
            ValueExpr::Load {
                destructive: false,
                source: GcCow::new(translate_place(place, fcx)),
            }
        },
        rs::Rvalue::Len(..) => return None, // This is IGNORED. It's generated due to bounds checking.
        x => {
            dbg!(x);
            todo!()
        }
    })
}

pub fn translate_operand<'tcx>(operand: &rs::Operand<'tcx>, fcx: &mut FnCtxt<'tcx>) -> ValueExpr {
    match operand {
        rs::Operand::Constant(box c) => {
            match c.literal {
                rs::ConstantKind::Val(val, ty) => {
                    let ty = translate_ty(ty, fcx.tcx);
                    let constant = match ty {
                        Type::Int(int_ty) => {
                            let val = val.try_to_scalar_int().unwrap();

                            use Signedness::*;
                            let bits = int_to_usize(int_ty.size.bits());
                            // TODO is there no better way to get the value from a ScalarInt?
                            let int: Int = match (int_ty.signed, bits) {
                                (Signed, 8) => val.try_to_i8().unwrap().into(),
                                (Signed, 16) => val.try_to_i16().unwrap().into(),
                                (Signed, 32) => val.try_to_i32().unwrap().into(),
                                (Signed, 64) => val.try_to_i64().unwrap().into(),
                                (Signed, 128) => val.try_to_i128().unwrap().into(),

                                (Unsigned, 8) => val.try_to_u8().unwrap().into(),
                                (Unsigned, 16) => val.try_to_u16().unwrap().into(),
                                (Unsigned, 32) => val.try_to_u32().unwrap().into(),
                                (Unsigned, 64) => val.try_to_u64().unwrap().into(),
                                (Unsigned, 128) => val.try_to_u128().unwrap().into(),
                                _ => panic!("unsupported integer type encountered!"),
                            };
                            Constant::Int(int)
                        },
                        // unit type `()`
                        Type::Tuple { fields, .. } if fields.is_empty() => {
                            Constant::Tuple(List::new())
                        }
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
        },
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
    let mut expr = PlaceExpr::Local(fcx.localname_map[&place.local]);
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
                let i = PlaceExpr::Local(fcx.localname_map[&loc]);
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

