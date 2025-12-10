//! Typed BCP47 language tags with names, plural metadata, and conversion helpers to common
//! libraries.

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

#[cfg(any(feature = "icu_locale_core", feature = "sqlx", feature = "whatlang"))] pub mod error;
pub mod prelude {
	//! Common imports for consumers of this crate.

	#[cfg(any(feature = "icu_locale_core", feature = "sqlx", feature = "whatlang"))]
	pub use crate::error::{Error, Result};
	pub use crate::generated::*;
}

mod generated;

#[cfg(feature = "scraper")] use scraper as _;

#[cfg(feature = "icu_locale_core")] mod icu_locale_core;
#[cfg(feature = "sqlx")] mod sqlx;
#[cfg(feature = "whatlang")] mod whatlang;

#[cfg(test)] use serde_json as _;
