// std
use std::str::FromStr;
// crates.io
use icu_locale_core::{LanguageIdentifier, Locale};
// self
use crate::prelude::*;

impl TryFrom<Language> for Locale {
	type Error = Error;

	fn try_from(value: Language) -> Result<Self, Self::Error> {
		Locale::from_str(value.tag()).map_err(Error::from)
	}
}

impl TryFrom<Locale> for Language {
	type Error = Error;

	fn try_from(value: Locale) -> Result<Self, Self::Error> {
		let value = value.to_string();

		Language::from_str(&value).map_err(|_| Error::UnsupportedIcuLocale(value))
	}
}

impl TryFrom<Language> for LanguageIdentifier {
	type Error = Error;

	fn try_from(value: Language) -> Result<Self, Self::Error> {
		LanguageIdentifier::from_str(value.tag()).map_err(Error::from)
	}
}

impl TryFrom<LanguageIdentifier> for Language {
	type Error = Error;

	fn try_from(value: LanguageIdentifier) -> Result<Self, Self::Error> {
		let value = value.to_string();

		Language::from_str(&value).map_err(|_| Error::UnsupportedIcuLocale(value))
	}
}
