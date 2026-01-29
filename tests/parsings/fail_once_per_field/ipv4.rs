use std::net::Ipv4Addr;

use validy::core::{Validate, ValidateAndParse};

use validy::assert_errors;

#[derive(Debug, Validate, PartialEq)]
#[validate(payload, failure_mode = FullFail)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(ipv4("custom message", "custom_code"))]
	pub a: Ipv4Addr,
	#[special(from_type(String))]
	#[parse(ipv4("custom message", "custom_code"))]
	pub b: Option<Ipv4Addr>,
}

#[test]
fn should_parse_ipv4s() {
	let cases = [""];

	let mut wrapper = TestWrapper::default();
	for case in cases.iter() {
		wrapper.a = Some(case.to_string());
		wrapper.b = Some(case.to_string());
		let result = Test::validate_and_parse(wrapper.clone());

		assert_errors!(result, wrapper, {
			"a" => [("custom_code", "custom message")],
			"b" => [("custom_code", "custom message")],
		});
	}
}
