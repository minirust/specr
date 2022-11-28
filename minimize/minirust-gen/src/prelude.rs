use crate::specr;
use crate::specr::prelude::*;
use crate::{lang, mem, prelude};
#[doc = " `raw` stores the size in bytes."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Size {
    raw: BigInt,
}
impl specr::hidden::GcCompat for Size {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.raw.points_to(s);
    }
}
impl Size {
    pub fn zero() -> Size {
        specr::hidden::ret(Size {
            raw: BigInt::from(0),
        })
    }
    #[doc = " Rounds `bits` up to the next-higher byte boundary, if `bits` is"]
    #[doc = " not a multiple of 8."]
    pub fn from_bits(bits: impl Into<BigInt>) -> Size {
        let bits = bits.into();
        let raw = bits / 8 + ((bits % 8) + 7) / 8;
        specr::hidden::ret(Size { raw })
    }
    pub fn from_bytes(bytes: impl Into<BigInt>) -> Size {
        let bytes = bytes.into();
        specr::hidden::ret(Size { raw: bytes })
    }
    pub fn bytes(self) -> BigInt {
        specr::hidden::ret(self.raw)
    }
    pub fn bits(self) -> BigInt {
        specr::hidden::ret(self.raw * 8)
    }
}
#[doc = " All operations are fallible, so they return `Result`.  If they fail, that"]
#[doc = " means the program caused UB or put the machine to a halt."]
pub type Result<T = ()> = std::result::Result<T, TerminationInfo>;
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerminationInfo {
    Ub(String),
    MachineStop(String),
}
impl specr::hidden::GcCompat for TerminationInfo {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Ub(a0) => {
                a0.points_to(s);
            }
            Self::MachineStop(a0) => {
                a0.points_to(s);
            }
        }
    }
}
#[doc = " Some macros for convenient yeeting, i.e., return an error from a"]
#[doc = " `Option`/`Result`-returning function."]
macro_rules! throw {
    ($ ($ tt : tt) *) => {
        specr::yeet!(())
    };
}
macro_rules ! throw_ub { ($ ($ tt : tt) *) => { specr :: yeet ! (TerminationInfo :: Ub (format ! ($ ($ tt) *))) } ; }
macro_rules ! throw_machine_stop { ($ ($ tt : tt) *) => { specr :: yeet ! (TerminationInfo :: MachineStop (format ! ($ ($ tt) *))) } ; }
#[doc = " We leave the encoding of the non-determinism monad opaque."]
pub use specr::Nondet;
pub type NdResult<T = ()> = Nondet<Result<T>>;
#[doc = " Whether an integer value is signed or unsigned."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Signedness {
    Unsigned,
    Signed,
}
impl specr::hidden::GcCompat for Signedness {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Unsigned => {}
            Self::Signed => {}
        }
    }
}
pub use Signedness::*;
#[doc = " Whether a pointer/reference/allocation is mutable or immutable."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mutability {
    Mutable,
    Immutable,
}
impl specr::hidden::GcCompat for Mutability {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Mutable => {}
            Self::Immutable => {}
        }
    }
}
pub use Mutability::*;
#[doc = " `raw` stores the align in bytes."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Align {
    raw: BigInt,
}
impl specr::hidden::GcCompat for Align {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.raw.points_to(s);
    }
}
impl Align {
    pub fn one() -> Align {
        specr::hidden::ret(Align {
            raw: BigInt::from(1),
        })
    }
    #[doc = " align is rounded up to the next power of two."]
    pub fn from_bytes(align: impl Into<BigInt>) -> Align {
        let align = align.into();
        let raw = align.next_power_of_two();
        specr::hidden::ret(Align { raw })
    }
    pub fn bytes(self) -> BigInt {
        specr::hidden::ret(self.raw)
    }
}
pub use specr::BigInt;
impl BigInt {
    #[doc = " Returns the unique value that is equal to `self` modulo `2^size.bits()`."]
    #[doc = " If `signed == Unsigned`, the result is in the interval `0..2^size.bits()`,"]
    #[doc = " else it is in the interval `-2^(size.bits()-1) .. 2^(size.bits()-1)`."]
    #[doc = ""]
    #[doc = " `size` must not be zero."]
    pub fn modulo(self, signed: Signedness, size: Size) -> BigInt {
        if size.is_zero() {
            panic!("BigInt::modulo received invalid size zero!");
        }
        let m = BigInt::from(2).pow(size.bits());
        let n = self % m;
        specr::hidden::ret(match signed {
            Unsigned if n < 0 => n + m,
            Signed if n >= m / 2 => n - m,
            Signed if n < -m / 2 => n + m,
            _ => n,
        })
    }
    #[doc = " Tests whether an integer is in-bounds of a finite integer type."]
    pub fn in_bounds(self, signed: Signedness, size: Size) -> bool {
        specr::hidden::ret(self == self.modulo(signed, size))
    }
}
