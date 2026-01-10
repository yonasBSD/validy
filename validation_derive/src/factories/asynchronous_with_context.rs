use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	factories::{
		boilerplates::{
			commons::get_throw_errors_boilerplate, defaults::get_async_default_factory_with_context_boilerplates,
		},
		core::AbstractValidationFactory,
		utils::defaults::DefaultsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct AsyncValidationWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> AsyncValidationWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncValidationWithContextFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>, imports: &RefCell<ImportsSet>) -> Output {
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);
		let imports = imports.borrow().build();
		let struct_name = self.struct_name;
		let context_type = self.context_type;

		let mut code_factory = DefaultsCodeFactory(&mut fields);
		let operations = code_factory.operations();

		let boilerplates = get_async_default_factory_with_context_boilerplates(struct_name, context_type);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#[async_trait]
  		  impl AsyncValidateWithContext<#context_type> for #struct_name {
  			  async fn async_validate_with_context(&self, context: &#context_type) -> Result<(), ValidationErrors> {
  					let mut errors = Vec::<ValidationError>::new();

  				  #(#operations)*

  				  if errors.is_empty() {
  					  Ok(())
  				  } else {
  						#throw_errors
  				  }
  			  }
  		  }

   			#[async_trait]
   		  impl SpecificAsyncValidateWithContext for #struct_name {
          type Context = #context_type;
   			  async fn specific_async_validate_with_context(&self, context: &#context_type) -> Result<(), ValidationErrors> {
            <#struct_name as AsyncValidateWithContext<#context_type>>::async_validate_with_context(self, context).await
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
		let context_type = self.context_type;

		quote! {
		  if let Err(e) = <#field_type as AsyncValidateWithContext<#context_type>>::async_validate_with_context(&#reference, &context).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
			}
		}
	}
}
