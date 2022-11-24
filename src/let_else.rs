use crate::prelude::*;

use std::collections::HashSet;

/// Sadly, let-else patterns are recognized by syn only as Stmt::Semi(Expr::Verbatim(_)), so our other stages cannot correctly work on them.
/// Hence we translate let-else ourselves.
pub fn let_else(mut ast: syn::File) -> syn::File {
    let mut v = Visitor;
    v.visit_file_mut(&mut ast);

    ast
}

struct Visitor;

impl VisitMut for Visitor {
    fn visit_stmt_mut(&mut self, node: &mut Stmt) {
        match node {
            Stmt::Semi(Expr::Verbatim(verb), _) => {
                let s = verb.to_string();
                if s.contains("let") && s.contains("=") && s.contains("else") {
                    *node = resolve_let_else(&s);
                }
            },
            _ => {},
        }
        visit_stmt_mut(self, node);
    }
}

fn resolve_let_else(s: &str) -> Stmt {
    let let_i = s.find("let").unwrap();
    let eq_i = s.find("=").unwrap();
    let else_i = s.find("else").unwrap();

    let pattern = &s[let_i+3 .. eq_i];
    let expr = &s[eq_i+1 .. else_i];
    let blk = &s[else_i+4 .. ];

    let pattern = parse_str::<Pat>(pattern).unwrap();
    let expr = parse_str::<Expr>(expr).unwrap();
    let blk = parse_str::<Block>(blk).unwrap();

    let idents = pat_idents(&pattern);

    let expr = quote! {
        let ( #(#idents),* ) = match #expr {
            #pattern => ( #(#idents),* ),
            _ => #blk,
        };
    };

    parse2(expr).unwrap()
}


// pat visitor

/// finds identifiers that are created within a pattern.
pub fn pat_idents(pat: &Pat) -> Vec<Ident> {
    let mut v = PatVisitor {
        idents: HashSet::new(),
    };
    v.visit_pat(pat);

    v.idents.into_iter()
            .map(|pi| pi.ident)
            .collect()
}

struct PatVisitor {
    idents: HashSet<PatIdent>,
}

impl Visit<'_> for PatVisitor {
    fn visit_pat_ident(&mut self, pat: &PatIdent) {
        self.idents.insert(pat.clone());

        visit_pat_ident(self, pat);
    }
}

