use std::cell::RefCell;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{ImportsSet, fields::FieldAttributes, imports::Import};

pub fn create_shouty_kebab_case(field: &mut FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports.borrow_mut().add(Import::ModificationFunction(
		"cases::shouty_kebab_case as shouty_kebab_case_fn",
	));

	let reference = field.get_reference();
	field.increment_modifications();
	let new_reference = field.get_reference();

	if field.is_ref() {
		field.set_is_ref(false);
		quote! {
			let mut #new_reference = shouty_kebab_case_fn(#reference);
		}
	} else {
		field.set_is_ref(false);
		quote! {
			let mut #new_reference = shouty_kebab_case_fn(&#reference);
		}
	}
}
