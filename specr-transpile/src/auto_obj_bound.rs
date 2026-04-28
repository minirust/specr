use crate::prelude::*;

/// Adds the implicit `T: Obj` bound at all relevant places.
pub fn auto_obj_bound(mut ast: syn::File) -> syn::File {
    for i in ast.items.iter_mut() {
        match i {
            Item::Struct(s) => {
                add_obj_bound(&mut s.generics);
            },
            Item::Enum(e) => {
                add_obj_bound(&mut e.generics);
            },
            Item::Fn(f) => {
                add_obj_bound(&mut f.sig.generics);
            },
            Item::Trait(t) => {
                add_obj_bound(&mut t.generics);
                add_obj_bound_punct(&mut t.supertraits);
                for it in &mut t.items {
                    match it {
                        TraitItem::Fn(itm) => {
                            add_obj_bound(&mut itm.sig.generics);
                        },
                        TraitItem::Type(itt) => {
                            add_obj_bound(&mut itt.generics);
                            add_obj_bound_punct(&mut itt.bounds);
                            add_serde_bounds_punct(&mut itt.bounds);
                        },
                        _ => {},
                    }
                }
            },
            Item::Impl(i) => {
                add_obj_bound(&mut i.generics);
                // We need serde bounds for trait impls. We choose to add them for all impl blocks for consistency.
                add_serde_bounds(&mut i.generics);
                for ii in &mut i.items {
                    match ii {
                        ImplItem::Fn(iim) => {
                            add_obj_bound(&mut iim.sig.generics);
                        },
                        ImplItem::Type(iit) => {
                            add_obj_bound(&mut iit.generics);
                        },
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }

    ast
}

pub fn add_obj_bound(g: &mut Generics) {
    for param in &mut g.params {
        let GenericParam::Type(t) = param else { continue };
        add_obj_bound_punct(&mut t.bounds);
    }
}

pub fn add_obj_bound_punct<T: Default>(punct: &mut Punctuated<TypeParamBound, T>) {
    let b = quote! { libspecr::hidden::Obj };
    let b: TraitBound = parse2(b).unwrap();
    let b: TypeParamBound = b.into();
    punct.push(b);
}


/// Associated types get an additional bound of `Serialize + for<'de> Deserialize<'de>`.
/// This is because we don't include this bound in `Obj` as that messes up `serde_derive`,
/// which then results in an "multiple `impl`s or `where` clauses satisfying `K: Deserialize<'_>` found" warning.
/// (We could add `Serialize` to `Obj` but choose not to, for symmetry reasons.)
/// It turns out that this is not just necessary in associated types, but also in `impl<T1, .., Tn> Trait for Type` generic 
/// parameters (T1 to Tn), since these usually end up contributing to the associated type.
pub fn add_serde_bounds(g: &mut Generics) {
    for param in &mut g.params {
        let GenericParam::Type(t) = param else {
            continue;
        };
        add_serde_bounds_punct(&mut t.bounds);
    }
}

pub fn add_serde_bounds_punct<T: Default>(punct: &mut Punctuated<TypeParamBound, T>) {
    let b = quote! { for<'de> serde::Deserialize<'de> };
    let b: TraitBound = parse2(b).unwrap();
    let b: TypeParamBound = b.into();
    punct.push(b);
    let b = quote! { serde::Serialize };
    let b: TraitBound = parse2(b).unwrap();
    let b: TypeParamBound = b.into();
    punct.push(b);
}
