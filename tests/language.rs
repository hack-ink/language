// std
use std::{collections::HashSet, str::FromStr};
// self
use language::prelude::*;

#[cfg(feature = "serde")]
#[test]
fn serde_should_work() {
	let lang = Language::En;
	let serialized = serde_json::to_string(&lang).unwrap();

	assert_eq!(serialized, "\"en\"");

	let deserialized: Language = serde_json::from_str(&serialized).unwrap();

	assert_eq!(deserialized, lang);
}

#[test]
fn roundtrip_tags_should_work() {
	for language in Language::all() {
		let tag = language.tag();

		assert_eq!(Language::from_str(tag).unwrap(), language);
	}
}

#[test]
fn tags_should_be_unique() {
	let mut seen = HashSet::new();

	for language in Language::all() {
		let tag = language.tag();

		assert!(seen.insert(tag), "Duplicate tag detected: {tag}.");
	}
	assert_eq!(seen.len(), Language::all().len());
}

#[test]
fn names_should_not_be_empty() {
	for language in Language::all() {
		assert!(!language.name().is_empty(), "English name is missing.");
		assert!(!language.local_name().is_empty(), "Autonym is missing.");
	}
}
