#![allow(missing_docs)]

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// The language tag is not supported by this crate.
	#[error("Unsupported language tag `{0}`.")]
	UnsupportedLanguageTag(String),

	/// Parsing an ICU locale failed.
	#[cfg(feature = "icu_locale_core")]
	#[error("Failed to parse ICU locale: {0}")]
	IcuLocaleParse(#[from] icu_locale_core::ParseError),
	/// The ICU locale is not mapped to this crate's `Language`.
	#[cfg(feature = "icu_locale_core")]
	#[error("Unsupported ICU locale `{0}`.")]
	UnsupportedIcuLocale(String),

	/// The base language subtag is not supported by `lingua`.
	#[cfg(feature = "lingua")]
	#[error("Unsupported lingua base language subtag `{0}`.")]
	UnsupportedLinguaBase(&'static str),
	/// The `lingua::Language` variant is not mapped in this crate.
	#[cfg(feature = "lingua")]
	#[error("Unsupported lingua Language variant `{0:?}`.")]
	UnsupportedLinguaLanguage(lingua::Language),

	/// The base language subtag is not supported by `whatlang`.
	#[cfg(feature = "whatlang")]
	#[error("Unsupported whatlang base language subtag `{0}`.")]
	UnsupportedWhatlangBase(&'static str),
	/// The `whatlang::Lang` variant is not mapped in this crate.
	#[cfg(feature = "whatlang")]
	#[error("Unsupported whatlang Lang variant `{0:?}`.")]
	UnsupportedWhatlangLang(whatlang::Lang),
}
