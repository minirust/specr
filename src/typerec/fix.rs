use syn::visit_mut::*;
use syn::token::Colon;

use crate::typerec::*;

pub(in crate::typerec) fn fix(mods: &mut [Module], elements: &HashSet<VariantElement>) {
    for m in mods {
        Visitor { elements }.visit_file_mut(&mut m.ast);
    }
}

struct Visitor<'a> {
    elements: &'a HashSet<VariantElement>,
}

impl VisitMut for Visitor<'_> {
    // `Variant { x: 2 }` ==> `Variant { x: Rc::new(2) }`
    fn visit_expr_struct_mut(&mut self, i: &mut ExprStruct) {
        for e in self.elements {
            let ElementIdx::Named(name) = &e.idx else { continue };

            let p = extract_variant(&i.path);
            if p == e.variant {
                for f in &mut i.fields {
                    let m = format!("{}", f.member.to_token_stream());
                    if &m == name {
                        // this solves the case `Variant { x }`.
                        f.colon_token = Some(Colon::default());

                        wrap_expr(&mut f.expr);
                    }
                }
            }
        }

        visit_expr_struct_mut(self, i);
    }

    // `Some(2)` ==> `Some(Rc::new(2))`
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

    // `Foo { x } => { ... }` ==> `Foo { x } => { let x = *x; ... }`
    fn visit_arm_mut(&mut self, i: &mut Arm) {
        let idents = pat_idents::pat_idents(&i.pat, self.elements);

        // TODO use quote for stuff like this:
        let mut s = String::from("{");
        for id in idents {
            // TODO write my own deref function to prevent referencing of being in the way.
            s.push_str(&format!("let {id} = baselib::hidden::deref_rc(&{id});"));
        }
        s.push_str(&format!("{}", i.body.to_token_stream()));
        s.push_str("}");
        let new_body = parse_str::<Expr>(&s).unwrap();

        i.body = Box::new(new_body);

        visit_arm_mut(self, i);
    }
}

// wraps an Expr in Rc::new(_)
fn wrap_expr(expr: &mut Expr) {
    let e = format!("std::rc::Rc::new({})", expr.to_token_stream());
    let e = parse_str::<Expr>(&e).unwrap();
    *expr = e;
}

// extract the last segment (i.e. the enum Variant) from a path
// `Foo::Bar` => `Bar`
pub(in crate::typerec) fn extract_variant(p: &Path) -> String {
    let p = p.segments.iter().last().unwrap(); 
    let p = format!("{}", p.to_token_stream());

    p
}

