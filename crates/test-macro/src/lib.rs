#![doc = include_str!("../README.md")]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn aidoku_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut item = parse_macro_input!(item as syn::ItemFn);
	let name = item.sig.ident.to_string();

	// inject panic hook at the top of the function body
	// this way, we can see panic info when the test fails
	item.block
		.stmts
		.insert(0, parse_quote! { extern crate std; });
	item.block.stmts.insert(
		1,
		parse_quote! {
			std::panic::set_hook(::aidoku::alloc::Box::new(|info| {
				::aidoku::prelude::println!("{info}");
			}));
		},
	);

	// if the function should be ignored, add it to the export name
	let mut ignore = "";
	if let Some(i) = item.attrs.iter().position(is_ignore) {
		item.attrs.remove(i);
		ignore = "ignore$"
	}

	// create a custom export name so we can read the exports in the test runner
	let res = quote! {
		#[cfg(test)]
		#[export_name = concat!("$aidoku-test$", #ignore, module_path!(), "::",  #name)]
		#item
	};
	res.into()
}

fn is_ignore(attr: &syn::Attribute) -> bool {
	attr.path().is_ident("ignore")
}
