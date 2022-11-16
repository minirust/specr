use crate::prelude::*;

pub fn autocopy(mut ast: syn::File) -> syn::File {
    for i in ast.items.iter_mut() {
        match i {
            Item::Struct(s) => {
                add_attr("Clone", &mut s.attrs);
                add_attr("Copy", &mut s.attrs);
            },
            Item::Enum(e) => {
                add_attr("Clone", &mut e.attrs);
                add_attr("Copy", &mut e.attrs);
            },
            _ => {},
        }
    }

    ast  
}

fn contains_attr(attr_str: &str, attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if format!("{}", attr.path.to_token_stream()) != "derive" { continue; }
        let Some(TokenTree::Group(g)) = attr.tokens.clone().into_iter().next() else { continue };
        if g.stream().into_iter().any(|t| format!("{}", t) == attr_str) { return true; }
    }

    false
}

fn add_attr(attr_str: &str, attrs: &mut Vec<Attribute>) {
    if !contains_attr(attr_str, attrs) {
        attrs.push(derive_attr(attr_str));
    }
}

// TODO create #[derive(attr_str)] directly
fn derive_attr(attr_str: &str) -> Attribute {
    let code = format!("#[derive({})] struct X;", attr_str);
    let Item::Struct(item) = parse_str::<Item>(&code).unwrap() else { unreachable!() };
    item.attrs[0].clone()
}
