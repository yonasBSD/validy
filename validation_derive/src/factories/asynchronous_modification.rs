use crate::{
	Output,
	factories::{
		boilerplates::{
			commons::get_throw_errors_boilerplate, modifications::get_async_modification_factory_boilerplates,
		},
		core::AbstractValidationFactory,
		utils::modifications::ModificationsCodeFactory,
	},
	fields::FieldAttributes,
	import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub struct AsyncModificationFactory<'a> {
	struct_name: &'a Ident,
}

impl<'a> AsyncModificationFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self { struct_name }
	}
}

impl<'a> AbstractValidationFactory for AsyncModificationFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();
		let struct_name = self.struct_name;

		let mut code_factory = ModificationsCodeFactory(&mut fields);
		let operations = code_factory.operations();
		let commit = code_factory.commit();

		let boilerplates = get_async_modification_factory_boilerplates(struct_name);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  use #import;
		  use #async_trait_import;

			#[async_trait]
		  impl AsyncValidateAndModificate for #struct_name {
			  async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
					let mut errors = Vec::<ValidationError>::new();

				  #(#operations)*

				  if errors.is_empty() {
						#commit
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
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as AsyncValidateAndModificate>::async_validate_and_modificate(&mut #new_reference).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
