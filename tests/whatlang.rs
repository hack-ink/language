#![cfg(feature = "whatlang")]

// crates.io
use whatlang::Lang;
// self
use language::prelude::*;

fn base_subtag(language: Language) -> &'static str {
	language.as_tag().split('-').next().unwrap()
}

#[test]
fn to_whatlang_roundtrip_supported() {
	for language in Language::all() {
		if let Ok(lang) = Lang::try_from(language)
			&& let Ok(back) = Language::try_from(lang)
		{
			let lhs = base_subtag(language);
			let rhs = base_subtag(back);

			if !(lhs == rhs || (lhs == "tl" && rhs == "fil")) {
				panic!("Base mismatch: {lhs} -> {:?} -> {rhs}.", lang);
			}
		}
	}
}

#[test]
fn unsupported_language_returns_err() {
	assert!(Lang::try_from(Language::Ba).is_err());
}

#[test]
fn tagalog_maps_to_tgl() {
	assert_eq!(Lang::try_from(Language::Fil).unwrap(), Lang::Tgl);
}

#[test]
fn whatlang_prefers_canonical_locale() {
	assert_eq!(Language::try_from(Lang::Spa).unwrap(), Language::Es);
}
