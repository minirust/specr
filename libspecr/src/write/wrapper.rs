use std::{io::Write, rc::Rc, fmt::{Debug, self}, hash::Hash, cell::RefCell, ptr};

use crate::gc::GcCompat;

#[derive(Clone)]
/// A wrapper for `Write` that is garbage collected.
pub(crate) struct WriteWrapper(Rc<RefCell<dyn Write>>);

impl WriteWrapper {
    pub(crate) fn new(write: impl Write + 'static) -> Self {
        WriteWrapper(Rc::new(RefCell::new(write)))
    }

    /// Writes a formatted string into this writer, returning any error
    /// encountered.
    pub(crate) fn write_fmt(&self, fmt: fmt::Arguments<'_>) -> Result<(), std::io::Error> {
        let mut write_stream = (*self.0).borrow_mut();
        write_stream.write_fmt(fmt)
    }
}

impl GcCompat for WriteWrapper {
    fn points_to(&self, _buffer: &mut std::collections::HashSet<usize>) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Debug for WriteWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WriteWrapper").finish()
    }
}

impl Hash for WriteWrapper {
    fn hash<H: ~const std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self.0.as_ptr(), state);
    }
}

impl PartialEq for WriteWrapper {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for WriteWrapper {}
