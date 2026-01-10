use std::cell::RefCell;

use crate::{
	ImportsSet, Output,
	factories::{
		boilerplates::{
			commons::get_throw_errors_boilerplate, payloads::get_payload_with_context_factory_boilerplates,
		},
		core::AbstractValidationFactory,
		utils::payloads::PayloadsCodeFactory,
	},
	fields::FieldAttributes,
	imports::Import,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

pub struct PayloadWithContextFactory<'a> {
	struct_name: &'a Ident,
	context_type: &'a Type,
}

impl<'a> PayloadWithContextFactory<'a> {
	pub fn new(struct_name: &'a Ident, context_type: &'a Type) -> Self {
		Self {
			struct_name,
			context_type,
		}
	}
}

impl<'a> AbstractValidationFactory for PayloadWithContextFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>, imports: &RefCell<ImportsSet>) -> Output {
		imports.borrow_mut().add(Import::ValidationCore);
		imports.borrow_mut().add(Import::AsyncTrait);
		imports.borrow_mut().add(Import::Deserialize);
		let imports = imports.borrow().build();
		let struct_name = self.struct_name;
		let context_type = self.context_type;

		let mut code_factory = PayloadsCodeFactory(&mut fields);
		let (wrapper_struct, wrapper_ident) = code_factory.wrapper(struct_name);
		let operations = code_factory.operations();
		let commit = code_factory.commit();

		let boilerplates = get_payload_with_context_factory_boilerplates(struct_name, &wrapper_ident, context_type);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  const _: () = {
  		  #imports

  			#wrapper_struct

  			impl ValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
         	fn validate_and_parse_with_context(___wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
       			let mut errors = Vec::<ValidationError>::new();

            #(#operations)*

            if errors.is_empty() {
              #commit
            } else {
             	#throw_errors
            }
    		  }
   	    }

        impl SpecificValidateAndParseWithContext for #struct_name {
          type Wrapper = #wrapper_ident;
          type Context = #context_type;
     			fn specific_validate_and_parse_with_context(___wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
  					<#struct_name as ValidateAndParseWithContext<#wrapper_ident, #context_type>>::validate_and_parse_with_context(___wrapper, context)
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
		let context_type = self.context_type;

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as ValidateAndParseWithContext<#wrapper_ident, #context_type>>::validate_and_parse_with_context(&#new_reference, context) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
