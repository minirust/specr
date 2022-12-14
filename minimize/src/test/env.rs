use crate::test::*;

pub fn assert_ub(prog: Program, msg: &str) {
    assert_eq!(run_program(prog), Outcome::Ub(msg.to_string()));
}

pub fn assert_stop(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Stop);
}

pub fn assert_unwell(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Unwell);
}

pub fn function_from_statements(stmts: Vec<Statement>, local_types: Vec<PlaceType>) -> Function {
    let bb = BasicBlock {
        statements: stmts.into_iter().collect(),
        terminator: Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Exit,
            arguments: List::new(),
            ret: None,
            next_block: None,
        },
    };
    
    let mut locals = Map::new();
    for (i, l) in local_types.into_iter().enumerate() {
        let lname = LocalName(Name(i as _));
        locals.insert(lname, l);
    }

    let bbname = BbName(Name(0));
    let mut blocks = Map::new();
    blocks.insert(bbname, bb);
    
    Function {
        locals,
        args: List::new(),
        ret: None,
        blocks,
        start: bbname,
    }
}

pub fn program_from_statements(stmts: Vec<Statement>, locals: Vec<PlaceType>) -> Program {
    let f = function_from_statements(stmts, locals);

    let mut functions = Map::new();
    let start = FnName(Name(0));
    functions.insert(start, f);
    Program {
        functions,
        start,
    }
}


