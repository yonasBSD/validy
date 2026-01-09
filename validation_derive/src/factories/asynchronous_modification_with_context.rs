use crate::{
	Output,
	factories::{
		boilerplates::commons::get_throw_errors_boilerplate, core::AbstractValidationFactory,
		utils::modifications::ModificationsCodeFactory,
	},
	fields::FieldAttributes,
	import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct AsyncModificationWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> AsyncModificationWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncModificationWithContextFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();
		let struct_name = self.struct_name;
		let context_type = self.context_type;

		let mut code_factory = ModificationsCodeFactory(&mut fields);
		let operations = code_factory.operations();
		let commit = code_factory.commit();

		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  use #import;
		  use #async_trait_import;

			#[async_trait]
		  impl AsyncValidateAndModificateWithContext<#context_type> for #struct_name {
			  async fn async_validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();

				  #(#operations)*

				  if errors.is_empty() {
						#commit
				  } else {
						#throw_errors
				  }
			  }
		  }
		};

		result.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let context_type = &self.context_type;

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as AsyncValidateAndModificateWithContext<#context_type>>::async_validate_and_modificate_with_context(&mut #new_reference, context).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
