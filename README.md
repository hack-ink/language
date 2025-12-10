<div align="center">

# language

Typed BCP47 language tags with names, plural metadata, and conversion helpers to common libraries.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Docs](https://img.shields.io/docsrs/language)](https://docs.rs/language)
[![Checks](https://github.com/hack-ink/language/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/language/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/language/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/language/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/language)](https://github.com/hack-ink/language/tags)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/language?color=red&style=plastic)](https://github.com/hack-ink/language)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/language)](https://github.com/hack-ink/language)

</div>

## Introduction

Language tags are defined in [BCP47](http://tools.ietf.org/html/bcp47). A friendly overview is available in the W3C article “[Language tags in HTML and XML](http://www.w3.org/International/articles/language-tags/).” These tags are commonly used in HTML and in the `Content-Language` and `Accept-Language` HTTP headers.

## Feature Highlights

- Typed coverage of BCP47 language tags through a single `Language` enum.
- Conversion helpers: `as_tag`, `from_tag`, `as_str`, and `as_local` give tags, English names, and native names.
- `Language::all()` provides a compile-time array for iterating over every language without allocation.
- Optional `serde` feature for serializing and deserializing language values.
- Code is generated from a Translation.io “languages with plural cases” snapshot stored in `data/`; `cargo build` enforces validity and the `language` binary can refresh/regenerate while keeping offline builds working.
- Optional ICU4X interop (`icu_locale_core` feature) for converting to/from `Locale` and `LanguageIdentifier`.
- Optional whatlang interop (`whatlang` feature) for converting to/from `whatlang::Lang` with clear error reporting.
- Optional SQLx interop (`sqlx-postgres` / `sqlx-mysql` / `sqlx-sqlite`) for `Type`/`Encode`/`Decode` support using textual tags.

## Usage

```rust
// crates.io
use language::Language;

let en = Language::En;

assert_eq!(en.as_tag(), "en");
assert_eq!(Language::from_tag("en"), Some(Language::En));
assert_eq!(en.as_str(), "English");
assert_eq!(en.as_local(), "English");
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
- Translation.io for publishing the languages-with-plural-cases reference we build from.

## Additional Acknowledgements

- Autonym data is sourced from `data/languages_with_plural_cases.html`; refresh it with `cargo run --features codegen --bin language -- --fetch` when updating generated code.

<div align="right">

#### License

<sup>Licensed under [GPL-3.0](LICENSE).</sup>

</div>
