use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	factories::{
		boilerplates::commons::get_throw_errors_boilerplate, core::AbstractValidationFactory,
		extensions::payloads::get_async_payload_with_context_extensions, utils::payloads::PayloadsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub struct AsyncPayloadWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> AsyncPayloadWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for AsyncPayloadWithContextFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>, imports: &RefCell<ImportsSet>) -> Output {
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);
		imports.borrow_mut().add(Import::Deserialize);

		let struct_name = self.struct_name;
		let context_type = self.context_type;

		let mut code_factory = PayloadsCodeFactory(&mut fields);
		let (wrapper_struct, wrapper_ident) = code_factory.wrapper(struct_name);
		let extensions =
			get_async_payload_with_context_extensions(self.struct_name, &wrapper_ident, self.context_type, imports);

		let operations = code_factory.operations();
		let commit = code_factory.commit();
		let imports = imports.borrow().build();

		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#wrapper_struct

  			#[async_trait]
  			impl AsyncValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
         	async fn async_validate_and_parse_with_context(___wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
       			let mut errors = Vec::<ValidationError>::new();

            #(#operations)*

            if errors.is_empty() {
              #commit
            } else {
             	#throw_errors
            }
    		  }
   	    }

        #[async_trait]
   			impl SpecificAsyncValidateAndParseWithContext for #struct_name {
          type Wrapper = #wrapper_ident;
          type Context = #context_type;
         	async fn specific_async_validate_and_parse_with_context(___wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
       			<#struct_name as AsyncValidateAndParseWithContext<#wrapper_ident, #context_type>>::async_validate_and_parse_with_context(___wrapper, context).await
    		  }
   	    }

        #extensions
			};
		};

		result.into()
	}

	fn create_nested(&self, field: &mut FieldAttributes) -> TokenStream {
		let reference = field.get_reference();
		field.increment_modifications();
		let new_reference = field.get_reference();
		let field_name = field.get_name();
		let field_type = field.get_type();
		let wrapper_ident = format_ident!("{}Wrapper", self.struct_name);
		let context_type = self.context_type;

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as AsyncValidateAndParseWithContext<#wrapper_ident, #context_type>>::async_validate_and_parse_with_context(&#new_reference, context).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
