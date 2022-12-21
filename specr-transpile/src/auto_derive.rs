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

    ast
}

// checks whether `attrs` contains some attribute `#[derive(..., t, ...)]`
fn contains_derive_attr(t: &str, attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if format!("{}", attr.path.to_token_stream()) != "derive" { return false; }
        let Some(TokenTree::Group(g)) = attr.tokens.clone().into_iter().next() else { return false };

        g.stream().into_iter().any(|tk| format!("{}", tk) == t)
    })
}

// adds `#[derive(t)]` to `attrs`, if it's missing.
fn add_derive_attr(t: &str, attrs: &mut Vec<Attribute>) {
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
