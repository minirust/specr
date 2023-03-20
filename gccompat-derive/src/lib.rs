// #![feature(proc_macro_quote)]

extern crate proc_macro;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::*;

#[proc_macro_derive(GcCompat)]
pub fn gccompat_derive(input: TokenStream1) -> TokenStream1 { 
    let i: syn::Item = parse(input).unwrap();
    match &i {
        Item::Struct(s) => impl_for_struct(s).to_token_stream().into(),
        Item::Enum(e) => impl_for_enum(e).to_token_stream().into(),
        _ => panic!("#[derive(GcCompat)] applied to invalid item!"),
    }
}

/// Generates `impl GcCompat for _`-Item for a struct.
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
    let g = generics_base(&s.generics);
    let tg = generics_trim(&g);

    let ts = quote! {
        impl #g libspecr::hidden::GcCompat for #name #tg {
            fn as_any(&self) -> &dyn std::any::Any { self }
            #[allow(unused_variables)]
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

/// Generates `impl GcCompat for _`-Item for an enum.
fn impl_for_enum(e: &ItemEnum) -> Item {
    // contains the correct match-arm for each variant.
    let var_arms: Vec<TokenStream2> = e.variants.iter().map(|v| {
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
    let g = generics_base(&e.generics);
    let tg = generics_trim(&g);

    let ts = quote! {
        impl #g libspecr::hidden::GcCompat for #enum_ident #tg {
            fn as_any(&self) -> &dyn std::any::Any { self }
            #[allow(unused_variables)]
            fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
                match *self {
                    #( #var_arms )*
                }
            }
        }
        
    };
    syn::parse2(ts).unwrap()
}

fn gccompat_bound() -> TypeParamBound {
    parse2(quote! { libspecr::hidden::GcCompat }).unwrap()
}

// removes defaults from generics, and adds GcCompat to the bounds.
// <T : Clone = ()> -> <T : Clone + GcCompat>
fn generics_base(g: &Generics) -> Generics {
    let mut g = g.clone();
    g.params = g.params.iter().map(|p| {
        match p {
            GenericParam::Type(t) => {
                let mut t = t.clone();
                t.default = None;
                t.bounds.push(gccompat_bound());

                GenericParam::Type(t)
            },
            x => x.clone(),
        }
    }).collect();

    g
}

// removes defaults and bounds from generics
// <T : Clone = ()> ==> <T>
fn generics_trim(g: &Generics) -> Generics {
    let mut g = g.clone();
    g.where_clause = None;
    g.params = g.params.iter().map(|p| {
        match p {
            GenericParam::Type(t) => {
                let mut t = t.clone();
                t.colon_token = None;
                t.bounds = Default::default();
                t.default = None;

                GenericParam::Type(t)
            },
            x => x.clone(),
        }
    }).collect();

    g
}

