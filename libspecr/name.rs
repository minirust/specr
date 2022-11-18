use crate::libspecr::*;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Name(pub(in crate::libspecr) String);

impl GcCompat for Name {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}
