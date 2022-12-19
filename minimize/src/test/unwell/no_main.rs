use crate::test::*;

#[test]
fn no_main() {
    let p = program(&[]);
    assert_unwell(p);
}
