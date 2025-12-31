use proc_macro2::TokenStream;
use quote::quote;

use crate::{fields::FieldAttributes, imports::import_modification_functions};

pub fn create_lower_camel_case(field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();
	let import = import_modification_functions("cases::lower_camel_case");

	quote! {
	  use #import;
		let mut #new_reference = lower_camel_case(&#reference);
	}
}
