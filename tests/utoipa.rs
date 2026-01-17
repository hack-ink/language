#![cfg(feature = "utoipa")]

// crates.io
use utoipa::PartialSchema;
// self
use language::prelude::*;

#[test]
fn utoipa_schema_should_include_language_tags() {
	let schema = Language::schema();
	let value = serde_json::to_value(schema).unwrap();
	let enum_values = value
		.get("enum")
		.and_then(|values| values.as_array())
		.expect("Language schema should include enum values.");

	let has_tag = |tag: &str| enum_values.iter().any(|value| value.as_str() == Some(tag));

	assert!(has_tag("en"), "Language schema should include the 'en' tag.");
	assert!(has_tag("zh-Hans"), "Language schema should include the 'zh-Hans' tag.");
}
