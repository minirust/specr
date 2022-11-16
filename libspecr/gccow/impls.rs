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

impl<A, B> GcCompat for (A, B) where A: GcCompat, B: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        let (a, b) = self;
        a.points_to(m);
        b.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}
