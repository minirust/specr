use crate::test::*;

// fns[0] is the start function.
// fns[i] has name FnName(Name(i))
pub fn program(fns: &[Function]) -> Program {
    let mut functions = Map::new();
    for (i, f) in fns.iter().enumerate() {
        functions.insert(FnName(Name(i as _)), *f);
    }
    Program {
        functions,
        start: FnName(Name(0)),
    }
}

// whether a function returns or not.
pub enum Ret { Yes, No }

// if ret == Yes, then _0 is the return local.
// the first block is the starting block.
// locals[i] has name LocalName(Name(i))
// blocks[i] has name BbName(Name(i))
pub fn function(ret: Ret, num_args: usize, locs: &[PlaceType], bbs: &[BasicBlock]) -> Function {
    let mut locals = Map::new();
    for (i, l) in locs.iter().enumerate() {
        locals.insert(LocalName(Name(i as _)), *l);
    }

    let args = (0..num_args).map(|x| {
        let idx = match ret {
            Ret::Yes => x+1,
            Ret::No => x,
        };

        (LocalName(Name(idx as _)), ArgAbi::Register)
    }).collect();

    let ret = match ret {
        Ret::Yes => Some((LocalName(Name(0)), ArgAbi::Register)),
        Ret::No => None,
    };

    let mut blocks = Map::new();
    for (i, b) in bbs.iter().enumerate() {
        blocks.insert(BbName(Name(i as _)), *b);
    }

    let start = BbName(Name(0));

    Function {
        locals,
        args,
        ret,
        blocks,
        start,
    }
}

pub fn block(statements: &[Statement], terminator: Terminator) -> BasicBlock {
    BasicBlock {
        statements: statements.iter().copied().collect(),
        terminator,
    }
}
