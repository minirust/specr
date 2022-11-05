use crate::typerec::*;

/// Returns all type names occuring inside of an enum.
fn enum_types(it_enum: &ItemEnum) -> HashSet<String> {
    let mut types = HashSet::new();

    for variant in &it_enum.variants {
        let fields: Vec<&Field> = match &variant.fields {
            Fields::Named(x) => x.named.iter().collect(),
            Fields::Unnamed(x) => x.unnamed.iter().collect(),
            Fields::Unit => Vec::new(),
        };

        for f in fields {
            let ty_str = format!("{}", f.ty.to_token_stream());
            types.insert(ty_str);
        }
    }

    types
}

/// Returns the names of all enums defined in the given modules.
fn enum_names(mods: &[syn::File]) -> HashSet<String> {
    let mut names = HashSet::new();
    for m in mods {
        for item in &m.items {
            if let Item::Enum(it_enum) = item {
                let n = format!("{}", it_enum.ident);
                names.insert(n);
            }
        }
    }

    names
}

/// Returns the subset of enums which contain other enums, and hence having potentially(!) infinite size.
pub fn inf_size_enums(mods: &[syn::File]) -> HashSet<String> {
    let names = enum_names(mods);

    let mut cands = HashSet::new();
    for m in mods {
        for item in &m.items {
            if let Item::Enum(it_enum) = item {
                let types = enum_types(it_enum);
                if !types.is_disjoint(&names) {
                    let n = format!("{}", it_enum.ident);
                    cands.insert(n);
                }
            }
        }
    }

    cands
}
