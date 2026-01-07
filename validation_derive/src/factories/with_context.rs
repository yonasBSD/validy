use crate::{
	Output, factories::core::AbstractValidationFactory, fields::FieldAttributes, import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct ValidationWithContextFactory<'a> {
	name: &'a Ident,
	context: &'a Type,
}

impl<'a> ValidationWithContextFactory<'a> {
	pub fn new(name: &'a Ident, context: &'a Type) -> Self {
		Self { name, context }
	}
}

impl<'a> AbstractValidationFactory for ValidationWithContextFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let operations = fields.iter_mut().flat_map(|field| field.get_operations());
		let async_trait_import = import_async_trait();
		let import = import_validation();

		let name = self.name;
		let context = self.context;

		quote! {
		  use #import;
		  use #async_trait_import;

		  impl ValidateWithContext<#context> for #name {
			  fn validate_with_context(&self, context: &#context) -> Result<(), ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();

				  #(#operations)*

				  if errors.is_empty() {
					  Ok(())
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

			#[async_trait]
		  impl AsyncValidateWithContext<#context> for #name
		  where
				#context: Send + Sync,
		  {
			  async fn async_validate_with_context(&self, context: &#context) -> Result<(), ValidationErrors> {
				  self.validate_with_context(context)
			  }
		  }

			#[async_trait]
		  impl AsyncValidateAndModificateWithContext<#context> for #name {
			  async fn async_validate_and_modificate_with_context(&mut self, context: &#context) -> Result<(), ValidationErrors> {
					self.validate_with_context(context)
				}
			}

			impl ValidateAndModificateWithContext<#context> for #name {
			  fn validate_and_modificate_with_context(&mut self, context: &#context) -> Result<(), ValidationErrors> {
					self.validate_with_context(context)
				}
			}
		}
		.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let context = &self.context;

		quote! {
		  if let Err(e) = <#field_type as ValidateWithContext<#context>>::validate_with_context(&#reference, &context) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
			}
		}
	}
}
