use proc_macro2::TokenStream;
use quote::quote;

pub fn get_throw_errors_boilerplate() -> TokenStream {
	#[rustfmt::skip]
	let result = quote! {
	  let map: ValidationErrors = errors
		  .into_iter()
		  .map(|e| match e {
			  ValidationError::Node(e) => (e.field.clone(), ValidationError::Node(e)),
			  ValidationError::Leaf(e) => (e.field.clone(), ValidationError::Leaf(e)),
		  })
		  .collect();

		Err(map)
	};

	result
}
