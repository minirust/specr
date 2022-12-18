use crate::test::*;

#[test]
fn no_main() {
    run_sequential(|| {
        let p = program(&[]);
        assert_unwell(p);
    });
}
