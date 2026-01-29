use validy::core::{Validate, ValidateAndParseWithContext, ValidationError};

use validy::{assert_errors, validation_error};

#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload, context = bool, failure_mode = FailFast)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[modificate(custom_with_context(modificate))]
	#[modificate(custom_with_context(modificate_params, [&wrapper.b]))]
	pub a: String,
	#[modificate(custom_with_context(modificate))]
	#[modificate(custom_with_context(modificate_params_two, [&tmp_1_a]))]
	pub b: Option<String>,
}

fn modificate(_value: &mut String, field: &str, _context: &bool) -> Result<(), ValidationError> {
	Err(validation_error!(field.to_string(), "custom_code", "custom message"))
}

fn modificate_params(
	_value: &mut String,
	field: &str,
	_context: &bool,
	_extra_arg: &Option<String>,
) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

fn modificate_params_two(
	_value: &mut String,
	field: &str,
	_context: &bool,
	_extra_arg: &Option<String>,
) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

#[test]
fn should_modificate_customs_with_context() {
	let cases = [("a", false)];

	let mut wrapper = TestWrapper::default();
	for (case, context) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::validate_and_parse_with_context(wrapper.clone(), context);

		assert_errors!(result, wrapper, {
			"a" => ("custom_code", "custom message"),
		});
	}
}
