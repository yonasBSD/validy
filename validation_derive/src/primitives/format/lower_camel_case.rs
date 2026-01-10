use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{ImportsSet, fields::FieldAttributes, imports::Import};

pub fn create_lower_camel_case(field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports.borrow_mut().add(Import::ModificationFunction(
		"cases::lower_camel_case as lower_camel_case_fn",
	));

	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();

	quote! {
		let mut #new_reference = lower_camel_case_fn(&#reference);
	}
}
