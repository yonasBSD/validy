use proc_macro_error::emit_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, parse::ParseStream};

use crate::{
	attributes::ValidationAttributes,
	fields::FieldAttributes,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

#[derive(Default)]
pub struct FromTypeArgs {
	pub value: Option<Type>,
}

impl ArgParser for FromTypeArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["value"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"value" => self.value = Some(input.parse()?),
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_from_type(
	input: ParseStream,
	field: &mut FieldAttributes,
	attributes: &ValidationAttributes,
) -> TokenStream {
	if !attributes.payload {
		emit_error!(input.span(), "requires payload attribute");
		return quote! {};
	}

	let content = remove_parens(input);
	let FromTypeArgs { value } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => FromTypeArgs::default(),
	};

	if let Some(initial_type) = value.as_ref() {
		field.set_initial_type(initial_type);
	} else {
		emit_error!(input.span(), "needs a value");
	}

	quote! {}
}
