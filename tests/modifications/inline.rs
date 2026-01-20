use validy::core::{Validate, ValidateAndParse};

use validy::{assert_errors, assert_parsed};

#[allow(unused)]
#[derive(Debug, Default, Validate, PartialEq)]
#[validate(payload)]
struct Test {
	#[modify(inline(|x: &str| x.to_string() + "_test"))]
	pub a: String,
	#[modify(inline(|x: &str, a: &Option<String>| a.clone().unwrap_or(x.to_string()), [&wrapper.a]))]
	pub b: Option<String>,
}

#[test]
fn should_modify_inlines() {
	let cases = [("a", "a_test"), ("b", "b_test"), ("c", "c_test")];

	let mut wrapper = TestWrapper::default();
	let mut result = Test::validate_and_parse(&wrapper);
	assert_errors!(result, wrapper, {
		"a" => ("required", "is required"),
	});

	for (case, expected) in cases.iter() {
		wrapper.a = Some(case.to_string());
		result = Test::validate_and_parse(&wrapper);

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: expected.to_string(),
				b: None
			}
		);
	}

	let last_a = result.expect("should be a valid result").a;
	for (case, _) in cases.iter() {
		wrapper.b = Some(case.to_string());
		result = Test::validate_and_parse(&wrapper);

		assert_parsed!(
			result,
			wrapper,
			Test {
				a: last_a.clone(),
				b: Some(last_a.clone().replace("_test", ""))
			}
		);
	}
}
