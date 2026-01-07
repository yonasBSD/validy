use crate::{
	Output, factories::core::AbstractValidationFactory, fields::FieldAttributes, import_async_trait, import_validation,
	imports::import_serde_deserialize,
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

pub struct PayloadFactory<'a> {
	name: &'a Ident,
}

impl<'a> PayloadFactory<'a> {
	pub fn new(name: &'a Ident) -> Self {
		Self { name }
	}
}

impl<'a> AbstractValidationFactory for PayloadFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = self.name;

		let field_declarations: Vec<TokenStream> = fields
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

		let commits: Vec<TokenStream> = fields
			.iter()
			.clone()
			.filter(|field| field.get_modifications() > 0)
			.map(|field| {
				let reference = field.get_reference();
				let name = field.get_name();
				let field_name = Ident::new(&name.value(), Span::call_site());

				quote! {
				  #field_name: #reference,
				}
			})
			.collect();

		let operations = fields.iter_mut().flat_map(|field| field.get_operations());
		let wrapper_ident = format_ident!("{}Wrapper", name);

		quote! {
		  use #import;
		  use #async_trait_import;

			#[derive(Deserialize)]
			struct #wrapper_ident {
			  #(#field_declarations)*
			}

		  impl ValidateAndParse<#wrapper_ident> for #name {
			  fn validate_and_parse(___wrapper: #wrapper_ident) -> Result<Self, ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();

				  #(#operations)*

				  if errors.is_empty() {
					  Ok(#name { #(#commits)* })
				  } else {
						let map: ValidationErrors = errors
							.into_iter()
							.map(|e| match e {
								ValidationError::Node(e) => (e.field.clone(), ValidationError::Node(e)),
								ValidationError::Leaf(e) => (e.field.clone(), ValidationError::Leaf(e)),
							})
							.collect();

					  Err(map)
				  }
			  }
		  }

		 //  impl<C> ValidateAndModificateWithContext<C> for #name {
			//   fn validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
			// 	  self.validate_and_modificate()
			//   }
		 //  }

			// #[async_trait]
		 //  impl AsyncValidateAndModificate for #name {
			//   async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
			// 	  self.validate_and_modificate()
			//   }
		 //  }

			// #[async_trait]
		 //  impl<C> AsyncValidateAndModificateWithContext<C> for #name {
			//   async fn async_validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
			// 	  self.validate_and_modificate()
			//   }
		 //  }
		}
		.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as ValidateAndModificate>::validate_and_modificate(&mut #new_reference) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
