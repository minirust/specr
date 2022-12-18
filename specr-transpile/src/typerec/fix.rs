use crate::typerec::*;

/// Fix constructing and matching of enum variants that were `GcCow<_>`-wrapped.
pub(in crate::typerec) fn fix(mods: &mut [Module], elements: &HashSet<VariantElement>) {
    for m in mods {
        Visitor { elements }.visit_file_mut(&mut m.ast);
    }
}

struct Visitor<'a> {
    elements: &'a HashSet<VariantElement>,
}

impl VisitMut for Visitor<'_> {
    // fixup named enum variant construction:
    // `Variant { x: 2 }` ==> `Variant { x: specr::hidden::GcCow::new(2) }`
    fn visit_expr_struct_mut(&mut self, i: &mut ExprStruct) {
        for e in self.elements {
            let ElementIdx::Named(name) = &e.idx else { continue };

            let p = extract_variant(&i.path);
            if p == e.variant {
                for f in &mut i.fields {
                    let Member::Named(m) = &f.member else { continue };
                    if m == name {
                        // this solves the case `Variant { x }`.
                        f.colon_token = Some(Default::default());

                        wrap_expr(&mut f.expr);
                    }
                }
            }
        }

        visit_expr_struct_mut(self, i);
    }

    // fixup unnamed enum variant construction:
    // `Some(2)` ==> `Some(specr::hidden::GcCow::new(2))`
    fn visit_expr_call_mut(&mut self, i: &mut ExprCall) {
        for e in self.elements {
            let ElementIdx::Unnamed(idx) = &e.idx else { continue };
            let Expr::Path(p) = &*i.func else { continue };
            let var = extract_variant(&p.path);

            if e.variant == var {
                let Some(arg_ref) = i.args.iter_mut().nth(*idx) else { continue };
                wrap_expr(arg_ref);
            }
        }

        visit_expr_call_mut(self, i);
    }

    // fixup matches:
    // `Foo { x } => { ... }` ==> `Foo { x } => { let x = x.get(); ... }`
    fn visit_arm_mut(&mut self, i: &mut Arm) {
        let idents: Vec<_> = pat_idents::pat_idents(&i.pat, self.elements).into_iter().collect();
        let body = &i.body;
        let body = quote! {
            {
                #( let #idents = #idents.get(); )*
                #body
            }
        };
        i.body = Box::new(parse2(body).unwrap());

        visit_arm_mut(self, i);
    }
}

// wraps an Expr in specr::hidden::GcCow::new(_)
fn wrap_expr(expr: &mut Expr) {
    let e = quote! {
        specr::hidden::GcCow::new(#expr)
    };
    *expr = parse2(e).unwrap();
}

// extract the last segment (i.e. the enum Variant) from a path
// `Foo::Bar` => `Bar`
pub(in crate::typerec) fn extract_variant(p: &Path) -> Ident {
    let p = p.segments.iter().last().unwrap(); 
    p.ident.clone()
}
