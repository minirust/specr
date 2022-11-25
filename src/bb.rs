use crate::*;

pub fn translate_bb(bb: &mir::BasicBlockData, fcx: FnCtxt) -> mini::BasicBlock {
    mini::BasicBlock {
        statements: bb.statements.iter().map(|x| translate_stmt(x, fcx)).collect(),
        terminator: translate_terminator(bb.terminator(), fcx),
    }
}

fn translate_stmt(stmt: &mir::Statement, fcx: FnCtxt) -> mini::Statement {
    match &stmt.kind {
        mir::StatementKind::Assign(box (place, rval)) => {
            mini::Statement::Assign {
                destination: translate_place(place, fcx),
                source: translate_rvalue(rval),
            }
        },
        _ => todo!(),
    }
}

fn translate_terminator(terminator: &mir::Terminator, fcx: FnCtxt) -> mini::Terminator {
    match terminator.kind {
        mir::TerminatorKind::Return => mini::Terminator::Return,
        mir::TerminatorKind::Goto { target } => mini::Terminator::Goto(fcx.bbname_map[&target]),
        _ => todo!(),
    }
}

fn translate_place(place: &mir::Place, fcx: FnCtxt) -> mini::PlaceExpr {
    mini::PlaceExpr::Local(fcx.localname_map[&place.local])
    // TODO apply projections
}

fn translate_rvalue(place: &mir::Rvalue) -> mini::ValueExpr {
    match place {
        mir::Rvalue::Use(mir::Operand::Constant(box c)) => {
            match c.literal {
                mir::ConstantKind::Val(mir::ConstValue::Scalar(mir::Scalar::Int(x)), _) => {
                    let x = x.try_to_i32().unwrap();
                    let c = mini::Constant::Int(x.into());
                    let ty = mini::IntType {
                        signed: mini::Signedness::Signed,
                        size: mini::Size::from_bytes(4),
                    };
                    let ty = mini::Type::Int(ty);

                    mini::ValueExpr::Constant(c, ty)
                },
                _ => todo!(),
            }
        },
        _ => todo!(),
    }
}
