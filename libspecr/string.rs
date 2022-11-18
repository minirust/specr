use crate::libspecr::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct String(pub(in crate::libspecr) GcCow<std::string::String>);

impl GcCompat for String {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for std::string::String {
    fn points_to(&self, m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}
