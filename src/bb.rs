use crate::*;

pub fn translate_bb(bb: &mir::BasicBlockData, localname_map: &HashMap<mir::Local, mini::LocalName>) -> mini::BasicBlock {
    mini::BasicBlock {
        statements: bb.statements.iter().map(|x| translate_stmt(x, localname_map)).collect(),
        terminator: translate_terminator(bb.terminator()),
    }
}

fn translate_stmt(stmt: &mir::Statement, localname_map: &HashMap<mir::Local, mini::LocalName>) -> mini::Statement {
    match &stmt.kind {
        mir::StatementKind::Assign(box (place, rval)) => {
            mini::Statement::Assign {
                destination: translate_place(place, localname_map),
                source: translate_rvalue(rval),
            }
        },
        _ => todo!(),
    }
}

fn translate_terminator(terminator: &mir::Terminator) -> mini::Terminator {
    match terminator.kind {
        mir::TerminatorKind::Return => mini::Terminator::Return,
        _ => todo!(),
    }
}

fn translate_place(place: &mir::Place, localname_map: &HashMap<mir::Local, mini::LocalName>) -> mini::PlaceExpr {
    mini::PlaceExpr::Local(localname_map[&place.local])
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
