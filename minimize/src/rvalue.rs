use crate::*;

pub fn translate_rvalue<'tcx>(rv: &rs::Rvalue<'tcx>, fcx: &mut FnCtxt<'tcx>) -> mini::ValueExpr {
    match rv {
        rs::Rvalue::Use(operand) => translate_operand(operand, fcx),
        rs::Rvalue::CheckedBinaryOp(bin_op, box (l, r)) | rs::Rvalue::BinaryOp(bin_op, box (l, r)) => {
            let lty = l.ty(&fcx.body, fcx.tcx);
            let rty = r.ty(&fcx.body, fcx.tcx);

            assert_eq!(lty, rty);

            let mini::Type::Int(int_ty) = translate_ty(lty, fcx.tcx) else {
                panic!("arithmetic operation with non-int type unsupported!");
            };

            let l = translate_operand(l, fcx);
            let r = translate_operand(r, fcx);

            let l = specr::GcCow::new(l);
            let r = specr::GcCow::new(r);

            use rs::BinOp::*;
            let op = if *bin_op == Offset {
                mini::BinOp::PtrOffset {
                    inbounds: true // FIXME where to find this bool `inbouds` in mir?
                }
            } else { // everything else right-now is a int op!
                let op_int = match bin_op {
                    Add => mini::BinOpInt::Add,
                    Sub => mini::BinOpInt::Sub,
                    Mul => mini::BinOpInt::Mul,
                    Div => mini::BinOpInt::Div,
                    x => {
                        dbg!(x);
                        todo!("unsupported BinOp")
                    },
                };
                mini::BinOp::Int(op_int, int_ty)
            };

            mini::ValueExpr::BinOp {
                operator: op,
                left: l,
                right: r,
            }
        },
        rs::Rvalue::Ref(_, bkind, place) => {
            let ty = place.ty(&fcx.body, fcx.tcx).ty;
            let pointee = layout_of(ty, fcx.tcx);

            let place = translate_place(place, fcx);
            let target = specr::GcCow::new(place);
            let mutbl = translate_mutbl(bkind.to_mutbl_lossy());

            let ptr_ty = mini::PtrType::Ref { mutbl, pointee };

            mini::ValueExpr::AddrOf { target, ptr_ty }
        },
        rs::Rvalue::AddressOf(_mutbl, place) => {
            let ty = place.ty(&fcx.body, fcx.tcx).ty;
            let pointee = layout_of(ty, fcx.tcx);

            let place = translate_place(place, fcx);
            let target = specr::GcCow::new(place);

            let ptr_ty = mini::PtrType::Raw { pointee };

            mini::ValueExpr::AddrOf { target, ptr_ty }
        },
        rs::Rvalue::Aggregate(box rs::AggregateKind::Array(ty), operands) => {
            let count = specr::Int::from(operands.len());
            let ty = translate_ty(*ty, fcx.tcx);
            let ty = mini::Type::Array { elem: specr::GcCow::new(ty), count };
            let ops: specr::List<_> = operands.iter().map(|x| {
                let op = translate_operand(x, fcx);
                let mini::ValueExpr::Constant(c, _) = op else {
                    panic!("non-constants in array-expr not supported!");
                };

                c
            }).collect();
            let c = mini::Constant::Tuple(ops);
            mini::ValueExpr::Constant(c, ty)
        }
        x => {
            dbg!(x);
            todo!()
        }
    }
}

pub fn translate_operand<'tcx>(operand: &rs::Operand<'tcx>, fcx: &mut FnCtxt<'tcx>) -> mini::ValueExpr {
    match operand {
        rs::Operand::Constant(box c) => {
            match c.literal {
                rs::ConstantKind::Val(val, ty) => {
                    let ty = translate_ty(ty, fcx.tcx);
                    let constant = match ty {
                        mini::Type::Int(int_ty) => {
                            let val = val.try_to_scalar_int().unwrap();

                            use mini::Signedness::*;
                            let bits = specr::int_to_usize(int_ty.size.bits());
                            // TODO is there no better way to get the value from a ScalarInt?
                            let int: specr::Int = match (int_ty.signed, bits) {
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
                            mini::Constant::Int(int)
                        },
                        // unit type `()`
                        mini::Type::Tuple { fields, .. } if fields.is_empty() => {
                            mini::Constant::Tuple(specr::List::new())
                        }
                        x => {
                            dbg!(x);
                            todo!()
                        }
                    };
                    mini::ValueExpr::Constant(constant, ty)
                }
                x => {
                    dbg!(x);
                    todo!()
                }
            }
        },
        rs::Operand::Copy(place) => {
            mini::ValueExpr::Load {
                destructive: false,
                source: specr::GcCow::new(translate_place(place, fcx)),
            }
        },
        rs::Operand::Move(place) => {
            mini::ValueExpr::Load {
                destructive: true,
                source: specr::GcCow::new(translate_place(place, fcx)),
            }
        },
    }
}

fn place_type_of<'tcx>(ty: rs::Ty<'tcx>, fcx: &mut FnCtxt<'tcx>) -> mini::PlaceType {
    let align = layout_of(ty, fcx.tcx).align;
    let ty = translate_ty(ty, fcx.tcx);

    mini::PlaceType { ty, align }
}

pub fn translate_place<'tcx>(place: &rs::Place<'tcx>, fcx: &mut FnCtxt<'tcx>) -> mini::PlaceExpr {
    let mut expr = mini::PlaceExpr::Local(fcx.localname_map[&place.local]);
    for (i, proj) in place.projection.iter().enumerate() {
        match proj {
            rs::ProjectionElem::Field(f, _ty) => {
                let f = f.index();
                let indirected = specr::GcCow::new(expr);
                expr = mini::PlaceExpr::Field {
                    root: indirected,
                    field: f.into(),
                };
            },
            rs::ProjectionElem::Deref => {
                let x = specr::GcCow::new(expr);
                let x = mini::ValueExpr::Load {
                    destructive: false,
                    source: x
                };
                let x = specr::GcCow::new(x);

                let ty = rs::Place::ty_from(place.local, &place.projection[..(i+1)], &fcx.body, fcx.tcx).ty;
                let ptype = place_type_of(ty, fcx);

                expr = mini::PlaceExpr::Deref {
                    operand: x,
                    ptype,
                };
            },
            rs::ProjectionElem::Index(i) => {
                todo!("{:?}", i)
            },
            x => todo!("{:?}", x),
        }
    }
    expr
}

