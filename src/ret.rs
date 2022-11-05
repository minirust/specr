use syn::*;
use syn::parse::Parse;
use syn::visit_mut::*;
use quote::ToTokens;

pub fn add_ret(mut ast: syn::File) -> syn::File {
	let mut v = Visitor;
	v.visit_file_mut(&mut ast);

	ast
}

struct Visitor;

fn parse<T: Parse>(s: &str) -> T {
	syn::parse_str::<T>(s).unwrap()
}

fn empty_ret() -> Expr {
	parse::<Expr>("baselib::ret(())")
}

fn wrap_ret_expr(expr: &mut Expr) {
	let s = format!("baselib::ret({})", expr.to_token_stream());
	*expr = parse::<Expr>(&s);
}

fn visit_block(b: &mut Block) {
	if let Some(Stmt::Expr(expr)) = b.stmts.last_mut() {
		wrap_ret_expr(expr);
	} else {
		let stmt = Stmt::Expr(empty_ret());
		b.stmts.push(stmt); 
	}
}

impl VisitMut for Visitor {
	fn visit_expr_return_mut(&mut self, node: &mut ExprReturn) {
		match &mut node.expr {
			Some(expr) => wrap_ret_expr(expr),
			None => { node.expr = Some(Box::new(empty_ret())); },
		}
		visit_expr_return_mut(self, node);
	}

	fn visit_item_fn_mut(&mut self, f: &mut ItemFn) {
		visit_block(&mut f.block);
		visit_item_fn_mut(self, f);
	}

	fn visit_impl_item_method_mut(&mut self, iim: &mut ImplItemMethod) {
		visit_block(&mut iim.block);
		visit_impl_item_method_mut(self, iim);
	}
}
