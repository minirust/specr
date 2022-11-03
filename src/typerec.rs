use syn::*;
use quote::ToTokens;
use proc_macro2::Span;

fn has_inf_recursion(e: &ItemEnum) -> bool {
    let ident = format!("{}", e.ident);

    for var in e.variants.iter() {
        let fields: Vec<&syn::Field> = match &var.fields {
            Fields::Named(x) => x.named.iter().collect(),
            Fields::Unnamed(x) => x.unnamed.iter().collect(),
            Fields::Unit => Vec::new(),
        };

        for field in fields {
            let ty_str = format!("{}", field.ty.to_token_stream());
            if ty_str == ident {
                return true;
            }
        }
    }

    false
}

pub fn fix(mut ast: syn::File) -> syn::File {
    let mut typedefs: Vec<Item> = Vec::new();

    for it in &mut ast.items {
        if let Item::Enum(e) = it {
            if has_inf_recursion(e) {
                let s = format!("{}", e.ident);
                let s_raw = format!("{s}_Raw");
                e.ident = Ident::new(&s_raw, Span::call_site());

                let typedef = format!("pub type {s} = std::rc::Rc<{s_raw}>;");
                let typedef = syn::parse_str::<syn::Item>(&typedef).unwrap();
                typedefs.push(typedef);
            }
        }
    }

    ast.items.extend(typedefs);

    ast
}
