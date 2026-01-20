use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::parse_str;

pub struct ImportsSet {
	set: HashSet<Import>,
}

impl ImportsSet {
	pub fn new() -> Self {
		ImportsSet { set: HashSet::new() }
	}

	pub fn add(&mut self, import: Import) {
		self.set.insert(import);
	}

	pub fn build(&self) -> TokenStream {
		let imports: Vec<TokenStream> = self
			.set
			.iter()
			.map(|import| {
				let import = match import {
					Import::ValidationFunction(function) => import_validation_functions(function),
					Import::ModificationFunction(function) => import_modification_functions(function),
					Import::ValidationCore => import_validation(),
					Import::AsyncTrait => import_async_trait(),
				};

				quote! { use #import; }
			})
			.collect();

		quote! { #(#imports)* }
	}
}

#[derive(PartialEq, Eq, Hash)]
pub enum Import {
	ValidationFunction(&'static str),
	ModificationFunction(&'static str),
	ValidationCore,
	AsyncTrait,
}

fn import_validation_functions(function: &str) -> TokenStream {
	let found_crate = crate_name("validy").expect("validation is present in `Cargo.toml`");
	let function_tokens: TokenStream = parse_str(function).expect("invalid validation path");

	match found_crate {
		FoundCrate::Itself => quote!(crate::functions::validy::#function_tokens),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::functions::validy::#function_tokens)
		}
	}
}

fn import_modification_functions(function: &str) -> TokenStream {
	let found_crate = crate_name("validy").expect("validation is present in `Cargo.toml`");
	let function_tokens: TokenStream = parse_str(function).expect("invalid validation path");

	match found_crate {
		FoundCrate::Itself => quote!(crate::functions::modification::#function_tokens),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::functions::modification::#function_tokens)
		}
	}
}

fn import_validation() -> TokenStream {
	let found_crate = crate_name("validy").expect("validation is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::core::*),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::core::*)
		}
	}
}

fn import_async_trait() -> TokenStream {
	let found_crate = crate_name("async-trait").expect("async-trait is present in `Cargo.toml`");

	match found_crate {
		FoundCrate::Itself => quote!(crate::async_trait),
		FoundCrate::Name(name) => {
			let ident = Ident::new(&name, Span::call_site());
			quote!(#ident::async_trait)
		}
	}
}
