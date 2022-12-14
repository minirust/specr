use std::ops::*;

/// Extension trait to implement `try_map`.
pub trait OptionExt<T> {
    fn try_map<O, E>(self, f: impl FnOnce(T) -> E) -> <<E as Try>::Residual as Residual<Option<O>>>::TryType
        where E: Try<Output=O>,
              <E as Try>::Residual: Residual<Option<O>>;
}

impl<T> OptionExt<T> for Option<T> {
    fn try_map<O, E>(self, f: impl FnOnce(T) -> E) -> <<E as Try>::Residual as Residual<Option<O>>>::TryType
        where E: Try<Output=O>,
              <E as Try>::Residual: Residual<Option<O>>
    {
        <<<E as Try>::Residual as Residual<Option<O>>>::TryType>::from_output(match self {
            Some(x) => Some(f(x)?),
            None => None,
        })
    }
}

#[test]
fn option_ext_test() {
    use crate::*;

    run_sequential(|| {
        use crate::*;

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
    });
}
