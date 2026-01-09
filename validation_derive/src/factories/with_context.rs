use crate::{
	Output,
	factories::{
		boilerplates::{
			commons::get_throw_errors_boilerplate, defaults::get_default_with_context_factory_boilerplates,
		},
		core::AbstractValidationFactory,
		utils::defaults::DefaultsCodeFactory,
	},
	fields::FieldAttributes,
	import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct ValidationWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> ValidationWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for ValidationWithContextFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();
		let struct_name = self.struct_name;
		let context_type = self.context_type;

		let mut code_factory = DefaultsCodeFactory(&mut fields);
		let operations = code_factory.operations();

		let boilerplates = get_default_with_context_factory_boilerplates(struct_name, context_type);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
  	let result = quote! {
      use #async_trait_import;
		  use #import;

		  impl ValidateWithContext<#context_type> for #struct_name {
			  fn validate_with_context(&self, context: &#context_type) -> Result<(), ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();

				  #(#operations)*

				  if errors.is_empty() {
					  Ok(())
				  } else {
						#throw_errors
				  }
			  }
		  }

			#boilerplates
		};

		result.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let context_type = &self.context_type;

		quote! {
		  if let Err(e) = <#field_type as ValidateWithContext<#context_type>>::validate_with_context(&#reference, &context) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
			}
		}
	}
}
