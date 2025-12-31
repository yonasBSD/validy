pub fn capitalize(value: &str) -> String {
	use heck::ToTitleCase;
	value.to_title_case()
}

pub fn camel_case(value: &str) -> String {
	use heck::ToUpperCamelCase;
	value.to_upper_camel_case()
}

pub fn lower_camel_case(value: &str) -> String {
	use heck::ToLowerCamelCase;
	value.to_lower_camel_case()
}

pub fn snake_case(value: &str) -> String {
	use heck::ToSnakeCase;
	value.to_snake_case()
}

pub fn shouty_snake_case(value: &str) -> String {
	use heck::ToShoutySnakeCase;
	value.to_shouty_snake_case()
}

pub fn kebab_case(value: &str) -> String {
	use heck::ToKebabCase;
	value.to_kebab_case()
}

pub fn shouty_kebab_case(value: &str) -> String {
	use heck::ToShoutyKebabCase;
	value.to_shouty_kebab_case()
}

pub fn train_case(value: &str) -> String {
	use heck::ToTrainCase;
	value.to_train_case()
}
