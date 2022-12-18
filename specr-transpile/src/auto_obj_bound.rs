use crate::prelude::*;

/// Adds the implicit `T: Obj` bound at all relevant places.
pub fn auto_obj_bound(mut ast: syn::File) -> syn::File {
    for i in ast.items.iter_mut() {
        match i {
            Item::Struct(s) => {
                add_bound(&mut s.generics);
            },
            Item::Enum(e) => {
                add_bound(&mut e.generics);
            },
            Item::Fn(f) => {
                add_bound(&mut f.sig.generics);
            },
            Item::Trait(t) => {
                add_bound(&mut t.generics);
                add_bound_punct(&mut t.supertraits);
                for it in &mut t.items {
                    match it {
                        TraitItem::Method(itm) => {
                            add_bound(&mut itm.sig.generics);
                        },
                        TraitItem::Type(itt) => {
                            add_bound(&mut itt.generics);
                            add_bound_punct(&mut itt.bounds);
                        },
                        _ => {},
                    }
                }
            },
            Item::Impl(i) => {
                add_bound(&mut i.generics);
                for ii in &mut i.items {
                    match ii {
                        ImplItem::Method(iim) => {
                            add_bound(&mut iim.sig.generics);
                        },
                        ImplItem::Type(iit) => {
                            add_bound(&mut iit.generics);
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

pub fn add_bound(g: &mut Generics) {
    for param in &mut g.params {
        let GenericParam::Type(t) = param else { continue };
        add_bound_punct(&mut t.bounds);
    }
}

pub fn add_bound_punct<T: Default>(punct: &mut Punctuated<TypeParamBound, T>) {
    let b = quote! { specr::hidden::Obj };
    let b: TraitBound = parse2(b).unwrap();
    let b: TypeParamBound = b.into();
    punct.push(b);
}
