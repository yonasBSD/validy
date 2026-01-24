use proc_macro2::TokenStream;
use quote::quote;

use crate::attributes::ValidationAttributes;

pub fn get_failure_mode_boilerplate(attributes: &ValidationAttributes) -> TokenStream {
	if let Some(mode) = attributes.failure_mode.as_ref() {
		quote! {
		  FailureMode::#mode
		}
	} else {
		quote! {
		  ValidationSettings::get_failure_mode()
		}
	}
}
