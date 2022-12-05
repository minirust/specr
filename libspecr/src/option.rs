use crate::*;

/// Extension trait to implement `try_map`.
pub trait OptionExt<T> {
    // Ideally, `try_map` would generalize over the error monad.
    // It should not only work with `NdResult`.

    // I (memoryleak47) have a hunch that this is not possible generally though.
    // except if you take the return type as generic parameter, which causes inference problems.
    fn try_map<O, E>(self, f: impl FnOnce(T) -> NdResult<O, E>) -> NdResult<Option<O>, E>;
}

impl<T> OptionExt<T> for Option<T> {
    fn try_map<O, E>(self, f: impl FnOnce(T) -> NdResult<O, E>) -> NdResult<Option<O>, E> {
        NdResult(Ok(match self {
            Some(x) => Some(f(x)?),
            None => None,
        }))
    }
}

#[test]
fn option_ext_test() {
    struct In;
    struct Out;
    struct Error;

    let ok: fn(In) -> NdResult<Out, Error> = |_: In| NdResult(Ok(Out));
    let err: fn(In) -> NdResult<Out, Error> = |_: In| NdResult(Err(Error));

    assert!(matches!(
        Some(In).try_map(ok),
        NdResult(Ok(Some(Out)))
    ));
    assert!(matches!(
        None.try_map(ok),
        NdResult(Ok(None))
    ));
    assert!(matches!(
        Some(In).try_map(err),
        NdResult(Err(Error))
    ));
    assert!(matches!(
        None.try_map(err),
        NdResult(Ok(None))
    ));
}
