use crate::typerec::*;

// TODO warn whenever two variant-elements could be confused.

/// Wraps enum variant elements marked with `#[specr::indirection]`.
pub(in crate::typerec) fn wrap_variant_elements(mods: &mut [Module]) -> HashSet<VariantElement> {
    let mut elements = HashSet::new();

    for m in mods {
        for item in &mut m.ast.items {
            if let Item::Enum(it_enum) = item {
                elements.extend(wrap_enum(it_enum));
            }
        }
    }

    elements
}

fn is_indirection_attr(attr: &Attribute) -> bool {
    let s = format!("{}", attr.path.to_token_stream()).replace(" ", "");
    s == "specr::indirection"
}

fn wrap_enum(it_enum: &mut ItemEnum) -> HashSet<VariantElement> {
    let mut elements = HashSet::new();

    for variant in &mut it_enum.variants {
        let fields: Vec<&mut Field> = match &mut variant.fields {
            Fields::Named(x) => x.named.iter_mut().collect(),
            Fields::Unnamed(x) => x.unnamed.iter_mut().collect(),
            Fields::Unit => Vec::new(),
        };

        for (i, f) in fields.into_iter().enumerate() {
            if let Some(j) = f.attrs.iter().position(is_indirection_attr) {
                f.attrs.remove(j);

                let t = &f.ty;
                let wrapped_ty = quote! { specr::hidden::GcCow<#t> };
                f.ty = parse2(wrapped_ty).unwrap();

                let idx = match &f.ident {
                    Some(id) => ElementIdx::Named(id.clone()),
                    None => ElementIdx::Unnamed(i),
                };
                let variant = variant.ident.clone();
                elements.insert(VariantElement { variant, idx });
            }
            
        }
    }

    elements
}
