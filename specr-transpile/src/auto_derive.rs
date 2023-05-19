use std::mem;

use crate::prelude::*;

static TRAITS: &[&str] = &["Clone", "Copy", "Debug", "PartialEq", "Eq", "Hash", "GcCompat"];

/// Adds `#[derive(_)]` for all missing traits in `TRAITS`.
pub fn auto_derive(mut ast: syn::File) -> syn::File {
    for t in TRAITS {
        for i in ast.items.iter_mut() {
            match i {
                Item::Struct(s) => {
                    add_derive_attr(t, &mut s.attrs);
                },
                Item::Enum(e) => {
                    add_derive_attr(t, &mut e.attrs);
                },
                _ => {},
            }
        }
    }

    for i in ast.items.iter_mut() {
        match i {
            Item::Struct(s) => {
                remove_no_auto_derive(&mut s.attrs);
            },
            Item::Enum(e) => {
                remove_no_auto_derive(&mut e.attrs);
            },
            _ => {},
        }
    }

    ast
}

// checks whether `attrs` contains some attribute `#[derive(..., t, ...)]`
fn contains_derive_attr(t: &str, attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let Meta::List(list) = &attr.meta else { return false };
        if format!("{}", list.path.to_token_stream()) != "derive" { return false; }
        let Some(TokenTree::Group(g)) = list.tokens.clone().into_iter().next() else { return false };

        g.stream().into_iter().any(|tk| format!("{}", tk) == t)
    })
}

// checks existence of `#[no_auto_derive]`
fn is_no_auto_derive(attr: &Attribute) -> bool {
    let Meta::Path(path) = &attr.meta else {
        return false 
    };

    format!("{}", path.to_token_stream()) == "no_auto_derive"
}

// removes `#[no_auto_derive]`
fn remove_no_auto_derive(attrs: &mut Vec<Attribute>) {
    let owned_attrs = mem::replace(attrs, vec![]);
    let owned_attrs = owned_attrs.into_iter().filter(|a| !is_no_auto_derive(a)).collect::<Vec<_>>();
    _ = mem::replace(attrs, owned_attrs);
}

// adds `#[derive(t)]` to `attrs`, if it's missing.
fn add_derive_attr(t: &str, attrs: &mut Vec<Attribute>) {
    if attrs.iter().any(is_no_auto_derive) { return }

    if !contains_derive_attr(t, attrs) {
        attrs.push(derive_attr(t));
    }
}

// generates `#[derive(t)]`
fn derive_attr(t: &str) -> Attribute {
    let id = format_ident!("{}", t);

    // Attributes can not be parsed in of itself,
    // but only as prefix to some Item.
    let code = quote! { #[derive(#id)] struct X; };
    let Item::Struct(item) = parse2(code).unwrap() else { unreachable!() };
    item.attrs[0].clone()
}
