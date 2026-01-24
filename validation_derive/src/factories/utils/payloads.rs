use crate::fields::FieldAttributes;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

pub struct PayloadsCodeFactory<'a>(pub &'a mut Vec<FieldAttributes>);

impl<'a> PayloadsCodeFactory<'a> {
	pub fn wrapper(&self, name: &'a Ident) -> (TokenStream, Ident) {
		let wrapper_ident = format_ident!("{}Wrapper", name);
		let field_declarations: Vec<TokenStream> = self
			.0
			.iter()
			.clone()
			.map(|field| {
				let name = field.get_name();
				let field_type = field.get_initial_type();
				let field_name = Ident::new(&name.value(), Span::call_site());

				quote! {
				  pub #field_name: #field_type,
				}
			})
			.collect();

		#[rustfmt::skip]
		let wrapper_struct = quote! {
  		#[derive(Debug, Default, ::serde::Deserialize)]
  		pub struct #wrapper_ident {
  		  #(#field_declarations)*
  		}
		};

		(wrapper_struct, wrapper_ident)
	}

	pub fn operations(&mut self) -> Vec<TokenStream> {
		self.0.iter_mut().map(|field| field.get_operations()).collect()
	}

	pub fn commit(&self) -> TokenStream {
		let commits: Vec<TokenStream> = self
			.0
			.iter()
			.clone()
			.map(|field| {
				let reference = field.get_reference();
				let name = field.get_name();
				let field_name = Ident::new(&name.value(), Span::call_site());

				if field.is_option() {
					quote! {
					  #field_name: #reference,
					}
				} else {
					#[rustfmt::skip]
					let result = quote! {
						#field_name: #reference.ok_or_else(|| {
						  let error = ValidationError::builder()
							  .with_field(#name)
							  .as_simple("unreachable")
							  .with_message("field missing after successful required validation check")
							  .build();

							let mut errors = ValidationErrors::new();
							append_error(&mut errors, error.into(), failure_mode, #name);

							errors
						})?,
					};

					result
				}
			})
			.collect();

		#[rustfmt::skip]
		let commit = quote! {
      Ok(Self { #(#commits)* })
		};

		commit
	}
}
