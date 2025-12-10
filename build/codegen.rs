// Standard library imports.
use std::{
	collections::BTreeMap,
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
	fs,
	io::Error as IoError,
	path::Path,
};

// crates.io
use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub enum CodegenError {
	Io(IoError),
	Parse(String),
	Validation(String),
}
impl From<IoError> for CodegenError {
	fn from(err: IoError) -> Self {
		CodegenError::Io(err)
	}
}
impl Display for CodegenError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match self {
			CodegenError::Io(e) => write!(f, "I/O error: {e}"),
			CodegenError::Parse(msg) => write!(f, "Parse error: {msg}"),
			CodegenError::Validation(msg) => write!(f, "Validation error: {msg}"),
		}
	}
}
impl StdError for CodegenError {}

#[derive(Debug)]
struct TagSpec {
	ident: String,
	tag: String,
	english: String,
	autonym: String,
}

pub fn generate(languages_path: &Path) -> Result<String, CodegenError> {
	let specs = load_languages(languages_path)?;

	Ok(render(&specs))
}

fn load_languages(path: &Path) -> Result<Vec<TagSpec>, CodegenError> {
	let content = fs::read_to_string(path)?;
	let document = Html::parse_document(&content);
	let row_selector = Selector::parse("#languages-table tbody tr")
		.map_err(|err| CodegenError::Parse(format!("Invalid selector: {err}")))?;
	let cell_selector = Selector::parse("td")
		.map_err(|err| CodegenError::Parse(format!("Invalid selector: {err}")))?;
	let mut merged: BTreeMap<String, (String, String)> = BTreeMap::new();

	for row in document.select(&row_selector) {
		let mut cells = row.select(&cell_selector).take(4).map(extract_text).collect::<Vec<_>>();

		if cells.len() < 4 {
			continue;
		}

		let tag = cells.remove(0);
		let name = cells.remove(0);
		let region = cells.remove(0);
		let native = cells.remove(0);
		// Skip Translation.io synthetic/system-specific tags that are not valid BCP47 language
		// codes.
		let lower_tag = tag.to_ascii_lowercase();

		if lower_tag.contains("system") {
			continue;
		}

		let english = if !region.is_empty() { region } else { name };

		if tag.is_empty() || english.is_empty() || native.is_empty() {
			continue;
		}

		merged.entry(tag).or_insert((english, native));
	}

	if merged.is_empty() {
		return Err(CodegenError::Parse("No languages found in source file.".into()));
	}

	let mut specs = Vec::with_capacity(merged.len());

	for (tag, (english, native)) in merged {
		validate_tag(&tag).map_err(|msg| CodegenError::Validation(format!("{msg} (tag {tag})")))?;

		let ident = tag_to_ident(&tag)
			.map_err(|msg| CodegenError::Validation(format!("{msg} (tag {tag})")))?;

		specs.push(TagSpec { ident, tag, english, autonym: native });
	}

	Ok(specs)
}

fn extract_text(cell: ElementRef<'_>) -> String {
	let joined = cell.text().collect::<String>();
	normalize_whitespace(&joined)
}

fn validate_tag(tag: &str) -> Result<(), String> {
	if tag.is_empty() {
		return Err("Tag is empty.".into());
	}
	if !tag.is_ascii() {
		return Err("Tag contains non-ASCII characters.".into());
	}
	if tag.starts_with('-') || tag.ends_with('-') {
		return Err("Tag must not start or end with a hyphen.".into());
	}

	let mut parts = tag.split('-').peekable();
	let Some(language) = parts.next() else {
		return Err("Tag is missing a language subtag.".into());
	};

	if !(2..=3).contains(&language.len()) || !language.chars().all(|c| c.is_ascii_alphabetic()) {
		return Err(format!("Invalid language subtag `{language}`."));
	}

	if let Some(script) = parts.peek().copied()
		&& script.len() == 4
		&& script.chars().all(|c| c.is_ascii_alphabetic())
	{
		parts.next();
	}
	if let Some(region) = parts.peek().copied() {
		let is_alpha_region = region.len() == 2 && region.chars().all(|c| c.is_ascii_alphabetic());
		let is_numeric_region = region.len() == 3 && region.chars().all(|c| c.is_ascii_digit());

		if is_alpha_region || is_numeric_region {
			parts.next();
		}
	}

	for variant in parts {
		if variant.len() < 4 || variant.len() > 8 {
			return Err(format!("Variant subtag `{variant}` has invalid length."));
		}
		if !variant.chars().all(|c| c.is_ascii_alphanumeric()) {
			return Err(format!("Variant subtag `{variant}` contains invalid characters."));
		}
	}

	Ok(())
}

fn normalize_whitespace(input: &str) -> String {
	let mut out = String::new();
	let mut last_space = false;

	for ch in input.chars() {
		// Translation.io includes soft hyphens in some autonyms (for example Azeri); drop those and
		// other control characters so generated code stays ASCII and lint-friendly.
		if ch == '\u{00ad}' || ch.is_control() {
			continue;
		}

		let is_space = ch.is_whitespace();

		if is_space {
			if !last_space {
				out.push(' ');
			}
		} else {
			out.push(ch);
		}

		last_space = is_space;
	}

	out.trim().to_string()
}

fn tag_to_ident(tag: &str) -> Result<String, String> {
	let mut ident = String::new();

	for part in tag.split('-') {
		let mut chars = part.chars();
		let Some(first) = chars.next() else {
			return Err("Tag contains an empty component.".into());
		};

		ident.push(first.to_ascii_uppercase());
		ident.extend(chars.map(|c| c.to_ascii_lowercase()));
	}

	if ident.is_empty() {
		return Err("Failed to derive identifier from tag.".into());
	}

	Ok(ident)
}

fn escape(s: &str) -> String {
	s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn render(specs: &[TagSpec]) -> String {
	let mut out = String::new();

	out.push_str(
		"\
// self
use Language::*;

/// Generated from Translation.io languages-with-plural-cases snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
",
	);

	for spec in specs {
		out.push_str(&format!(
			"	/// {}
	{},
",
			spec.english, spec.ident
		));
	}

	out.push_str(
		"}
impl Language {
	/// Get all languages.
	#[rustfmt::skip]
	pub const fn all() -> [Self; ",
	);
	out.push_str(&specs.len().to_string());
	out.push_str(
		"] {
		[
",
	);

	for spec in specs {
		out.push_str(&format!(
			"			{},
",
			spec.ident
		));
	}

	out.push_str(
		"		]
	}

	/// Get the language tag.
	pub fn as_tag(&self) -> &'static str {
		match self {
",
	);

	for spec in specs {
		out.push_str(&format!(
			"			{} => \"{}\",
",
			spec.ident, spec.tag
		));
	}

	out.push_str(
		"		}
	}

	/// Parse a language tag.
	pub fn from_tag(tag: &str) -> Option<Self> {
		Some(match tag {
",
	);

	for spec in specs {
		out.push_str(&format!(
			"			\"{}\" => {},
",
			spec.tag, spec.ident
		));
	}

	out.push_str(
		"			_ => return None,
		})
	}

	/// Get the language name.
	pub fn as_str(&self) -> &str {
		match self {
",
	);

	for spec in specs {
		out.push_str(&format!(
			"			{} => \"{}\",
",
			spec.ident,
			escape(&spec.english)
		));
	}

	out.push_str(
		"		}
	}

	/// Get the language name in the language itself.
	pub fn as_local(&self) -> &'static str {
		match self {
",
	);

	for spec in specs {
		out.push_str(&format!(
			"			{} => \"{}\",
",
			spec.ident,
			escape(&spec.autonym)
		));
	}

	out.push_str(
		"		}
	}
}

#[cfg(feature = \"serde\")]
impl serde::Serialize for Language {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		serializer.serialize_str(self.as_tag())
	}
}

#[cfg(feature = \"serde\")]
impl<'de> serde::Deserialize<'de> for Language {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		let tag: String = serde::Deserialize::deserialize(deserializer)?;

		Language::from_tag(&tag).ok_or_else(|| serde::de::Error::unknown_variant(&tag, &[]))
	}
}
",
	);

	out
}
