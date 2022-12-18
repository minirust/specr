use crate::*;

use num_traits::ToPrimitive;

mod obj;
pub use obj::*;

// The GcCow primites are only accessible through the `hidden` module.
pub use crate::gccow::{GcCow, GcCompat, mark_and_sweep, run_sequential};

impl Int {
    #[doc(hidden)]
    pub fn try_to_usize(self) -> Option<usize> {
        self.ext().to_usize()
    }

    #[doc(hidden)]
    pub fn try_to_u8(self) -> Option<u8> {
        self.ext().to_u8()
    }
}

impl<T: Obj> List<T> {
    #[doc(hidden)]
    pub fn from_elem(elem: T, n: Int) -> List<T> {
        let n = n.try_to_usize().expect("invalid number of elements in List::from_elem");
        let v: im::vector::Vector<T> = std::iter::repeat(elem).take(n).collect();

        List(GcCow::new(v))
    }
}
