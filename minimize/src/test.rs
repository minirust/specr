use crate::{*, mini::*, specr::*};

fn assert_ub(prog: Program, msg: &str) {
    assert_eq!(run_program(prog), Outcome::Ub(msg.to_string()));
}

fn assert_stop(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Stop);
}

fn assert_unwell(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Unwell);
}

fn function_from_statements(stmts: Vec<Statement>, local_types: Vec<PlaceType>) -> Function {
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

fn program_from_statements(stmts: Vec<Statement>, locals: Vec<PlaceType>) -> Program {
    let f = function_from_statements(stmts, locals);

    let mut functions = Map::new();
    let start = FnName(Name(0));
    functions.insert(start, f);
    Program {
        functions,
        start,
    }
}

#[test]
fn too_large_alloc() {
    let count = Int::from(2).pow(BasicMemory::PTR_SIZE.bits());
    let big_bad_array = Type::Array { elem: GcCow::new(Type::Bool), count }; 

    let l0 = LocalName(Name(0));
    let stmts = vec![
        Statement::StorageLive(l0),
        Statement::StorageDead(l0),
    ];
    let locals = vec![
        PlaceType {
            ty: big_bad_array,
            align: Align::from_bytes(64),
        }
    ];
    let p = program_from_statements(stmts, locals);

    assert_unwell(p);
}
