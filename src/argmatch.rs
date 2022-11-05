use std::mem;
use syn::{*, token::{Brace, Match}, punctuated::Punctuated};
use proc_macro2::{Ident, Span};
use quote::ToTokens;

// ItemImpl, but with only one item inside
type SingletonItemImpl = ItemImpl;

pub fn argmatch(mut arg: syn::File) -> syn::File {
    let fwd_decls = extract_forward_declarations(&mut arg);
    for fwd_decl in fwd_decls {
        let impls = extract_implementations(&fwd_decl, &mut arg);
        if impls.is_empty() { continue; }
        let idx = get_match_idx(&fwd_decl, &impls);
        let item_impl = merge_implementations(fwd_decl, impls, idx);
        arg.items.push(Item::Impl(item_impl));
    }

    arg
}

fn is_fwd_decl_block(b: &Block) -> bool {
    if b.stmts.len() != 1 { return false; }
    if let Stmt::Item(Item::Verbatim(ts)) = &b.stmts[0] {
        format!("{}", &ts) == ";"
    } else { false }
}

fn extract_forward_declarations(arg: &mut syn::File) -> Vec<SingletonItemImpl> {
    let mut out = Vec::new();

    for i in &mut arg.items {
        if let Item::Impl(x) = i {
            let mut old_items = Vec::new();
            mem::swap(&mut x.items, &mut old_items);

            let mut new_items = Vec::new();

            for ii in old_items {
                if let ImplItem::Method(iim) = &ii && is_fwd_decl_block(&iim.block) {
                    let mut candidate = x.clone();
                    candidate.items = vec![ii];
                    out.push(candidate);
                } else {
                    new_items.push(ii);
                }
            }
            mem::swap(&mut x.items, &mut new_items);
        }
    }

    out
}

fn get_match_idx(_fwd_decl: &SingletonItemImpl, impls: &[ImplItemMethod]) -> usize {
    let n = impls[0].sig.inputs.len();
    for i in 0..n {
        let f = |j: usize| format!("{}", impls[j].sig.inputs[i].to_token_stream());
        if f(0) != f(1) {
            return i;
        }
    }
    unreachable!()
}

fn method_fits_fwd_decl(fwd_decl: &SingletonItemImpl, iim: &ImplItemMethod) -> bool {
    if let ImplItem::Method(singleton_iim) = &fwd_decl.items[0] {
        let fwd_ident = &singleton_iim.sig.ident;
        let iim_ident = &iim.sig.ident;
        format!("{}", fwd_ident) == format!("{}", iim_ident)
    } else { unreachable!() }
}

// extracts all fitting ImplItemMethods
fn extract_implementations(fwd_decl: &SingletonItemImpl, arg: &mut syn::File) -> Vec<ImplItemMethod> {
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

fn merge_implementations(fwd: SingletonItemImpl, impls: Vec<ImplItemMethod>, idx: usize) -> ItemImpl {
    let match_ident: Ident = if let ImplItem::Method(iim) = &fwd.items[0] {
        match &iim.sig.inputs[idx] {
            FnArg::Receiver(_) => { Ident::new("self", Span::call_site()) },
            FnArg::Typed(pt) => {
                if let Pat::Ident(pi) = &*pt.pat {
                    pi.ident.clone()
                } else { unreachable!() }
            },
        }
    } else { unreachable!() };

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

    let mut out = fwd;

    let mut arms = Vec::new();
    for iim in impls {
        let pat = if let FnArg::Typed(pt) = &iim.sig.inputs[idx] {
            pt.pat.clone()
        } else { unreachable!() };

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

    if let ImplItem::Method(iim) = &mut out.items[0] {
        iim.block.stmts = vec![
            Stmt::Expr(Expr::Match(ExprMatch {
                attrs: Vec::new(),
                match_token: Match::default(),
                expr: match_expr,
                brace_token: Brace::default(),
                arms,
            }))
        ];
    } else { unreachable!() }

    out
        
}
