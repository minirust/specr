use crate::prelude::*;
use std::mem;

struct FwdDeclaration {
    // stores all generic parameters etc. of the surrounding impl block
    // but empty_ii.items is empty, so it's an empty impl block.
    empty_ii: ItemImpl,

    // stores the actual method forward declaration
    iim: ImplItemMethod,

    match_ident: String,
    match_idx: usize,
}

pub fn argmatch(mut mods: Vec<Module>) -> Vec<Module> {
    for m in mods.iter_mut() {
        m.ast = argmatch_ast(m.ast.clone());
    }

    mods
}

fn argmatch_ast(mut arg: syn::File) -> syn::File {
    let fwd_decls = extract_fwd_decls(&mut arg);
    for fwd_decl in fwd_decls {
        let impls = extract_implementations(&fwd_decl, &mut arg);
        let item_impl = merge_implementations(fwd_decl, impls);
        arg.items.push(Item::Impl(item_impl));
    }

    arg
}

fn get_argmatch_attr(attrs: &mut Vec<Attribute>) -> Option</*match ident: */ String> {
    for i in 0..attrs.len() {
        let attr = &attrs[i];
        let segments: Vec<String> = attr.path.segments
                                             .iter()
                                             .map(|x| format!("{}", x.to_token_stream()))
                                             .collect();
        let [l, r] = &segments[..] else { continue };
        if l == "specr" && r == "argmatch" {
            let Some(tok) = attr.tokens.clone().into_iter().next() else { continue };
            let TokenTree::Group(g) = tok else { continue };
            let match_ident = format!("{}", g.stream());
            attrs.remove(i);
            return Some(match_ident);
        }
    }

    None
}

fn get_fwd_decl(ii: &ItemImpl, iim: &mut ImplItemMethod) -> Option<FwdDeclaration> {
    let mut empty_ii = ii.clone();
    empty_ii.items = Vec::new();

    let match_ident = get_argmatch_attr(&mut iim.attrs)?;

    let match_idx = if match_ident == "self" {
        0
    } else {
        iim.sig.inputs.iter().position(|arg| {
            let FnArg::Typed(pat_ty) = arg else { return false };
            format!("{}", pat_ty.pat.to_token_stream()) == match_ident
        }).expect("Cannot find argmatch match_idx")
    };

    let iim = iim.clone();
    Some(FwdDeclaration {
        empty_ii,
        iim,
        match_ident,
        match_idx,
    })
}

fn extract_fwd_decls(arg: &mut syn::File) -> Vec<FwdDeclaration> {
    let mut out = Vec::new();

    for i in &mut arg.items {
        if let Item::Impl(x) = i {
            let mut old_items: Vec<syn::ImplItem> = Vec::new();
            mem::swap(&mut x.items, &mut old_items);

            let mut new_items: Vec<syn::ImplItem> = Vec::new();

            for mut ii in old_items {
                if let ImplItem::Method(iim) = &mut ii {
                    if let Some(fwd_decl) = get_fwd_decl(x, iim) {
                        out.push(fwd_decl);
                        continue;
                    }
                }

                // called if ii is not a fwd declaration
                new_items.push(ii);
            }
            mem::swap(&mut x.items, &mut new_items);
        }
    }

    out
}

fn method_fits_fwd_decl(fwd_decl: &FwdDeclaration, iim: &ImplItemMethod) -> bool {
    let fwd_ident = &fwd_decl.iim.sig.ident;
    let iim_ident = &iim.sig.ident;
    format!("{}", fwd_ident) == format!("{}", iim_ident)
}

// extracts all fitting ImplItemMethods
fn extract_implementations(fwd_decl: &FwdDeclaration, arg: &mut syn::File) -> Vec<ImplItemMethod> {
    let mut out = Vec::new();

    for i in &mut arg.items {
        if let Item::Impl(x) = i {
            let mut old_items = Vec::new();
            mem::swap(&mut x.items, &mut old_items);

            let mut new_items = Vec::new();

            for ii in old_items {
                if let ImplItem::Method(iim) = &ii && method_fits_fwd_decl(fwd_decl, &iim) {
                    out.push(iim.clone());
                } else {
                    new_items.push(ii);
                }
            }

            mem::swap(&mut x.items, &mut new_items);
        }
    }

    out
}

fn merge_implementations(fwd_decl: FwdDeclaration, impls: Vec<ImplItemMethod>) -> ItemImpl {
    let match_ident = Ident::new(&fwd_decl.match_ident, Span::call_site());

    let match_expr = Box::new(Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: {
                let mut punct = Punctuated::new();
                punct.push_value(PathSegment::from(match_ident));
                punct
            }
        }
    }));

    let mut arms = Vec::new();
    for iim in impls {
        let FnArg::Typed(pt) = &iim.sig.inputs[fwd_decl.match_idx] else { unreachable!() };
        let pat = pt.pat.clone();

        arms.push(Arm {
            attrs: Vec::new(),
            pat: *pat,
            guard: None,
            fat_arrow_token: Default::default(),
            body: Box::new(Expr::Block(ExprBlock {
                attrs: Vec::new(),
                label: None,
                block: iim.block,
            })),
            comma: Some(Default::default()),
        });
    }

    let mut iim = fwd_decl.iim.clone();
    iim.block.stmts = vec![
        Stmt::Expr(Expr::Match(ExprMatch {
            attrs: Vec::new(),
            match_token: Default::default(),
            expr: match_expr,
            brace_token: Default::default(),
            arms,
        }))
    ];

    let mut out = fwd_decl.empty_ii.clone();
    out.items = vec![ImplItem::Method(iim)];

    out
}
