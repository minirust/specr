use crate::prelude::*;

const BOUNDS: [&'static str; 5] = ["Debug", "Eq", "specr::hidden::GcCompat", "Copy", "Hash"];

pub fn autobounds(mut ast: syn::File) -> syn::File {
    for i in ast.items.iter_mut() {
        match i {
            Item::Struct(s) => {
                add_bounds(&mut s.generics);
            },
            Item::Enum(e) => {
                add_bounds(&mut e.generics);
            },
            Item::Fn(f) => {
                add_bounds(&mut f.sig.generics);
            },
            Item::Trait(t) => {
                add_bounds(&mut t.generics);
                add_bounds_punct(&mut t.supertraits);
                for it in &mut t.items {
                    match it {
                        TraitItem::Method(itm) => {
                            add_bounds(&mut itm.sig.generics);
                        },
                        TraitItem::Type(itt) => {
                            add_bounds(&mut itt.generics);
                            add_bounds_punct(&mut itt.bounds);
                        },
                        _ => {},
                    }
                }
            },
            Item::Impl(i) => {
                add_bounds(&mut i.generics);
                for ii in &mut i.items {
                    match ii {
                        ImplItem::Method(iim) => {
                            add_bounds(&mut iim.sig.generics);
                        },
                        ImplItem::Type(iit) => {
                            add_bounds(&mut iit.generics);
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

pub fn add_bounds(g: &mut Generics) {
    for param in &mut g.params {
        let GenericParam::Type(t) = param else { continue };
        add_bounds_punct(&mut t.bounds);
    }
}

pub fn add_bounds_punct<T: Default>(punct: &mut Punctuated<TypeParamBound, T>) {
    for b in BOUNDS {
        let b: TraitBound = parse_str(b).unwrap();
        let b: TypeParamBound = b.into();
        punct.push(b);
    }
}
