use syn::*;
use proc_macro2::Span;

// adds #[macro_export] to each macro_rules
pub fn add_macro_exports(mut ast: syn::File) -> syn::File {
    for item in &mut ast.items {
        if let Item::Macro(ItemMacro { attrs, .. }) = item {
            attrs.push(macro_export());
        }
    }

    ast
}

fn macro_export() -> Attribute {
    let ident: Ident = Ident::new("macro_export", Span::call_site());
    let ps: PathSegment = ident.into();

    Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        path: ps.into(),
        tokens: Default::default(),
    }
}
