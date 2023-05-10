use std::{io::{Write, Error}, fmt};

use crate::gc::GcCow;

use super::{DynWrite, wrapper::WriteWrapper};

impl DynWrite {
    /// Creates a new DynWriter.
    pub fn new(write: impl Write + 'static) -> Self {
        Self( GcCow::new( WriteWrapper::new( Box::new(write) ) ) )
    }

    /// Writes a formatted string into this writer, returning any error
    /// encountered.
    pub fn write_fmt(&self, fmt: fmt::Arguments<'_>) -> Result<(), Error> {
        self.0.call_ref_unchecked(|wrapper| wrapper.write_fmt(fmt))
    }
}
