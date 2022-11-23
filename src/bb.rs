use crate::*;

pub fn translate_bb(bb: &mir::BasicBlockData) -> mini::BasicBlock {
    mini::BasicBlock {
        statements: bb.statements.iter().map(translate_stmt).collect(),
        terminator: translate_terminator(bb.terminator()),
    }
}

fn translate_stmt(_stmt: &mir::Statement) -> mini::Statement {
    todo!()
}

fn translate_terminator(terminator: &mir::Terminator) -> mini::Terminator {
    match terminator.kind {
        mir::TerminatorKind::Return => mini::Terminator::Return,
        _ => todo!(),
    }
}
