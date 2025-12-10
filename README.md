<div align="center">

# language

Typed BCP47 language tags with names, plural metadata, and conversion helpers to common libraries.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Docs](https://img.shields.io/docsrs/language)](https://docs.rs/language)
[![Rust](https://github.com/hack-ink/language/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/hack-ink/language/actions/workflows/rust.yml)
[![Release](https://github.com/hack-ink/language/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/language/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/language)](https://github.com/hack-ink/language/tags)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/language?color=red&style=plastic)](https://github.com/hack-ink/language)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/language)](https://github.com/hack-ink/language)

</div>

## Introduction

Language tags are defined in [BCP47](http://tools.ietf.org/html/bcp47). A friendly overview is available in the W3C article “[Language tags in HTML and XML](http://www.w3.org/International/articles/language-tags/).” These tags are commonly used in HTML and in the `Content-Language` and `Accept-Language` HTTP headers.

## Feature Highlights

- Typed coverage of BCP47 language tags through a single `Language` enum.
- Conversion helpers: `tag`, `name`, and `local_name` give tags, English names, and native names, with `FromStr`/`TryFrom` for parsing.
- `Language::all()` provides a compile-time array for iterating over every language without allocation.
- Optional `serde` feature for serializing and deserializing language values.
- Code is generated directly from the translation.io “languages with plural cases” page; `cargo build` enforces validity and the `language` binary downloads fresh data when regenerating.
- Optional ICU4X interop (`icu_locale_core` feature) for converting to/from `Locale` and `LanguageIdentifier`.
- Optional whatlang interop (`whatlang` feature) for converting to/from `whatlang::Lang` with clear error reporting.
- Optional SQLx interop (`sqlx-postgres` / `sqlx-mysql` / `sqlx-sqlite`) for `Type`/`Encode`/`Decode` support using textual tags.

## Usage

```rust
// crates.io
use language::Language;

let en = Language::En;

assert_eq!(en.tag(), "en");
assert_eq!("en".parse::<Language>().unwrap(), Language::En);
assert_eq!(en.name(), "English");
assert_eq!(en.local_name(), "English");
```

### More examples

Parse user input safely (any invalid tag becomes a typed error):

```rust
// crates.io
use language::Language;

let parsed: Result<Language, _> = "zh-Hant".parse();

assert!(parsed.is_ok());

let bad = "zz-INVALID".parse::<Language>();

assert!(bad.is_err());
```

Iterate over every language without allocation:

```rust
// crates.io
use language::Language;

let tags: Vec<&'static str> = Language::all().iter().map(Language::tag).collect();

assert!(tags.contains(&"fr-CA"));
```

`serde` (enable the `serde` feature):

```rust
// crates.io
use language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Payload {
	lang: Language,
}

let json = r#"{"lang":"es-MX"}"#;
let payload: Payload = serde_json::from_str(json)?;

assert_eq!(payload.lang, Language::EsMx);
```

ICU4X interop (enable `icu_locale_core`):

```rust
// crates.io
use icu_locale_core::Locale;
use language::Language;

let locale: Locale = "pt-BR".parse()?;
let language = Language::try_from(&locale)?;

assert_eq!(language.tag(), "pt-BR");
```

whatlang interop (enable `whatlang`):

```rust
// crates.io
use language::Language;
use whatlang::Lang;

let lang = Language::try_from(Lang::Ukr)?;

assert_eq!(lang, Language::Uk);
```

SQLx (enable one of the `sqlx-*` features):

```rust
// crates.io
use language::Language;
use sqlx::types::Json;

// Language stores as a text tag; works with Postgres/MySQL/SQLite feature flags.
let stored = Language::Ja;
let row = sqlx::query!("select $1 as lang", stored)
	.fetch_one(&pool)
	.await?;

assert_eq!(row.lang, Language::Ja);
```

## Support Me

If you find this project helpful and would like to support its development, you can buy me a coffee!

Your support is greatly appreciated and motivates me to keep improving this project.

- **Fiat**
    - [Ko-fi](https://ko-fi.com/hack_ink)
    - [爱发电](https://afdian.com/a/hack_ink)
- **Crypto**
    - **Bitcoin**
        - `bc1pedlrf67ss52md29qqkzr2avma6ghyrt4jx9ecp9457qsl75x247sqcp43c`
    - **Ethereum**
        - `0x3e25247CfF03F99a7D83b28F207112234feE73a6`
    - **Polkadot**
        - `156HGo9setPcU2qhFMVWLkcmtCEGySLwNqa3DaEiYSWtte4Y`

Thank you for your support!

## Appreciation

We would like to extend our heartfelt gratitude to the following projects and contributors:

- The Rust community for their continuous support and development of the Rust ecosystem.
- translation.io for publishing the languages-with-plural-cases reference we build from.

## Additional Acknowledgements

- Autonym data is downloaded from <https://translation.io/docs/languages_with_plural_cases> whenever you regenerate code with `cargo run --features codegen --bin language`.

<div align="right">

#### License

<sup>Licensed under [GPL-3.0](LICENSE).</sup>

</div>
