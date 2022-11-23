use crate::*;

pub fn translate_bb(bb: &mir::BasicBlockData) -> mini::BasicBlock {
    mini::BasicBlock {
        statements: bb.statements.iter().map(|s| translate_stmt(s)).collect(),
        terminator: translate_terminator(&bb.terminator),
    }
}

fn translate_stmt(_stmt: &mir::Statement) -> mini::Statement {
    todo!()
}

// does `None` mean fallthrough? in other words, go from bb `i` to bb `i+1`?
fn translate_terminator(_stmt: &Option<mir::Terminator>) -> mini::Terminator {
    todo!()
}
