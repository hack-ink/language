#![cfg(feature = "icu_locale_core")]

// std
use std::str::FromStr;
// crates.io
use icu_locale_core::{LanguageIdentifier, Locale};
// self
use language::prelude::*;

#[test]
fn icu_locale_roundtrip_should_work() {
	for language in Language::all() {
		if let Ok(locale) = Locale::try_from(language)
			&& let Ok(back) = Language::try_from(locale)
		{
			assert_eq!(language, back);
		}
	}
}

#[test]
fn icu_langid_roundtrip_should_work() {
	for language in Language::all() {
		if let Ok(langid) = LanguageIdentifier::try_from(language)
			&& let Ok(back) = Language::try_from(langid)
		{
			assert_eq!(language, back);
		}
	}
}

#[test]
fn unsupported_locale_should_fail() {
	let locale = Locale::from_str("es-419").unwrap();

	assert!(Language::try_from(locale).is_err());
}
