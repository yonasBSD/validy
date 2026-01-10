use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{ImportsSet, fields::FieldAttributes, imports::Import};

pub fn create_snake_case(field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports
		.borrow_mut()
		.add(Import::ModificationFunction("cases::snake_case as snake_case_fn"));

	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();

	quote! {
		let mut #new_reference = snake_case_fn(&#reference);
	}
}
