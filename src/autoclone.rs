use syn::*;

use quote::ToTokens;
use proc_macro2::TokenTree;

pub fn autoclone(mut ast: syn::File) -> syn::File {
    for i in ast.items.iter_mut() {
        match i {
            Item::Struct(s) => {
                add_clone(&mut s.attrs);
            },
            Item::Enum(e) => {
                add_clone(&mut e.attrs);
            },
            _ => {},
        }
    }

    ast  
}

fn contains_clone(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if format!("{}", attr.path.to_token_stream()) != "derive" { continue; }
        let Some(TokenTree::Group(g)) = attr.tokens.clone().into_iter().next() else { continue };
        if g.stream().into_iter().any(|t| format!("{}", t) == "Clone") { return true; }
    }

    false
}

fn add_clone(attrs: &mut Vec<Attribute>) {
    if !contains_clone(attrs) {
        attrs.push(derive_clone());
    }
}

fn derive_clone() -> Attribute {
    // TODO create #[derive(Clone)] directly
    let Item::Struct(item) = parse_str::<Item>("#[derive(Clone)] struct X;").unwrap() else { unreachable!() };
    item.attrs[0].clone()
}
