use crate::*;

pub fn translate_rvalue<'cx, 'tcx>(rv: &rs::Rvalue<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> Option<ValueExpr> {
    Some(match rv {
        rs::Rvalue::Use(operand) => translate_operand(operand, fcx),
        rs::Rvalue::CheckedBinaryOp(bin_op, box (l, r)) | rs::Rvalue::BinaryOp(bin_op, box (l, r)) => {
            let lty = l.ty(&fcx.body, fcx.cx.tcx);
            let rty = r.ty(&fcx.body, fcx.cx.tcx);

            assert_eq!(lty, rty);

            let l = translate_operand(l, fcx);
            let r = translate_operand(r, fcx);

            let l = GcCow::new(l);
            let r = GcCow::new(r);

            use rs::BinOp::*;
            let op = if *bin_op == Offset {
                BinOp::PtrOffset { inbounds: true }
            } else { // everything else right-now is a int op!

                let op = |x| {
                    let Type::Int(int_ty) = translate_ty(lty, fcx.cx.tcx) else {
                        panic!("arithmetic operation with non-int type unsupported!");
                    };

                    BinOp::Int(x, int_ty)
                };
                let rel = |x| BinOp::IntRel(x);

                match bin_op {
                    Add => op(BinOpInt::Add),
                    Sub => op(BinOpInt::Sub),
                    Mul => op(BinOpInt::Mul),
                    Div => op(BinOpInt::Div),
                    Rem => op(BinOpInt::Rem),

                    Lt => rel(IntRel::Lt),
                    Le => rel(IntRel::Le),
                    Gt => rel(IntRel::Gt),
                    Ge => rel(IntRel::Ge),
                    Eq => rel(IntRel::Eq),
                    Ne => rel(IntRel::Ne),

                    BitAnd => return None,
                    x => {
                        dbg!(x);
                        todo!("unsupported BinOp")
                    },
                }
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
                    let ty = operand.ty(&fcx.body, fcx.cx.tcx);
                    let ty = translate_ty(ty, fcx.cx.tcx);
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
            let ty = place.ty(&fcx.body, fcx.cx.tcx).ty;
            let pointee = layout_of(ty, fcx.cx.tcx);

            let place = translate_place(place, fcx);
            let target = GcCow::new(place);
            let mutbl = translate_mutbl(bkind.to_mutbl_lossy());

            let ptr_ty = PtrType::Ref { mutbl, pointee };

            ValueExpr::AddrOf { target, ptr_ty }
        },
        rs::Rvalue::AddressOf(_mutbl, place) => {
            let ty = place.ty(&fcx.body, fcx.cx.tcx).ty;
            let pointee = layout_of(ty, fcx.cx.tcx);

            let place = translate_place(place, fcx);
            let target = GcCow::new(place);

            let ptr_ty = PtrType::Raw { pointee };

            ValueExpr::AddrOf { target, ptr_ty }
        },
        rs::Rvalue::Aggregate(box agg, operands) => {
            let ty = rv.ty(&fcx.body, fcx.cx.tcx);
            let ty = translate_ty(ty, fcx.cx.tcx);
            match ty {
                Type::Union { .. } => {
                    let rs::AggregateKind::Adt(_, _, _, _, Some(field_idx)) = agg else { panic!() };
                    assert_eq!(operands.len(), 1);
                    let expr = translate_operand(&operands[0], fcx);
                    ValueExpr::Union {
                        field: (*field_idx).into(),
                        expr: GcCow::new(expr),
                        union_ty: ty,
                    }
                },
                Type::Tuple { .. } | Type::Array { .. } => {
                    let ops: List<_> = operands.iter().map(|x| translate_operand(x, fcx)).collect();
                    ValueExpr::Tuple(ops, ty)
                },
                Type::Enum { .. } => todo!(),
                _ => panic!("invalid aggregate type!"),
            }
        },
        rs::Rvalue::CopyForDeref(place) => {
            ValueExpr::Load {
                destructive: false,
                source: GcCow::new(translate_place(place, fcx)),
            }
        },
        rs::Rvalue::Len(place) => {
            // as slices are unsupported as of now, we only need to care for arrays.
            let ty = place.ty(&fcx.body, fcx.cx.tcx).ty;
            let Type::Array { elem: _, count } = translate_ty(ty, fcx.cx.tcx) else { panic!() };
            use crate::minisyntax::build::TypeConv;
            ValueExpr::Constant(Constant::Int(count), <usize>::get_type())
        }
        rs::Rvalue::Cast(rs::CastKind::IntToInt, operand, ty) => {
            let operand = translate_operand(operand, fcx);
            let Type::Int(int_ty) = translate_ty(*ty, fcx.cx.tcx) else {
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
            let Type::Ptr(ptr_ty) = translate_ty(*ty, fcx.cx.tcx) else { panic!() };

            ValueExpr::UnOp {
                operator: UnOp::Int2Ptr(ptr_ty),
                operand: GcCow::new(operand),
            }
        },
        rs::Rvalue::Cast(rs::CastKind::PtrToPtr, operand, ty) => {
            let operand = translate_operand(operand, fcx);
            let Type::Ptr(ptr_ty) = translate_ty(*ty, fcx.cx.tcx) else { panic!() };

            ValueExpr::UnOp {
                operator: UnOp::Ptr2Ptr(ptr_ty),
                operand: GcCow::new(operand),
            }
        },
        rs::Rvalue::Repeat(op, c) => {
            let c = c.try_eval_usize(fcx.cx.tcx, rs::ParamEnv::empty()).unwrap();
            let c = Int::from(c);

            let elem_ty = translate_ty(op.ty(&fcx.body, fcx.cx.tcx), fcx.cx.tcx);
            let op = translate_operand(op, fcx);

            let ty = Type::Array {
                elem: GcCow::new(elem_ty),
                count: c,
            };

            let ls = list![op; c];
            ValueExpr::Tuple(ls, ty)
        }
        x => {
            dbg!(x);
            todo!()
        }
    })
}

fn translate_const_allocation<'cx, 'tcx>(allocation: rs::ConstAllocation<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> GlobalName {
    let allocation = allocation.inner();
    let size = allocation.size();
    let alloc_range = rs::AllocRange { start: rs::Size::ZERO, size };
    let bytes = allocation.get_bytes_strip_provenance(&fcx.cx.tcx, alloc_range).unwrap().iter().copied().map(Some).collect();
    let align = translate_align(allocation.align);
    let global = Global {
        bytes,
        relocations: List::new(), // TODO
        align,
    };

    let name = GlobalName(Name::new(fcx.cx.globals.iter().count() as _)); // TODO use .len() here, if supported
    fcx.cx.globals.insert(name, global);

    name
}

pub fn translate_const<'cx, 'tcx>(c: &rs::Constant<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> ValueExpr {
    let kind = c.literal.eval(fcx.cx.tcx, rs::ParamEnv::empty());
    let rs::ConstantKind::Val(val, ty) = kind else { panic!("unsupported ConstantKind!") };

    let pty = place_type_of(ty, fcx);
    let ty = pty.ty;

    // This handles usages of `const`
    if let rs::ConstValue::ByRef { alloc, offset } = val {
        let name = translate_const_allocation(alloc, fcx);
        let offset = translate_size(offset);
        let rel = Relocation { name, offset };
        let expr = Constant::Pointer(rel);

        let ptr_ty = Type::Ptr(PtrType::Raw { pointee: pty.layout::<BasicMemory>() });

        let expr = ValueExpr::Constant(expr, ptr_ty);
        let expr = PlaceExpr::Deref {
            operand: GcCow::new(expr),
            ptype: pty,
        };
        let expr = ValueExpr::Load {
            source: GcCow::new(expr),
            destructive: false
        };

        return expr;
    }

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
        // A `static`
        Type::Ptr(_) => {
            let (alloc_id, offset) = val.try_to_scalar()
                         .unwrap()
                         .to_pointer(&fcx.cx.tcx)
                         .unwrap()
                         .into_parts();
            let alloc_id = alloc_id.expect("no alloc id?");
            let rs::GlobalAlloc::Static(def_id) = fcx.cx.tcx.global_alloc(alloc_id) else { panic!() };

            let name = fcx.cx.static_map.get(&def_id).copied().unwrap_or_else(|| {
                let allocation = fcx.cx.tcx.eval_static_initializer(def_id).unwrap();
                let name = translate_const_allocation(allocation, fcx);
                fcx.cx.static_map.insert(def_id, name);
                name
            });

            let offset = translate_size(offset);
            let rel = Relocation { name, offset };
            Constant::Pointer(rel)
        },
        x => {
            dbg!(x);
            todo!()
        }
    };
    ValueExpr::Constant(constant, ty)
}

pub fn translate_operand<'cx, 'tcx>(operand: &rs::Operand<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> ValueExpr {
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

fn place_type_of<'cx, 'tcx>(ty: rs::Ty<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> PlaceType {
    let align = layout_of(ty, fcx.cx.tcx).align;
    let ty = translate_ty(ty, fcx.cx.tcx);

    PlaceType { ty, align }
}

pub fn translate_place<'cx, 'tcx>(place: &rs::Place<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> PlaceExpr {
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

                let ty = rs::Place::ty_from(place.local, &place.projection[..(i+1)], &fcx.body, fcx.cx.tcx).ty;
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

