use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{ImportsSet, fields::FieldAttributes, imports::Import};

pub fn create_camel_case(field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports
		.borrow_mut()
		.add(Import::ModificationFunction("cases::camel_case as camel_case_fn"));

	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();

	quote! {
		let mut #new_reference = camel_case_fn(&#reference);
	}
}
