use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct SomeArgs {
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for SomeArgs {
	fn default() -> Self {
		SomeArgs {
			code: LitStr::new("required", Span::call_site()),
			message: LitStr::new("is required", Span::call_site()),
		}
	}
}

impl ArgParser for SomeArgs {
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

pub fn create_is_some(input: ParseStream, field: &mut FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("option::validate_is_some");

	let SomeArgs { code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => SomeArgs::default(),
	};

	quote! {
	  use #import;
		if let Err(e) = validate_is_some(&#reference, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
