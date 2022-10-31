use syn::*;
use quote::ToTokens;

pub fn merge(mut ast: syn::File) -> syn::File {
	let mut ii_list: Vec<&mut ItemImpl> = 
		ast.items.iter_mut()
			.filter_map(|i|
				if let Item::Impl(ii) = i {
					Some(ii)
				} else { None }
			).collect();
	let n = ii_list.len();

	for i in 0..n {
		for j in (i+1)..n {
			if belong_together(&*ii_list[i], &*ii_list[j]) {
				let tmp = ii_list[j].items.split_off(0); // remove all items from ii_list[j]
				ii_list[i].items.extend(tmp);
			}
		}
	}

	ast
}

fn belong_together(ii1: &ItemImpl, ii2: &ItemImpl) -> bool {
	let to_str = |ii: &ItemImpl| {
		let mut ii = ii.clone();
		ii.attrs.clear();
		ii.items.clear();

		ii.to_token_stream()
		  .to_string()
	};

	to_str(ii1) == to_str(ii2)
}
