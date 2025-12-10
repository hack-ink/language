//! Typed BCP47 language tags with names, plural metadata, and conversion helpers to common
//! libraries.

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

pub mod error;
pub mod prelude {
	#![allow(missing_docs)]

	pub use crate::{
		error::{Error, Result},
		generated::*,
	};
}

mod generated;

#[cfg(feature = "scraper")] use scraper as _;

#[cfg(feature = "icu_locale_core")] mod icu_locale_core;
#[cfg(feature = "sqlx")] mod sqlx;
#[cfg(feature = "whatlang")] mod whatlang;

#[cfg(test)] use serde_json as _;
