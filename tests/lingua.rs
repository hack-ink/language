#![cfg(feature = "lingua")]

// crates.io
use lingua::Language as LinguaLanguage;
// self
use language::prelude::*;

fn base_subtag(language: Language) -> &'static str {
	let tag = language.tag();

	tag.split_once('-').map_or(tag, |(base, _)| base)
}

#[test]
fn to_lingua_roundtrip_supported() {
	for language in Language::all() {
		if let Ok(lang) = LinguaLanguage::try_from(language)
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
	assert!(LinguaLanguage::try_from(Language::Ba).is_err());
}

#[test]
fn tagalog_maps_to_tagalog() {
	assert_eq!(LinguaLanguage::try_from(Language::Fil).unwrap(), LinguaLanguage::Tagalog);
}

#[test]
fn lingua_prefers_canonical_locale() {
	assert_eq!(Language::try_from(LinguaLanguage::Chinese).unwrap(), Language::ZhHans);
}
