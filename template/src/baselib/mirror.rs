use crate::baselib::*;

// This module mirrors the `prelude.md` file that is "rust,ignore"d.

pub type Result<T=()> = std::result::Result<T, TerminationInfo>;

#[non_exhaustive]
pub enum TerminationInfo {
  Ub(String),
  MachineStop(String),
}

#[macro_export]
macro_rules! throw {
    ($($tt:tt)*) => {{
        None?;
        unreachable!();
    }};
}

#[macro_export]
macro_rules! throw_ub {
    ($($tt:tt)*) => {{
        Err(TerminationInfo::Ub(format!($($tt)*)))?;
        unreachable!();
    }};
}

#[macro_export]
macro_rules! throw_machine_stop {
    ($($tt:tt)*) => {{
        Err(TerminationInfo::MachineStop(format!($($tt)*)))?;
        unreachable!();
    }};
}

pub struct Nondet<T=()>(pub T);
pub type NdResult<T=()> = Nondet<Result<T>>;

pub struct Size;
pub struct Align;

pub enum Signedness {
    Unsigned,
    Signed,
}
pub use Signedness::*;

pub enum Mutability {
    Mutable,
    Immutable,
}
pub use Mutability::*;

pub enum Endianness {
    LittleEndian,
    BigEndian,
}
pub use Endianness::*;

impl Endianness {
    fn decode(self, signed: Signedness, bytes: List<u8>) -> BigInt { todo!() }
    fn encode(self, signed: Signedness, size: Size, int: BigInt) -> Option<List<u8>> { todo!() }
}
