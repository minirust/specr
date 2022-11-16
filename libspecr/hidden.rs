use crate::libspecr::*;

// `hidden` contents.
pub use crate::libspecr::ret::*;
pub use crate::libspecr::gccow::{GcCow, GcCompat};

// TODO this function panics in some cases. I should handle those cases.
pub fn bigint_to_usize(b: BigInt) -> usize {
    let (sign, digits) = b.0.call_ref(|b| b.to_u64_digits());
    if sign == num_bigint::Sign::Minus {
        panic!("cannot convert negative number to usize");
    }
    if digits.len() > 1 {
        panic!("number too large to fit into usize!");
    }

    *digits.get(0).unwrap_or(&0) as usize
}

pub fn list_from_elem<T: GcCompat + Clone>(elem: T, n: BigInt) -> List<T> {
    let n = bigint_to_usize(n);
    let v: im::vector::Vector<T> = std::iter::repeat(elem).take(n).collect();

    List(GcCow::new(v))
}
