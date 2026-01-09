use crate::{
	Output,
	factories::{
		boilerplates::{commons::get_throw_errors_boilerplate, defaults::get_default_factory_boilerplates},
		core::AbstractValidationFactory,
		utils::defaults::DefaultsCodeFactory,
	},
	fields::FieldAttributes,
	import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub struct ValidationFactory<'a> {
	struct_name: &'a Ident,
}

impl<'a> ValidationFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self { struct_name }
	}
}

impl<'a> AbstractValidationFactory for ValidationFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();
		let struct_name = self.struct_name;

		let mut code_factory = DefaultsCodeFactory(&mut fields);
		let operations = code_factory.operations();

		let boilerplates = get_default_factory_boilerplates(struct_name);
		let throw_errors = get_throw_errors_boilerplate();

		quote! {
		  use #async_trait_import;
		  use #import;

		  impl Validate for #struct_name {
			  fn validate(&self) -> Result<(), ValidationErrors> {
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
		}
		.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();

		quote! {
		  if let Err(e) = <#field_type as Validate>::validate(&#reference) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
