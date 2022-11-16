use crate::prelude::*;

pub fn gccompat_impl(mut ast: syn::File) -> syn::File {
    let mut i = 0;
    while i < ast.items.len() {
        match &ast.items[i] {
            Item::Struct(s) => {
                ast.items.insert(i+1, impl_for_struct(s));
                i += 1;
            },
            Item::Enum(e) => {
                ast.items.insert(i+1, impl_for_enum(e));
                i += 1;
            },
            _ => {},
        };
        i += 1;
    }

    ast
}

fn impl_for_struct(s: &ItemStruct) -> Item {
    let mut named: Vec<&Ident> = Vec::new();
    let mut unnamed: Vec<Index> = Vec::new();
    match &s.fields {
        Fields::Named(n) => {
            named = n.named.iter()
                           .map(|i| i.ident.as_ref().unwrap())
                           .collect();
        }
        Fields::Unnamed(u) => {
            unnamed = (0..u.unnamed.len())
                        .map(syn::Index::from)
                        .collect()
        }
        Fields::Unit => {},
    };

    let name = &s.ident;
    let g = &s.generics;
    let tg = trimmed_generics(g);

    let ts = quote! {
        impl #g crate::specr::hidden::GcCompat for #name #tg {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
                #(
                    self.#named.points_to(s);
                )*
                #(
                    self.#unnamed.points_to(s);
                )*
            }
        }
    };
    syn::parse2(ts).unwrap()
}

fn impl_for_enum(e: &ItemEnum) -> Item {
    // contains the correct match-arm for each variant.
    let var_arms: Vec<TokenStream> = e.variants.iter().map(|v| {
        let ident = &v.ident;
        match &v.fields {
            Fields::Named(n) => {
                let names: Vec<&Ident> = n.named.iter().map(|x| x.ident.as_ref().unwrap()).collect();
                quote! {
                    Self::#ident { #( #names ),* } => {
                        #( #names.points_to(s); )*
                    }
                }
            }
            Fields::Unnamed(u) => {
                let names: Vec<Ident> = (0..u.unnamed.len()).map(|i| format_ident!("a{}", i)).collect();
                quote! {
                    Self::#ident(#( #names ),*) => {
                        #( #names.points_to(s); )*
                    }
                }
            }
            Fields::Unit => {
                quote! { Self::#ident => {} }
            }
        }
    }).collect();

    let enum_ident = &e.ident;
    let g = &e.generics;
    let tg = trimmed_generics(g);

    let ts = quote! {
        impl #g crate::specr::hidden::GcCompat for #enum_ident #tg {
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
                match self {
                    #( #var_arms )*
                }
            }
        }
        
    };
    syn::parse2(ts).unwrap()
}

fn trimmed_generics(g: &Generics) -> Generics {
    let mut g = g.clone();
    g.where_clause = None;
    g.params = g.params.iter().map(|p| {
        match p {
            GenericParam::Type(t) => {
                let mut t = t.clone();
                t.colon_token = None;
                t.bounds = Default::default();

                GenericParam::Type(t)
            },
            x => x.clone(),
        }
    }).collect();

    g
}
