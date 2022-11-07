use syn::*;

fn is_verbatim_method(item: &ImplItem) -> bool{
    if let ImplItem::Method(iim) = item {
        iim.block.stmts.iter()
            .any(|x| matches!(x, Stmt::Item(Item::Verbatim(_))))
    } else { false }
}

pub fn clear_verbatim(mut ast: syn::File) -> syn::File {
    // TODO still relevant?
    ast.items.retain(|x| !matches!(x, Item::Verbatim(_)));

    // remove forward-declared methods
    for x in ast.items.iter_mut() {
        if let Item::Impl(ItemImpl { items, .. } ) = x {
            items.retain(|x| !is_verbatim_method(x));
        }
    }

    // remove empty impls
    ast.items.retain(|item| match item {
        Item::Impl(ii) if ii.items.len() == 0 => false,
        _ => true,
    });

    ast
}
