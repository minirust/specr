use std::collections::HashSet;

mod inf_enums;

mod wrap;
use wrap::wrap_variant_elements;

mod fix;

/// Specifies an Element of some enum Variant.
/// Note that this does not know the name of the enum.
///
/// Example:
/// enum Foo {
///   A { a: u32 },
///   B(u32),
/// }
///
/// referencing Foo::A::a would be done by
/// VariantElement { variant: "A", idx: ElementIdx::Named("a") } 
/// while the u32-argument of Foo::B would be referenced by
/// VariantElement { variant: "B", idx: ElementIdx::Unnamed(0) } 
///
#[derive(Hash, PartialEq, Eq)]
struct VariantElement {
    variant: String,
    idx: ElementIdx,
}

/// Indexes an enum variant, either by-name, or by argument position.
#[derive(Hash, PartialEq, Eq)]
enum ElementIdx {
    Named(String),
    Unnamed(usize),
}

// TODO support generic enums.
pub fn typerec(mut mods: Vec<syn::File>) -> Vec<syn::File> {
    let elements = wrap_variant_elements(&mut mods);
    fix::fix(&mut mods, &elements);

    mods
}
