use crate::libspecr::*;

#[derive(Copy, Clone)]
pub enum Endianness {
    LittleEndian,
    BigEndian
}
pub use Endianness::*;
