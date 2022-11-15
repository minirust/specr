use syn::*;
use quote::*;
use proc_macro2::*;

// TODO support generics.

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
    let mut unnamed: Vec<TokenStream> = Vec::new();
    match &s.fields {
        Fields::Named(n) => {
            named = n.named.iter()
                           .map(|i| i.ident.as_ref().unwrap())
                           .collect();
        }
        Fields::Unnamed(u) => {
            unnamed = (0..u.unnamed.len()).map(|i| {
                            TokenTree::Literal(Literal::usize_unsuffixed(i)).into()
                        }).collect();
        }
        Fields::Unit => {},
    };

    let name = &s.ident;
    let ts = quote! {
        impl #name {
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
    let enum_ident = &e.ident;

    // contains the correct match-arm for each variant.
    let var_arms: Vec<TokenStream> = e.variants.iter().map(|v| {
        let ident = &v.ident;
        match &v.fields {
            Fields::Named(n) => {
                let names: Vec<&Ident> = n.named.iter().map(|x| x.ident.as_ref().unwrap()).collect();
                quote! {
                    #enum_ident::#ident { #( #names ),* } => {
                        #( #names.points_to(s); )*
                    }
                }
            }
            Fields::Unnamed(u) => {
                let names: Vec<Ident> = (0..u.unnamed.len()).map(|i| format_ident!("a{}", i)).collect();
                quote! {
                    #enum_ident::#ident(#( #names ),*) => {
                        #( #names.points_to(s); )*
                    }
                }
            }
            Fields::Unit => {
                quote! { #enum_ident::#ident => {} }
            }
        }
    }).collect();

    let ts = quote! {
        impl #enum_ident {
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
