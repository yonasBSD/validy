use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub fn get_payload_factory_boilerplates(struct_name: &Ident, wrapper_ident: &Ident) -> TokenStream {
	let method = quote! { <#struct_name as ValidateAndParse<#wrapper_ident>>::validate_and_parse(___wrapper) };
	let boilerplates = vec![
		get_payload_with_context_boilerplate(struct_name, wrapper_ident, None, &method),
		get_async_payload_boilerplate(struct_name, wrapper_ident, &method),
		get_async_payload_with_context_boilerplate(struct_name, wrapper_ident, None, &method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_payload_with_context_boilerplate(
	struct_name: &Ident,
	wrapper_ident: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
   	  impl ValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
       	fn validate_and_parse_with_context(___wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
  			  #method
  		  }
  	  }
    },
    None => quote! {
  	  impl<C> ValidateAndParseWithContext<#wrapper_ident, C> for #struct_name where C: Send + Sync {
  			fn validate_and_parse_with_context(___wrapper: &#wrapper_ident, _: &C) -> Result<Self, ValidationErrors> {
  			  #method
  		  }
  	  }
    }
	};

	result
}

pub fn get_async_payload_boilerplate(struct_name: &Ident, wrapper_ident: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
    #[async_trait]
    impl AsyncValidateAndParse<#wrapper_ident> for #struct_name {
      async fn async_validate_and_parse(___wrapper: &#wrapper_ident) -> Result<Self, ValidationErrors> {
     			#method
      }
    }
	};

	result
}

pub fn get_async_payload_with_context_boilerplate(
	struct_name: &Ident,
	wrapper_ident: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
     	#[async_trait]
     	impl AsyncValidateAndParseWithContext<#wrapper_ident, #context_type> for #struct_name {
     	  async fn async_validate_and_parse_with_context(___wrapper: &#wrapper_ident, context: &#context_type) -> Result<Self, ValidationErrors> {
         	#method
     		}
      }
    },
    None => quote! {
     	#[async_trait]
     	impl<C> AsyncValidateAndParseWithContext<#wrapper_ident, C> for #struct_name where C: Send + Sync {
     	  async fn async_validate_and_parse_with_context(___wrapper: &#wrapper_ident, _: &C) -> Result<Self, ValidationErrors> {
         	#method
     		}
      }
    }
	};

	result
}
