use syn::*;

// `type X;` => `use baselib::X;`
// `pub type X;` => `pub use baselib::X;`
pub fn apply_baselib_use(mut ast: syn::File) -> syn::File {
    for x in ast.items.iter_mut() {
        if let Item::Verbatim(ts) = x {
            let mut tokens: Vec<String> = ts.clone()
                                        .into_iter()
                                        .map(|t| format!("{}", t))
                                        .collect();
            // get rid of doc comments
            while &*tokens[0] == "#" {
                tokens.remove(0); // removes #
                tokens.remove(0); // removes [ "comment" ]
            }

            // get rid of `pub`
            if &*tokens[0] == "pub" {
                tokens.remove(0);
            }

            let Some(type_tok) = &tokens.get(0) else { continue };
            let Some(ident_tok) = &tokens.get(1) else { continue };
            if &**type_tok == "type" {
                let use_item = format!("pub use baselib::{ident_tok};\n");
                *x = parse_str::<Item>(&use_item).unwrap();
            }
        }
    }

    ast
}
