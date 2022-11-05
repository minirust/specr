use syn::*;

fn is_verbatim(item: &ImplItem) -> bool{
    if let ImplItem::Method(iim) = item {
        iim.block.stmts.iter()
            .any(|x| matches!(x, Stmt::Item(Item::Verbatim(_))))
    } else { false }
}

pub fn clear_verbatim(mut ast: syn::File) -> syn::File {
    ast.items.retain(|x| !matches!(x, Item::Verbatim(_)));

    for x in ast.items.iter_mut() {
        if let Item::Impl(ItemImpl { items, .. } ) = x {
            items.retain(|x| !is_verbatim(x));
        }
    }

    ast
}

pub fn clear_empty_impls(mut ast: syn::File) -> syn::File {
    ast.items.retain(|item| match item {
        Item::Impl(ii) if ii.items.len() == 0 => false,
        _ => true,
    });

    ast
}
