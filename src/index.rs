use crate::prelude::*;

pub fn index(mut ast: syn::File) -> syn::File {
    Visitor.visit_file_mut(&mut ast);

    ast
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Index(idx) = &*node {
            // dbg!(&node);
            let lhs = &*idx.expr;
            let rhs = &*idx.index;
            let ts = quote! {
                (#lhs).index_at(#rhs)
            };
            let call: ExprMethodCall = parse2(ts).unwrap();
            *node = call.into();
        }

        visit_expr_mut(self, node);
    }
}
