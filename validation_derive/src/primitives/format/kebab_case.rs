use proc_macro2::TokenStream;
use quote::quote;

use crate::{fields::FieldAttributes, imports::import_modification_functions};

pub fn create_kebab_case(field: &mut FieldAttributes) -> TokenStream {
	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();
	let import = import_modification_functions("cases::kebab_case");

	quote! {
	  use #import;
		let mut #new_reference = kebab_case(&#reference);
	}
}
