use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitInt, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct NowArgs {
	pub ms_tolerance: LitInt,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for NowArgs {
	fn default() -> Self {
		NowArgs {
			ms_tolerance: LitInt::new("500", Span::call_site()),
			code: LitStr::new("now", Span::call_site()),
			message: LitStr::new("isn't now", Span::call_site()),
		}
	}
}

impl ArgParser for NowArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["ms_tolerance", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"ms_tolerance" => self.ms_tolerance = input.parse()?,
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_now(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("time::validate_is_now");

	let NowArgs {
		ms_tolerance,
		code,
		message,
	} = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => NowArgs::default(),
	};

	quote! {
	  use #import;
		if let Err(e) = validate_is_now(&#reference, #ms_tolerance, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
