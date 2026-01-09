use crate::{
	Output,
	factories::{
		boilerplates::{commons::get_throw_errors_boilerplate, payloads::get_payload_factory_boilerplates},
		core::AbstractValidationFactory,
		utils::payloads::PayloadsCodeFactory,
	},
	fields::FieldAttributes,
	import_async_trait, import_validation,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

pub struct PayloadFactory<'a> {
	struct_name: &'a Ident,
}

impl<'a> PayloadFactory<'a> {
	pub fn new(struct_name: &'a Ident) -> Self {
		Self { struct_name }
	}
}

impl<'a> AbstractValidationFactory for PayloadFactory<'a> {
	fn create(&self, mut fields: Vec<FieldAttributes>) -> Output {
		let async_trait_import = import_async_trait();
		let import = import_validation();
		let struct_name = self.struct_name;

		let mut code_factory = PayloadsCodeFactory(&mut fields);
		let (wrapper_struct, wrapper_ident) = code_factory.wrapper(struct_name);
		let operations = code_factory.operations();
		let commit = code_factory.commit();

		let boilerplates = get_payload_factory_boilerplates(struct_name, &wrapper_ident);
		let throw_errors = get_throw_errors_boilerplate();

		#[rustfmt::skip]
		let result = quote! {
		  use #async_trait_import;
		  use #import;

			#wrapper_struct

      impl ValidateAndParse<#wrapper_ident> for #struct_name {
        fn validate_and_parse(___wrapper: &#wrapper_ident) -> Result<Self, ValidationErrors> {
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
		let wrapper_ident = format_ident!("{}Wrapper", field_type.to_token_stream().to_string());

		quote! {
		  let mut #new_reference = #reference.clone();
		  if let Err(e) = <#field_type as ValidateAndParse<#wrapper_ident>>::validate_and_parse(&#new_reference) {
				errors.push(ValidationError::Node(NestedValidationError::from(
					e,
					#field_name,
				)));
		  }
		}
	}
}
