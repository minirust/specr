use std::fmt;
use std::cell::RefCell;
use std::io::{Write, Stdout, Stderr, Error};
use std::fmt::Debug;

use crate::*;

/// An object that fulfills both GcCompat and Write.
pub trait GcWrite: GcCompat + Write {}
impl<T> GcWrite for T where T: GcCompat + Write {}

/// Garbage-collected data structure representing a write stream and implementing `Copy`.
#[derive(Copy, Clone, GcCompat)]
pub struct DynWrite(GcCow<RefCell<Box<dyn GcWrite>>>);

impl DynWrite {
    /// Creates a new DynWriter.
    pub fn new(write: impl GcWrite + 'static) -> Self {
        Self( GcCow::new( RefCell::new( Box::new( write ) ) ) )
    }

    /// Writes a formatted string into this writer, returning any error
    /// encountered.
    pub fn write_fmt(&self, fmt: fmt::Arguments<'_>) -> Result<(), Error> {
        self.0.call_ref_unchecked(
            |refcell| refcell.borrow_mut().write_fmt(fmt)
        )
    }
}

impl Debug for DynWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DynWrite").finish()
    }
}


// GcCompat implementation that are only relevant to `DynWrite`.
impl GcCompat for Stdout {
    fn points_to(&self, _buffer: &mut std::collections::HashSet<usize>) {}
}

impl GcCompat for Stderr {
    fn points_to(&self, _buffer: &mut std::collections::HashSet<usize>) {}
}
