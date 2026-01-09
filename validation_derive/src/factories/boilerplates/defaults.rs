use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

use crate::factories::boilerplates::modifications::{
	get_async_modification_boilerplate, get_async_modification_with_context_boilerplate, get_modification_boilerplate,
	get_modification_with_context_boilerplate,
};

pub fn get_default_factory_boilerplates(struct_name: &Ident) -> TokenStream {
	let method = quote! { self.validate() };
	let boilerplates = vec![
		get_default_with_context_boilerplate(struct_name, None, &method),
		get_async_default_boilerplate(struct_name, &method),
		get_async_default_with_context_boilerplate(struct_name, None, &method),
		get_modification_boilerplate(struct_name, &method),
		get_modification_with_context_boilerplate(struct_name, None, &method),
		get_async_modification_boilerplate(struct_name, &method),
		get_async_modification_with_context_boilerplate(struct_name, None, &method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_default_with_context_factory_boilerplates(struct_name: &Ident, context_type: &Type) -> TokenStream {
	let method = quote! { self.validate_with_context(context) };
	let boilerplates = vec![
		get_async_default_with_context_boilerplate(struct_name, Some(context_type), &method),
		get_modification_with_context_boilerplate(struct_name, Some(context_type), &method),
		get_async_modification_with_context_boilerplate(struct_name, Some(context_type), &method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_async_default_factory_boilerplates(struct_name: &Ident) -> TokenStream {
	let method = quote! { self.async_validate().await };
	let boilerplates = vec![
		get_async_default_with_context_boilerplate(struct_name, None, &method),
		get_async_modification_boilerplate(struct_name, &method),
		get_async_modification_with_context_boilerplate(struct_name, None, &method),
	];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_async_default_factory_with_context_boilerplates(struct_name: &Ident, context_type: &Type) -> TokenStream {
	let method = quote! { self.async_validate_with_context(context).await };
	let boilerplates = vec![get_async_modification_with_context_boilerplate(
		struct_name,
		Some(context_type),
		&method,
	)];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_default_with_context_boilerplate(
	struct_name: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
		  impl ValidateWithContext<#context_type> for #struct_name {
			  fn validate_with_context(&self, context: &#context_type) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    },
    None => quote! {
  		impl<C> ValidateWithContext<C> for #struct_name {
			  fn validate_with_context(&self, _: &C) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    }
	};

	result
}

pub fn get_async_default_boilerplate(struct_name: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
		#[async_trait]
	  impl AsyncValidate for #struct_name {
		  async fn async_validate(&self) -> Result<(), ValidationErrors> {
  		  #method
		  }
	  }
	};

	result
}

pub fn get_async_default_with_context_boilerplate(
	struct_name: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
			#[async_trait]
		  impl AsyncValidateWithContext<#context_type> for #struct_name {
			  async fn async_validate_with_context(&self, context: &#context_type) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    },
    None => quote! {
      #[async_trait]
		  impl<C> AsyncValidateWithContext<C> for #struct_name {
			  async fn async_validate_with_context(&self, _: &C) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    }
	};

	result
}
