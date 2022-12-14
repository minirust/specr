use crate::{*, mini::*, specr::*};

mod env;
use env::*;

#[test]
fn too_large_alloc() {
    fn program_alloc(bytes: Int) -> Program {
        let count = bytes;
        let array = Type::Array { elem: GcCow::new(Type::Bool), count }; 

        let l0 = LocalName(Name(0));
        let stmts = vec![
            Statement::StorageLive(l0),
            Statement::StorageDead(l0),
        ];
        let locals = vec![
            PlaceType {
                ty: array,
                align: Align::from_bytes(64),
            }
        ];
        program_from_statements(stmts, locals)
    }

    let large = Int::from(2).pow(BasicMemory::PTR_SIZE.bits());
    assert_unwell(program_alloc(large));

    let small = Int::from(2);
    assert_stop(program_alloc(small));
}

#[test]
fn double_live() {
    let l0 = LocalName(Name(0));
    let stmts = vec![
        Statement::StorageLive(l0),
        Statement::StorageLive(l0),
    ];
    let locals = vec![
        PlaceType {
            ty: Type::Bool,
            align: Align::from_bytes(1),
        }
    ];
    let p = program_from_statements(stmts, locals);
    assert_unwell(p);
}

#[test]
fn dead_before_live() {
    let l0 = LocalName(Name(0));
    let stmts = vec![
        Statement::StorageDead(l0),
    ];
    let locals = vec![
        PlaceType {
            ty: Type::Bool,
            align: Align::from_bytes(1),
        }
    ];
    let p = program_from_statements(stmts, locals);
    assert_unwell(p);
}
