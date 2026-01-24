use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::{
	ImportsSet,
	attributes::ValidationAttributes,
	factories::extensions::axum::defaults::{
		get_async_default_axum_extension, get_async_default_axum_multipart_extension,
		get_async_default_with_context_axum_extension, get_async_default_with_context_axum_multipart_extension,
	},
};

pub fn get_default_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let mut extensions = vec![];

	match (
		attributes.axum,
		attributes.multipart,
		cfg!(feature = "axum"),
		cfg!(feature = "axum_multipart"),
	) {
		(true, false, true, _) => extensions.push(get_async_default_axum_extension(struct_name)),
		(true, true, true, true) => extensions.push(get_async_default_axum_multipart_extension(struct_name)),
		_ => {}
	}

	quote! { #(#extensions)* }
}

pub fn get_default_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let mut extensions = vec![];

	match (
		attributes.axum,
		attributes.multipart,
		cfg!(feature = "axum"),
		cfg!(feature = "axum_multipart"),
	) {
		(true, false, true, _) => extensions.push(get_async_default_with_context_axum_extension(struct_name)),
		(true, true, true, true) => {
			extensions.push(get_async_default_with_context_axum_multipart_extension(struct_name))
		}
		_ => {}
	}

	quote! { #(#extensions)* }
}

pub fn get_async_default_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let mut extensions = vec![];

	match (
		attributes.axum,
		attributes.multipart,
		cfg!(feature = "axum"),
		cfg!(feature = "axum_multipart"),
	) {
		(true, false, true, _) => extensions.push(get_async_default_axum_extension(struct_name)),
		(true, true, true, true) => extensions.push(get_async_default_axum_multipart_extension(struct_name)),
		_ => {}
	}

	quote! { #(#extensions)* }
}

pub fn get_async_default_with_context_extensions(
	struct_name: &Ident,
	attributes: &ValidationAttributes,
	_: &Type,
	_: &RefCell<ImportsSet>,
) -> TokenStream {
	let mut extensions = vec![];

	match (
		attributes.axum,
		attributes.multipart,
		cfg!(feature = "axum"),
		cfg!(feature = "axum_multipart"),
	) {
		(true, false, true, _) => extensions.push(get_async_default_with_context_axum_extension(struct_name)),
		(true, true, true, true) => {
			extensions.push(get_async_default_with_context_axum_multipart_extension(struct_name))
		}
		_ => {}
	}

	quote! { #(#extensions)* }
}
