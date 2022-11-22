use crate::libspecr::*;

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

empty_gccompat!((), bool, u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl<A, B> GcCompat for (A, B)
where
    A: GcCompat + 'static,
    B: GcCompat + 'static,
{
    fn points_to(&self, m: &mut HashSet<usize>) {
        let (a, b) = self;
        a.points_to(m);
        b.points_to(m);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: GcCompat + 'static> GcCompat for Option<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        match self {
            Some(x) => x.points_to(m),
            None => {}
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
