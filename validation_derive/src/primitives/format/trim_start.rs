use proc_macro2::TokenStream;
use quote::quote;

use crate::fields::FieldAttributes;

pub fn create_trim_start(field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();

	quote! {
		let mut #new_reference = #reference.trim_start();
	}
}
