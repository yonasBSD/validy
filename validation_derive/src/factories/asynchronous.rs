use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	factories::{
		boilerplates::{commons::get_throw_errors_boilerplate, defaults::get_async_default_factory_boilerplates},
		core::AbstractValidationFactory,
		utils::defaults::DefaultsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub struct AsyncValidationFactory<'a> {
	struct_name: &'a Ident,
}

impl<'a> AsyncValidationFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self { struct_name }
	}
}

impl<'a> AbstractValidationFactory for AsyncValidationFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>, imports: &RefCell<ImportsSet>) -> Output {
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);
		let imports = imports.borrow().build();
		let struct_name = self.struct_name;

		let mut code_factory = DefaultsCodeFactory(&mut fields);
		let operations = code_factory.operations();

		let boilerplates = get_async_default_factory_boilerplates(struct_name);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#[async_trait]
  		  impl AsyncValidate for #struct_name {
  			  async fn async_validate(&self) -> Result<(), ValidationErrors> {
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
		};

		result.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();

		quote! {
		  if let Err(e) = <#field_type as AsyncValidate>::validate(&#reference).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
