use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	factories::{
		boilerplates::{commons::get_throw_errors_boilerplate, payloads::get_async_payload_factory_boilerplates},
		core::AbstractValidationFactory,
		utils::payloads::PayloadsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub struct AsyncPayloadFactory<'a> {
	struct_name: &'a Ident,
}

impl<'a> AsyncPayloadFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self { struct_name }
	}
}

impl<'a> AbstractValidationFactory for AsyncPayloadFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>, imports: &RefCell<ImportsSet>) -> Output {
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);
		imports.borrow_mut().add(Import::Deserialize);
		let imports = imports.borrow().build();
		let struct_name = self.struct_name;

		let mut code_factory = PayloadsCodeFactory(&mut fields);
		let (wrapper_struct, wrapper_ident) = code_factory.wrapper(struct_name);
		let operations = code_factory.operations();
		let commit = code_factory.commit();

		let boilerplates = get_async_payload_factory_boilerplates(struct_name, &wrapper_ident);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
				#imports

  			#wrapper_struct

  			#[async_trait]
  			impl AsyncValidateAndParse<#wrapper_ident> for #struct_name {
         	async fn async_validate_and_parse(___wrapper: &#wrapper_ident) -> Result<Self, ValidationErrors> {
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
   			impl SpecificAsyncValidateAndParse for #struct_name {
          type Wrapper = #wrapper_ident;
         	async fn specific_async_validate_and_parse(___wrapper: &#wrapper_ident) -> Result<Self, ValidationErrors> {
       			<#struct_name as AsyncValidateAndParse<#wrapper_ident>>::async_validate_and_parse(___wrapper).await
    		  }
   	    }

        #boilerplates
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

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as AsyncValidateAndParse<#wrapper_ident>>::async_validate_and_parse(&#new_reference, context).await {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
