use crate::typerec::*;

/// finds identifiers within a pattern that need to be get'ed before being used.
pub(in crate::typerec) fn pat_idents(pat: &Pat, elements: &HashSet<VariantElement>) -> HashSet<Ident> {
    let mut v = Visitor {
        elements,
        idents: HashSet::new(),
    };
    v.visit_pat(pat);

    v.idents
}

struct Visitor<'a> {
    elements: &'a HashSet<VariantElement>,
    idents: HashSet<Ident>,
}

impl Visit<'_> for Visitor<'_> {
    fn visit_pat_struct(&mut self, pat: &PatStruct) {
        let var = fix::extract_variant(&pat.path);
        for e in self.elements {
            if e.variant != var { continue; }
            let ElementIdx::Named(n) = &e.idx else { continue };

            for f in &pat.fields {
                let Member::Named(m) = &f.member else { continue };
                if m != n { continue; }

                if let Pat::Ident(id) = &*f.pat {
                    self.idents.insert(id.ident.clone());
                }
            }
        }

        visit_pat_struct(self, pat);
    }

    fn visit_pat_tuple_struct(&mut self, pat: &PatTupleStruct) {
        let var = fix::extract_variant(&pat.path);
        for e in self.elements {
            if e.variant != var { continue; }
            let ElementIdx::Unnamed(idx) = &e.idx else { continue };

            let Some(f) = pat.elems.iter().nth(*idx) else { continue };
            if let Pat::Ident(id) = f {
                self.idents.insert(id.ident.clone());
            }
        }

        visit_pat_tuple_struct(self, pat);
    }
}
