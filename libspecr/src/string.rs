use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct String(pub GcCow<std::string::String>);

impl GcCompat for String {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for std::string::String {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}


pub fn mk_string(s: std::string::String) -> String {
    String(GcCow::new(s))
}

pub macro format {
    ($($thing:expr),*) => {
        mk_string(
            std::format!(
                $($thing),*
            )
        )
    },
}
