use crate::*;

use std::convert::Infallible;

/// `GcCompat` expresses that a type is compatible with the garbage collector.
/// It is required in order to contain `GcCow` and to be the generic param to `GcCow`.
pub trait GcCompat: 'static {
    /// Writes the gc'd objs, that `self` directly points to, into `buffer`.
    fn points_to(&self, buffer: &mut HashSet<usize>);

    /// converts self to `Any`.
    fn as_any(&self) -> &dyn Any;
}

// impls for GcCompat:

macro_rules! empty_gccompat {
    ( $( $t:ty ),* ) => {
        $(
            impl GcCompat for $t {
                fn points_to(&self, _m: &mut HashSet<usize>) {}
                fn as_any(&self) -> &dyn Any { self }
            }
        )*
    };
}

empty_gccompat!((), bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, std::string::String, Infallible, ExtInt);

impl<A, B> GcCompat for (A, B) where A: GcCompat, B: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        let (a, b) = self;
        a.points_to(m);
        b.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: GcCompat> GcCompat for Option<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        match self {
            Some(x) => x.points_to(m),
            None => {},
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: GcCompat, E: GcCompat> GcCompat for Result<T, E> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        match self {
            Ok(x) => x.points_to(m),
            Err(x) => x.points_to(m),
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

