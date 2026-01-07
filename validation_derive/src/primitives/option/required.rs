use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct RequiredArgs {
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for RequiredArgs {
	fn default() -> Self {
		RequiredArgs {
			code: LitStr::new("required", Span::call_site()),
			message: LitStr::new("is required", Span::call_site()),
		}
	}
}

impl ArgParser for RequiredArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_required(input: ParseStream, field: &mut FieldAttributes) -> TokenStream {
	let content = remove_parens(input);

	let required_args = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => RequiredArgs::default(),
	};

	if field.is_option() {
		emit_error!(input.span(), "for Option fields, use 'is_some' instead 'required'");
		return quote! {};
	}

	field.set_required_args(required_args);

	quote! {}
}
