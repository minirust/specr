use syn::visit_mut::VisitMut;
use syn::*;

struct PubVisitor;

pub fn access(mut ast: syn::File) -> syn::File{
    PubVisitor.visit_file_mut(&mut ast);

    ast
}

fn pub_fields(f: &mut Fields) {
    match f {
        Fields::Named(FieldsNamed { named, .. }) => {
            for x in named.iter_mut() { x.vis = pub_vis(); }
        },
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            for x in unnamed.iter_mut() { x.vis = pub_vis(); }
        },
        Fields::Unit => {},
    }
}

impl VisitMut for PubVisitor {
    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        node.vis = pub_vis();
        pub_fields(&mut node.fields);
    }

    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) { node.vis = pub_vis(); }
    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) { node.vis = pub_vis(); }
    fn visit_item_type_mut(&mut self, node: &mut ItemType) { node.vis = pub_vis(); }
    fn visit_item_trait_mut(&mut self, node: &mut ItemTrait) { node.vis = pub_vis(); }
    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        if node.trait_.is_none() {
            for i in &mut node.items {
                match i {
                    ImplItem::Const(x) => { x.vis = pub_vis(); },
                    ImplItem::Method(x) => { x.vis = pub_vis(); },
                    ImplItem::Type(x) => { x.vis = pub_vis(); },
                    _ => {},
                }
            }
        }
    }
}

fn pub_vis() -> Visibility {
    use syn::token::Pub;

    let pub_token = Pub::default();
    VisPublic { pub_token }.into()
}
