use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct NaiveTimeArgs {
	pub format: Option<LitStr>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for NaiveTimeArgs {
	fn default() -> Self {
		NaiveTimeArgs {
			format: None,
			code: LitStr::new("naive_time", Span::call_site()),
			message: LitStr::new("invalid naive time format", Span::call_site()),
		}
	}
}

impl ArgParser for NaiveTimeArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["format", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"format" => self.format = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_naive_time(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("time::validate_naive_time");

	let NaiveTimeArgs { format, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => NaiveTimeArgs::default(),
	};

	if format.is_none() {
		emit_error!(input.span(), "needs a format");
		return quote! {};
	}

	quote! {
	  use #import;
		if let Err(e) = validate_naive_time(&#reference, #format, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
