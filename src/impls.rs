use crate::core::{NestedValidationError, SimpleValidationError, ValidationError, ValidationErrors};
use std::{borrow::Cow, collections::HashMap};

impl NestedValidationError {
	pub fn from(errors: ValidationErrors, field: impl Into<Cow<'static, str>>) -> Self {
		NestedValidationError {
			field: field.into(),
			code: "nested".into(),
			errors,
		}
	}

	pub fn new(field: impl Into<Cow<'static, str>>) -> Self {
		let errors = HashMap::<Cow<'static, str>, ValidationError>::new();

		NestedValidationError {
			field: field.into(),
			code: "nested".into(),
			errors,
		}
	}

	pub fn put(&mut self, error: ValidationError) {
		match error {
			ValidationError::Node(error) => {
				self.errors.insert(error.field.clone(), error.into());
			}
			ValidationError::Leaf(error) => {
				self.errors.insert(error.field.clone(), error.into());
			}
		}
	}
}

impl SimpleValidationError {
	pub fn new(field: impl Into<Cow<'static, str>>, code: impl Into<Cow<'static, str>>) -> Self {
		SimpleValidationError {
			field: field.into(),
			code: code.into(),
			message: None,
		}
	}

	pub fn with_message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
		self.message = Some(message.into());
		self
	}
}

impl From<NestedValidationError> for ValidationError {
	fn from(value: NestedValidationError) -> Self {
		ValidationError::Node(value)
	}
}

impl From<SimpleValidationError> for ValidationError {
	fn from(value: SimpleValidationError) -> Self {
		ValidationError::Leaf(value)
	}
}
