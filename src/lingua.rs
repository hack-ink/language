// std
use std::str::FromStr;
// crates.io
use lingua::{IsoCode639_1, IsoCode639_3, Language as LinguaLanguage};
// self
use crate::prelude::*;

impl TryFrom<Language> for LinguaLanguage {
	type Error = Error;

	fn try_from(value: Language) -> Result<Self, Self::Error> {
		let base = base_subtag(value);

		let language = match base {
			"fil" => LinguaLanguage::Tagalog,
			_ => lingua_language_for_base(base).ok_or(Error::UnsupportedLinguaBase(base))?,
		};

		Ok(language)
	}
}

impl TryFrom<LinguaLanguage> for Language {
	type Error = Error;

	fn try_from(value: LinguaLanguage) -> Result<Self, Self::Error> {
		match value {
			LinguaLanguage::Tagalog => {
				return Language::try_from("fil")
					.map_err(|_| Error::UnsupportedLinguaLanguage(value));
			},
			LinguaLanguage::Chinese => {
				return Language::try_from("zh-Hans")
					.map_err(|_| Error::UnsupportedLinguaLanguage(value));
			},
			_ => {},
		}

		let iso_639_1 = value.iso_code_639_1().to_string();

		if let Ok(language) = Language::try_from(iso_639_1.as_str()) {
			return Ok(language);
		}

		let iso_639_3 = value.iso_code_639_3().to_string();

		Language::try_from(iso_639_3.as_str())
			.map_err(|_| Error::UnsupportedLinguaLanguage(value))
	}
}

fn base_subtag(language: Language) -> &'static str {
	let tag = language.tag();

	tag.split_once('-').map_or(tag, |(base, _)| base)
}

fn lingua_language_for_base(base: &str) -> Option<LinguaLanguage> {
	let languages = LinguaLanguage::all();

	if let Ok(c) = IsoCode639_1::from_str(base)
		&& let Some(l) = languages.iter().find(|lang| lang.iso_code_639_1() == c) {
			return Some(*l);
		}
	if let Ok(c) = IsoCode639_3::from_str(base)
		&& let Some(l) = languages.iter().find(|lang| lang.iso_code_639_3() == c) {
			return Some(*l);
		}

	None
}
