# Spec

Normative behavior for the `language` crate.

## Source of truth

- Upstream dataset: translation.io `languages_with_plural_cases`.
- Generated output: `src/generated.rs`.
- Codegen: `build/codegen.rs` and `src/main.rs`.

## Public API contract

- `Language`: Generated enum for supported BCP47 tags.
- `Error`: Error type for parsing and interop.
- `Result<T>`: Alias for `std::result::Result<T, Error>`.

`Language` methods:

- `tag() -> &'static str`: Canonical tag.
- `name() -> &str`: English name.
- `local_name() -> &'static str`: Autonym.
- `all() -> [Language; N]`: All supported tags.

Parsing:

- `TryFrom` expects canonical tag format.
- No case or separator normalization.
- Unknown tags return `Error::UnsupportedLanguageTag`.

## Data pipeline rules

Extraction:

- Read the first four cells as tag, English name, region name, autonym.
- Use the region name when present.
- Skip rows with missing fields.
- Skip tags containing `system`.

Normalization:

- Collapse and trim whitespace.
- Remove control characters and soft hyphens.

Tag validation:

- ASCII only.
- Primary language: 2-3 alphabetic characters.
- Optional script: 4 alphabetic characters.
- Optional region: 2 alphabetic or 3 numeric characters.
- Optional variants: 4-8 alphanumeric characters.
- Reject extensions and private-use subtags.

Output:

- Write `src/generated.rs`.
- Do not edit the file by hand.

## Interop contracts

`serde`:

- Serialize to the canonical tag string.
- Deserialize from a string tag only.
- Unknown tags fail deserialization.

`icu_locale_core`:

- Convert via tag strings.
- Unsupported mappings return `Error::UnsupportedIcuLocale`.

`whatlang`:

- Use only the base language subtag.
- `Language::Fil` maps to `whatlang::Lang::Tgl`.
- Unsupported values return the relevant `Error` variant.

`sqlx-*`:

- Encode and decode as text tags.
- Implementations exist only for enabled backends.
