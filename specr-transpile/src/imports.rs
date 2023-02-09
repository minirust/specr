use crate::prelude::*;

/// Prepends implicit imports to each module.
pub fn add_imports(mut mods: Vec<Module>) -> Vec<Module> {
    let imports: Vec<Ident> = mods.iter()
                                  .map(|m| format_ident!("{}", &m.name))
                                  .collect();

    let code = quote! {
        #[allow(unused_imports)]
        use crate::{ #(#imports),* };

        // this is currently required to get the #[derive(GcCompat)] to work.
        // TODO fix.
        #[allow(unused_imports)]
        use libspecr::hidden::GcCompat;
    };
    let f: syn::File = parse2(code).unwrap();

    let prelude_code = quote! { use crate::prelude::*; };
    let prelude_item: Item = parse2(prelude_code).unwrap();

    // add imports within module
    for m in mods.iter_mut() {
        let mut items = f.items.clone();
        if m.name != "prelude" {
            items.push(prelude_item.clone());
        }

        for (i, it) in items.into_iter().enumerate() {
            m.ast.items.insert(i, it);
        }

    }

    mods
}
