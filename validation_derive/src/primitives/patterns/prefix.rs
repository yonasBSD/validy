use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, LitStr, Result, parse::ParseStream};

use crate::{
	fields::FieldAttributes,
	imports::import_validation_functions,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct PrefixArgs {
	pub prefix: Option<LitStr>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for PrefixArgs {
	fn default() -> Self {
		PrefixArgs {
			prefix: None,
			code: LitStr::new("prefix", Span::call_site()),
			message: LitStr::new("invalid prefix", Span::call_site()),
		}
	}
}

impl ArgParser for PrefixArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["prefix", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"prefix" => self.prefix = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_prefix(input: ParseStream, field: &FieldAttributes) -> TokenStream {
	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);
	let import = import_validation_functions("prefix::validate_prefix");

	let PrefixArgs { prefix, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => PrefixArgs::default(),
	};

	if prefix.is_none() {
		let span = input.span();
		emit_error!(span, "needs a prefix");
		return quote! {};
	}

	quote! {
	  use #import;
		if let Err(e) = validate_prefix(&#reference, #prefix, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
