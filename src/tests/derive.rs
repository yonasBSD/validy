use crate::core::Validate;

#[derive(Debug, Validate)]
#[validate(payload)]
struct User {
	#[modify(uppercase)]
	#[validate(email)]
	pub name: String,
}

// #[derive(Debug, Validate)]
// #[validate(payload)]
// struct Users {
// 	#[special(for_each())]
// 	pub names: Vec<String>,
// }

pub fn a() {
	String::default();
}
// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_call_fails_required_field() {
// 		let wrapper = UserWrapper { name: None };
// 		let result = User::validate_and_parse(&wrapper);

// 		assert!(result.is_err());
// 	}

// 	#[test]
// 	fn test_apply_modifications() {
// 		let wrapper = UserWrapper {
// 			name: Some("TeStVaLuE".to_string()),
// 		};

// 		let result = User::validate_and_parse(&wrapper);
// 		assert!(result.is_ok());

// 		let user = result.unwrap();
// 		assert_eq!(user.name, "TESTVALUE");
// 	}
// }
