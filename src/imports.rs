use crate::prelude::*;

pub fn add_imports(mut mods: Vec<Module>) -> Vec<Module> {
    let imports: Vec<Ident> = mods.iter()
                                  .map(|m| format_ident!("{}", &m.name))
                                  .collect();

    let code = quote! {
        use crate::{ #(#imports),* };
        use crate::specr::prelude::*;
        use crate::specr;
    };
    let f: syn::File = parse2(code).unwrap();

    let prelude_code = quote! { use crate::prelude::*; };
    let prelude_item: Item = parse2(prelude_code).unwrap();

    // add imports within module
    for m in mods.iter_mut() {
        m.ast.items.extend(f.items.clone());

        if m.name != "prelude" {
            m.ast.items.push(prelude_item.clone());
        }
    }

    mods
}
