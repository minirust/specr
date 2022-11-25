use crate::libspecr::*;

// `hidden` contents.
pub use crate::libspecr::ret::*;
pub use crate::libspecr::gccow::{GcCow, GcCompat};
pub use crate::libspecr::obj::Obj;

// TODO this function panics in some cases. I should handle those cases.
pub fn int_to_usize(b: Int) -> usize {
    let (sign, digits) = b.ext().to_u64_digits();
    if sign == num_bigint::Sign::Minus {
        panic!("cannot convert negative number to usize");
    }
    if digits.len() > 1 {
        panic!("number too large to fit into usize!");
    }

    *digits.get(0).unwrap_or(&0) as usize
}

pub fn list_from_elem<T: Obj>(elem: T, n: Int) -> List<T> {
    let n = int_to_usize(n);
    let v: im::vector::Vector<T> = std::iter::repeat(elem).take(n).collect();

    List(GcCow::new(v))
}

pub fn mk_string(s: std::string::String) -> String {
    String(GcCow::new(s))
}
