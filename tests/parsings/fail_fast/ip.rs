use std::net::IpAddr;

use validy::core::{Validate, ValidateAndParse};

use validy::assert_errors;

#[derive(Debug, Validate, PartialEq)]
#[validate(payload, failure_mode = FailFast)]
#[wrapper_derive(Debug, Clone)]
struct Test {
	#[special(from_type(String))]
	#[parse(ip("custom message", "custom_code"))]
	pub a: IpAddr,
	#[special(from_type(String))]
	#[parse(ip("custom message", "custom_code"))]
	pub b: Option<IpAddr>,
}

#[test]
fn should_parse_ips() {
	let cases = [""];

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
