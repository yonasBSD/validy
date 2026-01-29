use validy::core::{AsyncValidateAndParseWithContext, Validate, ValidationError};
use validy::{assert_errors, validation_error};

#[derive(Debug, Validate, PartialEq)]
#[validate(payload, asynchronous, context = bool, failure_mode = LastFailPerField)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[modificate(async_custom_with_context(modificate))]
	#[modificate(async_custom_with_context(modificate_params, [&wrapper.b]))]
	pub a: String,
	#[modificate(async_custom_with_context(modificate))]
	#[modificate(async_custom_with_context(modificate_params_two, [&tmp_1_a]))]
	pub b: Option<String>,
}

async fn modificate(_value: &mut String, field: &str, _context: &bool) -> Result<(), ValidationError> {
	Err(validation_error!(field.to_string(), "custom_code", "custom message"))
}

async fn modificate_params(
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

async fn modificate_params_two(
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

#[tokio::test]
async fn should_modificate_async_customs_with_context() {
	let cases = [("a", false)];

	let mut wrapper = TestWrapper::default();
	for (case, context) in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::async_validate_and_parse_with_context(wrapper.clone(), context).await;

		assert_errors!(result, wrapper, {
			"a" => ("custom_code_2", "custom message 2"),
			"b" => ("custom_code_2", "custom message 2"),
		});
	}
}
