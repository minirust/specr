use crate::*;

pub fn translate_bb<'tcx>(bb: &rs::BasicBlockData<'tcx>, fcx: FnCtxt<'_, 'tcx>) -> mini::BasicBlock {
    let mut statements = specr::List::new();
    for stmt in bb.statements.iter() {
        translate_stmt(stmt, fcx, &mut statements);
    }
    mini::BasicBlock {
        statements,
        terminator: translate_terminator(bb.terminator(), fcx),
    }
}

fn translate_stmt<'tcx>(stmt: &rs::Statement<'tcx>, fcx: FnCtxt<'_, 'tcx>, statements: &mut specr::List<mini::Statement>) {
    match &stmt.kind {
        rs::StatementKind::Assign(box (place, rval)) => {
            statements.push(
                mini::Statement::Assign {
                    destination: translate_place(place, fcx),
                    source: translate_rvalue(rval, fcx),
                }
            );
        },
        rs::StatementKind::StorageLive(local) => {
            statements.push(
                mini::Statement::StorageLive(fcx.localname_map[&local])
            );
        },
        rs::StatementKind::StorageDead(local) => {
            statements.push(
                mini::Statement::StorageDead(fcx.localname_map[&local])
            );
        },
        rs::StatementKind::Deinit(_) => { /* this has no mini::_ equivalent. */ },
        x => {
            dbg!(x);
            todo!()
        }
    }
}

fn translate_terminator<'tcx>(terminator: &rs::Terminator<'tcx>, fcx: FnCtxt<'_, 'tcx>) -> mini::Terminator {
    match &terminator.kind {
        rs::TerminatorKind::Return => mini::Terminator::Return,
        rs::TerminatorKind::Goto { target } => mini::Terminator::Goto(fcx.bbname_map[&target]),
        rs::TerminatorKind::Call { func, target, destination, args, .. } => {
            let rs::Operand::Constant(box f) = func else { panic!() };
            let rs::ConstantKind::Val(_, f) = f.literal else { panic!() };
            let rs::TyKind::FnDef(f, _) = f.kind() else { panic!() };
            mini::Terminator::Call {
                callee: fcx.fnname_map[&f],
                arguments: args.iter().map(|x| (translate_operand(x, fcx), arg_abi())).collect(),
                ret: (translate_place(&destination, fcx), arg_abi()),
                next_block: fcx.bbname_map[&target.unwrap()], // TODO handle `None`: it means that the call necessarily diverges, see the docs.
            }
        }
        x => {
            dbg!(x);
            todo!()
        }
    }
}

fn translate_place<'tcx>(place: &rs::Place<'tcx>, fcx: FnCtxt<'_, 'tcx>) -> mini::PlaceExpr {
    mini::PlaceExpr::Local(fcx.localname_map[&place.local])
    // TODO apply projections
}

fn translate_rvalue<'tcx>(place: &rs::Rvalue<'tcx>, fcx: FnCtxt<'_, 'tcx>) -> mini::ValueExpr {
    match place {
        rs::Rvalue::Use(operand) => translate_operand(operand, fcx),
        x => {
            dbg!(x);
            todo!()
        }
    }
}

fn translate_operand<'tcx>(operand: &rs::Operand<'tcx>, fcx: FnCtxt<'_, 'tcx>) -> mini::ValueExpr {
    match operand {
        rs::Operand::Constant(box c) => {
            match c.literal {
                rs::ConstantKind::Val(val, ty) => {
                    let ty = translate_ty(&ty, fcx.tcx);
                    let constant = match ty {
                        mini::Type::Int(int_ty) => {
                            let val = val.try_to_scalar_int().unwrap();

                            use mini::Signedness::*;
                            let bits = specr::hidden::int_to_usize(int_ty.size.bits());
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
                source: specr::hidden::GcCow::new(translate_place(place, fcx)),
            }
        },
        rs::Operand::Move(place) => {
            mini::ValueExpr::Load {
                destructive: true,
                source: specr::hidden::GcCow::new(translate_place(place, fcx)),
            }
        },
    }
}
