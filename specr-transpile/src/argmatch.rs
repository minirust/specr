use crate::prelude::*;

/// Resolve `argmatches` from the source code, by converting them to a match.
///
/// How does `argmatch` work:
/// There is a method with an argmatch attribute
/// impl Option<()> {
///     #[specr::argmatch(self)]
///     fn foo(self);
/// }
///
/// And then a sequence of submatches:
/// impl Foo {
///     fn foo(None: Self) { ... }
/// }
///
/// impl Foo {
///     fn foo(Some(()): Self) { ... }
/// }
///
/// This will be glued together into:
///
/// impl Option<()> {
///     fn foo(self) {
///         match self {
///             None => ...,
///             Some(()) => ...,
///         }
///     }
/// }
///
///
/// See the README for more information.
pub fn argmatch(mut mods: Vec<Module>) -> Vec<Module> {
    for m in mods.iter_mut() {
        m.ast = argmatch_ast(m.ast.clone());
    }

    mods
}

// represents a `fn` item within an impl block.
#[derive(PartialEq, Eq)]
struct MethodIdx {
    item_idx: usize,
    fn_idx: usize,
}

impl MethodIdx {
    fn as_ref<'a>(&self, ast: &'a syn::File) -> &'a ImplItemMethod {
        let Item::Impl(ref ii) = ast.items[self.item_idx] else { panic!() };
        let ImplItem::Method(ref iim) = ii.items[self.fn_idx] else { panic!() };
        iim
    }

    fn as_mut<'a>(&self, ast: &'a mut syn::File) -> &'a mut ImplItemMethod {
        let Item::Impl(ref mut ii) = ast.items[self.item_idx] else { panic!() };
        let ImplItem::Method(ref mut iim) = ii.items[self.fn_idx] else { panic!() };
        iim
    }
}

// expresses everything that can be contained in an `specr::argmatch` attribute.
struct AttrInfo {
    // which attribute is the argmatch attribute.
    attr_idx: usize,

    // the index and ident of the function argument we match upon
    // Typically `match_idx = 0` and `match_ident = self`.
    match_idx: usize,
    match_ident: Ident,
}

struct Argmatch {
    // the method which has the `argmatch` attribute.
    method_idx: MethodIdx,

    // the information in this attribute.
    attr_info: AttrInfo,
}

fn argmatch_ast(mut ast: syn::File) -> syn::File {
    while let Some(argmatch) = locate_argmatch(&ast) {
        let submatches = locate_submatches(&argmatch, &ast);
        let block = construct_block(&argmatch, &ast, &submatches[..]);

        let r = argmatch.method_idx.as_mut(&mut ast);

        // remove the `argmatch` attribute.
        r.attrs.remove(argmatch.attr_info.attr_idx);

        // set the newly-constructed block.
        r.block = block;

        clear_submatches(&mut ast, submatches);
    }

    ast
}

// finds a method with an #[specr::argmatch] attribute.
fn locate_argmatch(ast: &syn::File) -> Option<Argmatch> {
    for (i, x) in ast.items.iter().enumerate() {
        let Item::Impl(ii) = x else { continue };
        for (j, y) in ii.items.iter().enumerate() {
            let ImplItem::Method(ref iim) = y else { continue };
            let Some(attr_info) = get_attr_info(iim) else { continue };
            let method_idx = MethodIdx { item_idx: i, fn_idx: j };
            return Some(Argmatch { attr_info, method_idx });
        }
    }

    None
}

fn construct_block(argmatch: &Argmatch, ast: &syn::File, submatches: &[MethodIdx]) -> Block {
    let match_ident = &argmatch.attr_info.match_ident;
    let pats: Vec<&Pat> = submatches.iter().map(|x| {
            let iim = x.as_ref(ast);
            let FnArg::Typed(ref pt) = iim.sig.inputs[argmatch.attr_info.match_idx] else {
                panic!("expected match-able pattern, got `self`!")
            };

            &*pt.pat
        }).collect();
    let blocks: Vec<&Block> = submatches.iter().map(|x| &x.as_ref(ast).block).collect();

    let tokens = quote! {{
        match #match_ident {
            #(#pats => #blocks,)*
        }
    }};
    parse2(tokens).expect("Cannot parse block!")
}

// returns the submatches in the order they are written down in the input file.
fn locate_submatches(argmatch: &Argmatch, ast: &syn::File) -> Vec<MethodIdx> {
    let mut submatches = Vec::new();

    for (i, x) in ast.items.iter().enumerate() {
        let Item::Impl(ref ii) = x else { continue };
        for (j, y) in ii.items.iter().enumerate() {
            let ImplItem::Method(_) = y else { continue };

            let method_idx = MethodIdx { item_idx: i, fn_idx: j };
            match is_submatch(argmatch, &method_idx, ast) {
                SubmatchResult::Yes => {
                    submatches.push(method_idx);
                },
                SubmatchResult::No => {},
                SubmatchResult::YesButMismatch { error_msg } => {
                    panic!("{}", error_msg);
                },
            }
        }
    }

    submatches
}

enum SubmatchResult {
    Yes,
    No,

    // It seems to be a submatch, but something is off.
    // This generates an error.
    YesButMismatch { error_msg: String },
}

fn is_submatch(argmatch: &Argmatch, method_idx: &MethodIdx, ast: &syn::File) -> SubmatchResult {
    if *method_idx == argmatch.method_idx {
        // this is no "submatch", it's the original method_idx itself!
        return SubmatchResult::No;
    }

    let iim1 = argmatch.method_idx.as_ref(ast);
    let iim2 = method_idx.as_ref(ast);

    if iim1.sig.ident != iim2.sig.ident {
        // this is not a submatch, it's not even the same function name.
        return SubmatchResult::No;
    }

    // check that signature are the same, except for the FnArg we match upon.
    let hide_match_ident = |sig: &Signature| {
        let default_receiver = parse2(quote!{self}).unwrap();
        let mut sig = sig.clone();
        sig.inputs[argmatch.attr_info.match_idx] = default_receiver;

        sig
    };
    let sig1 = hide_match_ident(&iim1.sig);
    let sig2 = hide_match_ident(&iim2.sig);

    if sig1 != sig2 {
        let error_msg = format!("`argmatch` encountered signature mismatch!\n{}\n{}\n", iim1.sig.to_token_stream(), iim2.sig.to_token_stream());
        return SubmatchResult::YesButMismatch { error_msg };
    }

    // check that the impl blocks are compatible
    let hide_items = |item_idx: usize| {
        let Item::Impl(ii) = &ast.items[item_idx] else { unreachable!() };
        let mut ii = ii.clone();
        ii.items.clear();
        ii
    };

    let ii1 = hide_items(argmatch.method_idx.item_idx);
    let ii2 = hide_items(method_idx.item_idx);

    if ii1 != ii2 {
        let error_msg = format!("`argmatch` encountered impl-block mismatch!\n{}\n{}\n", ii1.to_token_stream(), ii2.to_token_stream());
        return SubmatchResult::YesButMismatch { error_msg };
    }

    SubmatchResult::Yes

}

fn clear_submatches(ast: &mut syn::File, mut submatches: Vec<MethodIdx>) {
    submatches.reverse();

    for s in submatches {
        let Item::Impl(ref mut ii) = ast.items[s.item_idx] else { panic!() };
        ii.items.remove(s.fn_idx);

        // it the resulting impl block would then be empty, remove it.
        if ii.items.is_empty() {
            ast.items.remove(s.item_idx);
        }
    }
}

// Searches for an `argmatch` attribute and returns its info, if successful.
fn get_attr_info(iim: &ImplItemMethod) -> Option<AttrInfo> {
    let attrs = &iim.attrs;
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

            let attr_idx = i;
            let match_ident = format_ident!("{}", g.stream().to_string());
            let match_idx = if match_ident == "self" {
                0
            } else {
                iim.sig.inputs.iter().position(|arg| {
                    let FnArg::Typed(pat_ty) = arg else { return false };
                    let Pat::Ident(pi) = &*pat_ty.pat else { return false };

                    pi.ident == match_ident
                }).expect("Cannot find argmatch match_idx")
            };

            return Some(AttrInfo { attr_idx, match_ident, match_idx });
        }
    }

    None
}
