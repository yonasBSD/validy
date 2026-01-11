use std::cell::RefCell;

use proc_macro_error::emit_error;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ExprArray, LitStr, Result, parse::ParseStream};

use crate::{
	ImportsSet,
	fields::FieldAttributes,
	imports::Import,
	primitives::commons::{ArgParser, parse_attrs, remove_parens},
};

pub struct AnyArgs {
	pub items: Option<ExprArray>,
	pub code: LitStr,
	pub message: LitStr,
}

impl Default for AnyArgs {
	fn default() -> Self {
		AnyArgs {
			items: None,
			code: LitStr::new("any", Span::call_site()),
			message: LitStr::new("has item outside allowlist", Span::call_site()),
		}
	}
}

impl ArgParser for AnyArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["items", "message", "code"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"items" => self.items = Some(input.parse()?),
			"code" => self.code = input.parse()?,
			"message" => self.message = input.parse()?,
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn create_any(input: ParseStream, field: &FieldAttributes, imports: &RefCell<ImportsSet>) -> TokenStream {
	imports
		.borrow_mut()
		.add(Import::ValidationFunction("iter::validate_any as validate_any_fn"));

	let field_name = field.get_name();
	let reference = field.get_reference();
	let content = remove_parens(input);

	let AnyArgs { items, code, message } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => AnyArgs::default(),
	};

	if items.is_none() {
		emit_error!(input.span(), "needs a collection of items to use as allowlist");
		return quote! {};
	}

	quote! {
		if let Err(e) = validate_any_fn(&#reference, #items, #field_name, #code, #message) {
		  errors.push(e);
	  }
	}
}
