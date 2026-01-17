// crates.io
use whatlang::Lang;
// self
use crate::prelude::*;

fn base_subtag(language: Language) -> &'static str {
	let tag = language.tag();

	debug_assert!(!tag.is_empty());

	// whatlang only understands base language codes; drop script/region/variants when mapping.
	tag.split('-').next().unwrap()
}

impl TryFrom<Language> for Lang {
	type Error = Error;

	fn try_from(value: Language) -> Result<Self, Self::Error> {
		let base = base_subtag(value);

		let lang = match base {
			"af" => Lang::Afr,
			"am" => Lang::Amh,
			"ar" => Lang::Ara,
			"az" => Lang::Aze,
			"be" => Lang::Bel,
			"bg" => Lang::Bul,
			"bn" => Lang::Ben,
			"ca" => Lang::Cat,
			"cs" => Lang::Ces,
			"cy" => Lang::Cym,
			"da" => Lang::Dan,
			"de" => Lang::Deu,
			"el" => Lang::Ell,
			"en" => Lang::Eng,
			"es" => Lang::Spa,
			"et" => Lang::Est,
			"fa" => Lang::Pes,
			"fi" => Lang::Fin,
			"fr" => Lang::Fra,
			"gu" => Lang::Guj,
			"he" => Lang::Heb,
			"hi" => Lang::Hin,
			"hr" => Lang::Hrv,
			"hu" => Lang::Hun,
			"hy" => Lang::Hye,
			"id" => Lang::Ind,
			"it" => Lang::Ita,
			"ja" => Lang::Jpn,
			"ka" => Lang::Kat,
			"km" => Lang::Khm,
			"kn" => Lang::Kan,
			"ko" => Lang::Kor,
			"la" => Lang::Lat,
			"lt" => Lang::Lit,
			"lv" => Lang::Lav,
			"mk" => Lang::Mkd,
			"ml" => Lang::Mal,
			"mr" => Lang::Mar,
			"my" => Lang::Mya,
			"nb" => Lang::Nob,
			"nl" => Lang::Nld,
			"ne" => Lang::Nep,
			"or" | "ory" => Lang::Ori,
			"pa" => Lang::Pan,
			"pl" => Lang::Pol,
			"pt" => Lang::Por,
			"ro" => Lang::Ron,
			"ru" => Lang::Rus,
			"si" => Lang::Sin,
			"sk" => Lang::Slk,
			"sl" => Lang::Slv,
			"sr" => Lang::Srp,
			"sv" => Lang::Swe,
			"ta" => Lang::Tam,
			"te" => Lang::Tel,
			"th" => Lang::Tha,
			"tk" => Lang::Tuk,
			"tl" => return Err(Error::UnsupportedWhatlangBase(base)),
			// whatlang names Tagalog as `Tgl` (ISO 639-2/3). In BCP47, Tagalog keeps the legacy
			// `tl` primary subtag while Filipino (the Tagalog-based standard) uses `fil`. Accept
			// only `fil` here so callers stay BCP47-aligned while still roundtripping through
			// whatlang.
			"fil" => Lang::Tgl,
			"tr" => Lang::Tur,
			"uk" => Lang::Ukr,
			"ur" => Lang::Urd,
			"uz" => Lang::Uzb,
			"vi" => Lang::Vie,
			"yi" => Lang::Yid,
			"zh" => Lang::Cmn,
			"zu" => Lang::Zul,
			_ => return Err(Error::UnsupportedWhatlangBase(base)),
		};

		Ok(lang)
	}
}

impl TryFrom<Lang> for Language {
	type Error = Error;

	fn try_from(value: Lang) -> Result<Self, Self::Error> {
		let tag = match value {
			Lang::Afr => "af",
			Lang::Amh => "am",
			Lang::Ara => "ar",
			Lang::Aze => "az",
			Lang::Bel => "be",
			Lang::Ben => "bn",
			Lang::Bul => "bg",
			Lang::Cat => "ca",
			Lang::Ces => "cs",
			Lang::Cmn => "zh-Hans",
			Lang::Cym => "cy",
			Lang::Dan => "da",
			Lang::Deu => "de",
			Lang::Ell => "el",
			Lang::Eng => "en",
			Lang::Est => "et",
			Lang::Fin => "fi",
			Lang::Fra => "fr",
			Lang::Guj => "gu",
			Lang::Heb => "he",
			Lang::Hin => "hi",
			Lang::Hrv => "hr",
			Lang::Hun => "hu",
			Lang::Hye => "hy",
			Lang::Ind => "id",
			Lang::Ita => "it",
			Lang::Jpn => "ja",
			Lang::Kan => "kn",
			Lang::Kat => "ka",
			Lang::Khm => "km",
			Lang::Kor => "ko",
			Lang::Lav => "lv",
			Lang::Lit => "lt",
			Lang::Mal => "ml",
			Lang::Mar => "mr",
			Lang::Mkd => "mk",
			Lang::Mya => "my",
			Lang::Nep => "ne",
			Lang::Nld => "nl",
			Lang::Nob => "nb",
			Lang::Ori => "or",
			Lang::Pan => "pa",
			Lang::Pes => "fa",
			Lang::Pol => "pl",
			Lang::Por => "pt",
			Lang::Ron => "ro",
			Lang::Rus => "ru",
			Lang::Sin => "si",
			Lang::Slk => "sk",
			Lang::Slv => "sl",
			Lang::Spa => "es",
			Lang::Srp => "sr",
			Lang::Swe => "sv",
			Lang::Tam => "ta",
			Lang::Tel => "te",
			Lang::Tha => "th",
			// whatlang names Tagalog as `Tgl` (ISO 639-2/3). In BCP47, Tagalog keeps the legacy
			// `tl` primary subtag while Filipino (the Tagalog-based standard language) uses
			// `fil`. Map `Tgl` to `fil` so we stay aligned with BCP47 naming but still roundtrip
			// through whatlang.
			Lang::Tgl => "fil",
			Lang::Tur => "tr",
			Lang::Tuk => "tk",
			Lang::Ukr => "uk",
			Lang::Urd => "ur",
			Lang::Uzb => "uz",
			Lang::Vie => "vi",
			Lang::Yid => "yi",
			Lang::Zul => "zu",
			Lang::Sna => "sn",
			// A few whatlang variants (for example Yiddish `Yid` → `yi` and Latin `Lat` → `la`)
			// are absent from the generated translation.io dataset. Keep this arm fallible until
			// those tags exist in `Language`.
			_ => return Err(Error::UnsupportedWhatlangLang(value)),
		};

		Language::try_from(tag).map_err(|_| Error::UnsupportedWhatlangLang(value))
	}
}
