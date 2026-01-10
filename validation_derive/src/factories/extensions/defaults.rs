use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::{
	ImportsSet,
	factories::extensions::axum::defaults::{
		get_async_default_axum_extension, get_async_default_with_context_axum_extension,
	},
};

pub fn get_default_extensions(struct_name: &Ident, _: &RefCell<ImportsSet>) -> TokenStream {
	let extensions = vec![get_async_default_axum_extension(struct_name)];
	quote! { #(#extensions)* }
}

pub fn get_default_with_context_extensions(struct_name: &Ident, _: &Type, _: &RefCell<ImportsSet>) -> TokenStream {
	let extensions = vec![get_async_default_with_context_axum_extension(struct_name)];
	quote! { #(#extensions)* }
}

pub fn get_async_default_extensions(struct_name: &Ident, _: &RefCell<ImportsSet>) -> TokenStream {
	let extensions = vec![get_async_default_axum_extension(struct_name)];
	quote! { #(#extensions)* }
}

pub fn get_async_default_with_context_extensions(
	struct_name: &Ident,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let extensions = vec![get_async_default_with_context_axum_extension(struct_name)];
	quote! { #(#extensions)* }
}
