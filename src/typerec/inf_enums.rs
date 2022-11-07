use crate::typerec::*;

/// returns whether `attr` is `#[specr::rc]`
fn attr_is_rc(attr: &Attribute) -> bool {
    let segments: Vec<String> = attr.path.segments.iter()
                                    .map(|x| format!("{}", x.ident))
                                    .collect();
    let [s1, s2] = &segments[..] else { return false };

    s1 == "specr" && s2 == "rc"
}

/// Returns the enums being marked with `#[specr::rc]` without any generic parameters.
/// It also removes this attribute from the source code.
pub fn inf_size_enums(mods: &mut [syn::File]) -> HashSet<String> {
    let mut enums = HashSet::new();

    for m in mods {
        for item in &mut m.items {
            let Item::Enum(it_enum) = item else { continue };
            let Some(i) = it_enum.attrs.iter().position(attr_is_rc) else { continue };

            it_enum.attrs.remove(i);
            let name = format!("{}", it_enum.ident);
            enums.insert(name);
        }
    }

    enums
}
