use validy::core::{Validate, ValidateAndParseWithContext, ValidationError};

use validy::{assert_errors, assert_parsed, validation_error};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload, context = bool)]
struct Test {
	#[modify(custom_with_context(modify))]
	pub a: String,
	#[modify(custom_with_context(modify_two, [&wrapper.a]))]
	pub b: Option<String>,
	#[modify(custom_with_context(modify_three, [&wrapper.a, &wrapper.b]))]
	pub c: Option<String>,
}

fn modify(value: &str, _field_name: &str, context: &bool) -> (String, Option<ValidationError>) {
	if *context {
		(value.to_string() + "_test", None)
	} else {
		(value.to_string(), None)
	}
}

fn modify_two(
	value: &str,
	_field_name: &str,
	context: &bool,
	extra_arg: &Option<String>,
) -> (String, Option<ValidationError>) {
	if *context {
		(extra_arg.clone().unwrap_or(value.to_string()), None)
	} else {
		(value.to_string(), None)
	}
}

fn modify_three(
	value: &str,
	field_name: &str,
	_context: &bool,
	a: &Option<String>,
	b: &Option<String>,
) -> (String, Option<ValidationError>) {
	match (a, b) {
		(_, None) => (
			value.to_string(),
			Some(validation_error!(
				field_name.to_string(),
				"custom_code",
				"custom message"
			)),
		),
		(Some(a), _) => (a.to_string(), None),
		(None, Some(b)) => (b.to_string(), None),
	}
}

#[test]
fn should_modify_customs_with_context() {
	let cases = [("a", false, "a"), ("b", true, "b_test"), ("c", true, "c_test")];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::validate_and_parse_with_context(&wrapper, &true);
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, context, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::validate_and_parse_with_context(&wrapper, context);

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: expected.to_string(),
				b: None,
				c: None,
			}
		);
	}

	let last_a = result.expect("should be a valid result").a;
	for (case, context, _) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse_with_context(&wrapper, context);

		let expected = if *context {
			Test {
				a: last_a.clone(),
				b: Some(last_a.clone().replace("_test", "")),
				c: None,
			}
		} else {
			Test {
				a: last_a.clone().replace("_test", ""),
				b: Some(case.to_string()),
				c: None,
			}
		};

		assert_parsed!(result, wrapper, expected);
	}

	wrapper.c = Some("".to_string());
	wrapper.b = None;
	result = Test::validate_and_parse_with_context(&wrapper, &true);
	assert_errors!(result, wrapper, {
	  "c" => ("custom_code", "custom message")
	});

	for (case, context, _) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse_with_context(&wrapper, context);

		let expected = if *context {
			Test {
				a: last_a.clone(),
				b: Some(last_a.clone().replace("_test", "")),
				c: Some(last_a.clone().replace("_test", "")),
			}
		} else {
			Test {
				a: last_a.clone().replace("_test", ""),
				b: Some(case.to_string()),
				c: Some(last_a.clone().replace("_test", "")),
			}
		};

		assert_parsed!(result, wrapper, expected);
	}
}
