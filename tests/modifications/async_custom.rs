use validy::core::{AsyncValidateAndParse, Validate, ValidationError};

use validy::{assert_errors, assert_parsed, validation_error};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload, asynchronous)]
struct Test {
	#[modify(async_custom(modify))]
	pub a: String,
	#[modify(async_custom(modify_two, [&wrapper.a]))]
	pub b: Option<String>,
	#[modify(async_custom(modify_three, [&wrapper.a, &wrapper.b]))]
	pub c: Option<String>,
}

async fn modify(value: &str, _field_name: &str) -> (String, Option<ValidationError>) {
	(value.to_string() + "_test", None)
}

async fn modify_two(value: &str, _field_name: &str, extra_arg: &Option<String>) -> (String, Option<ValidationError>) {
	(extra_arg.clone().unwrap_or(value.to_string()), None)
}

async fn modify_three(
	value: &str,
	field_name: &str,
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

#[tokio::test]
async fn should_modify_customs() {
	let cases = [("a", "a_test"), ("b", "b_test"), ("c", "c_test")];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::async_validate_and_parse(&wrapper).await;
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::async_validate_and_parse(&wrapper).await;

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
	for (case, _) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::async_validate_and_parse(&wrapper).await;

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: last_a.clone(),
				b: Some(last_a.clone().replace("_test", "")),
				c: None
			}
		);
	}

	wrapper.c = Some("".to_string());
	wrapper.b = None;
	result = Test::async_validate_and_parse(&wrapper).await;
	assert_errors!(result, wrapper, {
	  "c" => ("custom_code", "custom message")
	});

	for (case, _) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::async_validate_and_parse(&wrapper).await;

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: last_a.clone(),
				b: Some(last_a.clone().replace("_test", "")),
				c: Some(last_a.clone().replace("_test", "")),
			}
		);
	}
}
