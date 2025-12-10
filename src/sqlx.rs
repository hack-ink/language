// crates.io
use sqlx::{
	Decode, Encode, Type,
	database::{HasArguments, HasValueRef},
	encode::IsNull,
	error::BoxDynError,
};
// self
use crate::prelude::*;

#[cfg(feature = "sqlx-postgres")]
impl Type<sqlx::Postgres> for Language {
	fn type_info() -> sqlx::postgres::PgTypeInfo {
		<&str as Type<sqlx::Postgres>>::type_info()
	}

	fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
		<&str as Type<sqlx::Postgres>>::compatible(ty)
	}
}

#[cfg(feature = "sqlx-postgres")]
impl sqlx::postgres::PgHasArrayType for Language {
	fn array_type_info() -> sqlx::postgres::PgTypeInfo {
		sqlx::postgres::PgTypeInfo::with_name("_text")
	}
}

#[cfg(feature = "sqlx-postgres")]
impl<'q> Encode<'q, sqlx::Postgres> for Language {
	fn encode_by_ref(
		&self,
		buf: &mut <sqlx::Postgres as HasArguments<'q>>::ArgumentBuffer,
	) -> IsNull {
		let tag = self.as_tag();

		<&str as Encode<'q, sqlx::Postgres>>::encode(tag, buf)
	}
}

#[cfg(feature = "sqlx-postgres")]
impl<'r> Decode<'r, sqlx::Postgres> for Language {
	fn decode(value: <sqlx::Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
		let tag = <&str as Decode<'r, sqlx::Postgres>>::decode(value)?;

		Language::from_tag(tag).ok_or_else(|| format!("Invalid language tag `{tag}`.").into())
	}
}

#[cfg(feature = "sqlx-mysql")]
impl Type<sqlx::MySql> for Language {
	fn type_info() -> sqlx::mysql::MySqlTypeInfo {
		<&str as Type<sqlx::MySql>>::type_info()
	}

	fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
		<&str as Type<sqlx::MySql>>::compatible(ty)
	}
}

#[cfg(feature = "sqlx-mysql")]
impl<'q> Encode<'q, sqlx::MySql> for Language {
	fn encode_by_ref(&self, buf: &mut <sqlx::MySql as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
		let tag = self.as_tag();

		<&str as Encode<'q, sqlx::MySql>>::encode(tag, buf)
	}
}

#[cfg(feature = "sqlx-mysql")]
impl<'r> Decode<'r, sqlx::MySql> for Language {
	fn decode(value: <sqlx::MySql as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
		let tag = <&str as Decode<'r, sqlx::MySql>>::decode(value)?;

		Language::from_tag(tag).ok_or_else(|| format!("Invalid language tag `{tag}`.").into())
	}
}

#[cfg(feature = "sqlx-sqlite")]
impl Type<sqlx::Sqlite> for Language {
	fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
		<&str as Type<sqlx::Sqlite>>::type_info()
	}

	fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
		<&str as Type<sqlx::Sqlite>>::compatible(ty)
	}
}

#[cfg(feature = "sqlx-sqlite")]
impl<'q> Encode<'q, sqlx::Sqlite> for Language {
	fn encode_by_ref(
		&self,
		buf: &mut <sqlx::Sqlite as HasArguments<'q>>::ArgumentBuffer,
	) -> IsNull {
		let tag = self.as_tag();

		<&str as Encode<'q, sqlx::Sqlite>>::encode(tag, buf)
	}
}

#[cfg(feature = "sqlx-sqlite")]
impl<'r> Decode<'r, sqlx::Sqlite> for Language {
	fn decode(value: <sqlx::Sqlite as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
		let tag = <&str as Decode<'r, sqlx::Sqlite>>::decode(value)?;

		Language::from_tag(tag).ok_or_else(|| format!("Invalid language tag `{tag}`.").into())
	}
}
