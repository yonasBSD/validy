use crate::fields::FieldAttributes;
use proc_macro2::TokenStream;

pub struct DefaultsCodeFactory<'a>(pub &'a mut Vec<FieldAttributes>);

impl<'a> DefaultsCodeFactory<'a> {
	pub fn operations(&mut self) -> Vec<TokenStream> {
		self.0.iter_mut().map(|field| field.get_operations()).collect()
	}
}
