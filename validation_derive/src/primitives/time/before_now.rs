use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Ident, LitBool, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct BeforeArgs {
	pub accept_equals: LitBool,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for BeforeArgs {
	fn default() -> Self {
		BeforeArgs {
			accept_equals: LitBool::new(false, Span::call_site()),
			code: LitStr::new("before_now", Span::call_site()),
			message: LitStr::new("is after now", Span::call_site()),
		}
	}
}

impl ArgParser for BeforeArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["accept_equals", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"accept_equals" => self.accept_equals = input.parse()?,
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}

	fn apply_positional(&mut self, index: usize, input: ParseStream) -> Result<()> {
		if input.peek(Ident) {
			match input.parse::<Ident>()?.to_string().as_str() {
				"accept_equals" => self.accept_equals = LitBool::new(true, Span::call_site()),
				_ => return Err(Error::new(input.span(), "incomplete or unknown arg")),
			}

			Ok(())
		} else {
			let name = Self::POSITIONAL_KEYS
				.get(index)
				.ok_or_else(|| Error::new(input.span(), "too many positional args"))?;

			self.apply_value(name, input)
		}
	}
}

pub fn create_before_now(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("time::validate_is_before_now");

	let BeforeArgs {
		accept_equals,
		code,
		message,
	} = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => BeforeArgs::default(),
	};

	quote! {
	  use #import;
		if let Err(e) = validate_is_before_now(&#reference, #accept_equals, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
