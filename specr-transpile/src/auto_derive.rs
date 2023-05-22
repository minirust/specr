use std::mem;

use crate::prelude::*;

/// Traits all structs & enums should derive.
static GENERAL_TRAITS: &[&str] = &["GcCompat", "Clone", "Debug"];

/// Traits only "objects" should derive. They get used in maps, sets, etc.
static OBJ_TRAITS: &[&str] = &["Copy", "PartialEq", "Eq", "Hash"];

/// Adds `#[derive(_)]` for all missing traits in `GENERAL_TRAITS` and `OBJ_TRAITS`.
pub fn auto_derive(mut ast: syn::File) -> syn::File {
    for i in ast.items.iter_mut() {
        let attrs = match i {
            Item::Struct(s) => {
                &mut s.attrs
            },
            Item::Enum(e) => {
                &mut e.attrs
            },
            _ => { continue; },
        };

        for t in GENERAL_TRAITS {
            add_derive_attr(t, attrs);
        }

        // If attr `#[no_obj]` is present remove it and skip obj traits.
        if attrs.iter().any(is_no_obj) {
            remove_no_obj(attrs);
            continue;
        }
        
        for t in OBJ_TRAITS {
            add_derive_attr(t, attrs);
        }
    }

    ast
}

/// checks whether `attrs` contains some attribute `#[derive(..., t, ...)]`
fn contains_derive_attr(t: &str, attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let Meta::List(list) = &attr.meta else { return false };
        if format!("{}", list.path.to_token_stream()) != "derive" { return false; }
        let Some(TokenTree::Group(g)) = list.tokens.clone().into_iter().next() else { return false };

        g.stream().into_iter().any(|tk| format!("{}", tk) == t)
    })
}

/// checks if this attribute is `#[no_obj]`
fn is_no_obj(attr: &Attribute) -> bool {
    let Meta::Path(path) = &attr.meta else {
        return false 
    };

    format!("{}", path.to_token_stream()) == "no_obj"
}

/// removes `#[no_obj]`
fn remove_no_obj(attrs: &mut Vec<Attribute>) {
    let owned_attrs = mem::replace(attrs, vec![]);
    let owned_attrs = owned_attrs.into_iter().filter(|a| !is_no_obj(a)).collect::<Vec<_>>();
    _ = mem::replace(attrs, owned_attrs);
}

/// adds `#[derive(t)]` to `attrs`, if it's missing.
fn add_derive_attr(t: &str, attrs: &mut Vec<Attribute>) {
    if !contains_derive_attr(t, attrs) {
        attrs.push(derive_attr(t));
    }
}

/// generates `#[derive(t)]`
fn derive_attr(t: &str) -> Attribute {
    let id = format_ident!("{}", t);

    // Attributes can not be parsed in of itself,
    // but only as prefix to some Item.
    let code = quote! { #[derive(#id)] struct X; };
    let Item::Struct(item) = parse2(code).unwrap() else { unreachable!() };
    item.attrs[0].clone()
}
