use crate::sql_types::{self, is_nullable, SqlType};

/// Marker trait for types which can be used with `MAX` and `MIN`
pub trait SqlOrd: SqlType {}

impl SqlOrd for sql_types::SmallInt {}
impl SqlOrd for sql_types::Integer {}
impl SqlOrd for sql_types::BigInt {}
impl SqlOrd for sql_types::Float {}
impl SqlOrd for sql_types::Double {}
impl SqlOrd for sql_types::Text {}
impl SqlOrd for sql_types::Date {}
impl SqlOrd for sql_types::Interval {}
impl SqlOrd for sql_types::Time {}
impl SqlOrd for sql_types::Timestamp {}
impl<T> SqlOrd for sql_types::Nullable<T> where T: SqlOrd + SqlType<IsNull = is_nullable::NotNull> {}

#[cfg(feature = "postgres")]
impl SqlOrd for sql_types::Timestamptz {}
#[cfg(feature = "postgres")]
impl<T: SqlOrd> SqlOrd for sql_types::Array<T> {}
#[cfg(all(feature = "postgres", feature = "uuid"))]
impl SqlOrd for sql_types::Uuid {}

#[cfg(feature = "mysql")]
impl SqlOrd for sql_types::Datetime {}
#[cfg(feature = "mysql")]
impl SqlOrd for sql_types::Unsigned<sql_types::SmallInt> {}
#[cfg(feature = "mysql")]
impl SqlOrd for sql_types::Unsigned<sql_types::Integer> {}
#[cfg(feature = "mysql")]
impl SqlOrd for sql_types::Unsigned<sql_types::BigInt> {}
