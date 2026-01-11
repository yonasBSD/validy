use crate::primitives::commons::{ArgParser, parse_attrs, remove_parens};
use proc_macro_error::emit_error;
use syn::{Error, Result, Type, parse::ParseStream};

#[derive(Default)]
pub struct NestedArgs {
	pub value: Option<Type>,
	pub wrapper: Option<Type>,
}

impl ArgParser for NestedArgs {
	const POSITIONAL_KEYS: &'static [&'static str] = &["value", "wrapper"];

	fn apply_value(&mut self, name: &str, input: ParseStream) -> Result<()> {
		match name {
			"value" => self.value = Some(input.parse()?),
			"wrapper" => self.wrapper = Some(input.parse()?),
			_ => return Err(Error::new(input.span(), "unknown arg")),
		}

		Ok(())
	}
}

pub fn get_nested_type(input: ParseStream) -> (Option<Type>, Option<Type>) {
	let content = remove_parens(input);
	let NestedArgs { value, wrapper } = match content {
		Ok(content) => parse_attrs(&content)
			.inspect_err(|erro| emit_error!(erro.span(), "{}", erro))
			.unwrap_or_default(),
		Err(_) => NestedArgs::default(),
	};

	if let Some(nested_type) = &value {
		(Some(nested_type.clone()), wrapper)
	} else {
		emit_error!(input.span(), "needs the value type");

		(None, None)
	}
}
