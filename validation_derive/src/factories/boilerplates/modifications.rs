use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub fn get_modification_factory_boilerplates(struct_name: &Ident) -> TokenStream {
	let method = quote! { self.validate_and_modificate() };
	let boilerplates = vec![
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

pub fn get_modification_with_context_factory_boilerplates(struct_name: &Ident, context_type: &Type) -> TokenStream {
	let method = quote! { self.validate_and_modificate_with_context(context) };
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

pub fn get_async_modification_factory_boilerplates(struct_name: &Ident) -> TokenStream {
	let method = quote! { self.async_validate_and_modificate().await };
	let boilerplates = vec![get_async_modification_with_context_boilerplate(
		struct_name,
		None,
		&method,
	)];

	#[rustfmt::skip]
	let result = quote! {
	  #(#boilerplates)*
	};

	result
}

pub fn get_modification_with_context_boilerplate(
	struct_name: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
   	  impl ValidateAndModificateWithContext<#context_type> for #struct_name {
        fn validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    },
    None => quote! {
      impl<C> ValidateAndModificateWithContext<C> for #struct_name {
			  fn validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    }
	};

	result
}

pub fn get_modification_boilerplate(struct_name: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
		impl ValidateAndModificate for #struct_name {
		  fn validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
				#method
			}
	  }
	};

	result
}

pub fn get_async_modification_boilerplate(struct_name: &Ident, method: &TokenStream) -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
		#[async_trait]
	  impl AsyncValidateAndModificate for #struct_name {
		  async fn async_validate_and_modificate(&mut self) -> Result<(), ValidationErrors> {
		    #method
		  }
	  }
	};

	result
}

pub fn get_async_modification_with_context_boilerplate(
	struct_name: &Ident,
	context_type: Option<&Type>,
	method: &TokenStream,
) -> TokenStream {
	#[rustfmt::skip]
	let result = match context_type {
    Some(context_type) => quote! {
  		#[async_trait]
		  impl AsyncValidateAndModificateWithContext<#context_type> for #struct_name {
		  async fn async_validate_and_modificate_with_context(&mut self, context: &#context_type) -> Result<(), ValidationErrors> {
				  #method
			  }
		  }
    },
    None => quote! {
      #[async_trait]
		  impl<C> AsyncValidateAndModificateWithContext<C> for #struct_name {
			  async fn async_validate_and_modificate_with_context(&mut self, _: &C) -> Result<(), ValidationErrors> {
			    #method
			  }
		  }
    }
	};

	result
}
