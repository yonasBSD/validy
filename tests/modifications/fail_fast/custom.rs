use validy::core::{Validate, ValidateAndParse, ValidationError};

use validy::{assert_errors, validation_error};

#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload, failure_mode = FailFast)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[modificate(custom(modificate))]
	#[modificate(custom(modificate_params, [&wrapper.b]))]
	pub a: String,
	#[modificate(custom(modificate))]
	#[modificate(custom(modificate_params_two, [&tmp_1_a]))]
	pub b: Option<String>,
}

fn modificate(_value: &mut String, field: &str) -> Result<(), ValidationError> {
	Err(validation_error!(field.to_string(), "custom_code", "custom message"))
}

fn modificate_params(_value: &mut String, field: &str, _extra_arg: &Option<String>) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

fn modificate_params_two(_value: &mut String, field: &str, _extra_arg: &Option<String>) -> Result<(), ValidationError> {
	Err(validation_error!(
		field.to_string(),
		"custom_code_2",
		"custom message 2"
	))
}

#[test]
fn should_modificate_customs() {
	let cases = ["a"];

	let mut wrapper = TestWrapper::default();
	for case in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::validate_and_parse(wrapper.clone());

		assert_errors!(result, wrapper, {
			"a" => ("custom_code", "custom message"),
		});
	}
}
