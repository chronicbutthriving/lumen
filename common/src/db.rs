#[cfg(feature = "diesel")]
use derive_where::derive_where;

#[cfg(feature = "diesel")]
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{self, ToSql},
    sql_types,
};

#[cfg(feature = "diesel")]
use iddqd::{Comparable, Equivalent};
#[cfg(feature = "diesel")]
use lumen_uuid_kinds::{GenericUuid, TypedUuid, TypedUuidKind};
#[cfg(feature = "diesel")]
use schemars::JsonSchema;
#[cfg(feature = "diesel")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "diesel")]
use std::fmt;
#[cfg(feature = "diesel")]
use std::str::FromStr;
#[cfg(feature = "diesel")]
use uuid::Uuid;

/// Pagination parameters for database queries.
pub struct PaginationParams {
    /// The maximum number of items to return.
    pub limit: Option<u32>,

    /// The number of items to skip before starting to collect the result set.
    pub offset: Option<u32>,
}

impl PaginationParams {
    /// Creates a new PaginationParams with the specified limit and offset.
    pub fn new(limit: Option<u32>, offset: Option<u32>) -> Self {
        Self { limit, offset }
    }

    /// Sets the limit for the number of items to return.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the offset for the number of items to skip.
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: Some(50),
            offset: Some(0),
        }
    }
}

/// Returns the corresponding `DbTypedUuid` for this `TypedUuid`.
///
/// Code external to the `db-model` crate sometimes needs a way to convert a
/// `TypedUuid` to a `DbTypedUuid`. We don't want `DbTypedUuid` to be used
/// anywhere, so we don't make it public. Instead, we expose this function.
#[cfg(feature = "diesel")]
#[inline]
pub fn to_db_typed_uuid<T: TypedUuidKind>(id: TypedUuid<T>) -> DbTypedUuid<T> {
    DbTypedUuid(id)
}

/// A UUID with information about the kind of type it is.
///
/// Despite the fact that this is marked `pub`, this is *private* to the
/// `db-model` crate (this type is not exported at the top level). External
/// users must use omicron-common's `TypedUuid`.
#[cfg(feature = "diesel")]
#[derive_where(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[derive(AsExpression, FromSqlRow, Serialize, Deserialize, JsonSchema)]
#[diesel(sql_type = sql_types::Uuid)]
#[serde(transparent, bound = "")]
pub struct DbTypedUuid<T: TypedUuidKind>(pub(crate) TypedUuid<T>);

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind, DB> ToSql<sql_types::Uuid, DB> for DbTypedUuid<T>
where
    DB: Backend,
    Uuid: ToSql<sql_types::Uuid, DB>,
{
    fn to_sql<'a>(
        &'a self,
        out: &mut serialize::Output<'a, '_, DB>,
    ) -> serialize::Result {
        self.0.as_untyped_uuid().to_sql(out)
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind, DB> FromSql<sql_types::Uuid, DB> for DbTypedUuid<T>
where
    DB: Backend,
    Uuid: FromSql<sql_types::Uuid, DB>,
{
    #[inline]
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let id = Uuid::from_sql(bytes)?;
        Ok(TypedUuid::from_untyped_uuid(id).into())
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> fmt::Debug for DbTypedUuid<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> fmt::Display for DbTypedUuid<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> FromStr for DbTypedUuid<T> {
    type Err = lumen_uuid_kinds::ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TypedUuid::from_str(s)?.into())
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> From<TypedUuid<T>> for DbTypedUuid<T> {
    #[inline]
    fn from(id: TypedUuid<T>) -> Self {
        Self(id)
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> From<DbTypedUuid<T>> for TypedUuid<T> {
    #[inline]
    fn from(id: DbTypedUuid<T>) -> Self {
        id.0
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> GenericUuid for DbTypedUuid<T> {
    #[inline]
    fn from_untyped_uuid(uuid: Uuid) -> Self {
        TypedUuid::from_untyped_uuid(uuid).into()
    }

    #[inline]
    fn into_untyped_uuid(self) -> Uuid {
        self.0.into_untyped_uuid()
    }

    #[inline]
    fn as_untyped_uuid(&self) -> &Uuid {
        self.0.as_untyped_uuid()
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> Equivalent<TypedUuid<T>> for DbTypedUuid<T> {
    #[inline]
    fn equivalent(&self, other: &TypedUuid<T>) -> bool {
        self.0.as_untyped_uuid() == other.as_untyped_uuid()
    }
}

#[cfg(feature = "diesel")]
impl<T: TypedUuidKind> Comparable<TypedUuid<T>> for DbTypedUuid<T> {
    #[inline]
    fn compare(&self, key: &TypedUuid<T>) -> std::cmp::Ordering {
        self.0.as_untyped_uuid().cmp(key.as_untyped_uuid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "diesel")]
    use lumen_uuid_kinds::ObjectUuid;

    #[cfg(feature = "diesel")]
    use std::hash::{BuildHasher, RandomState};

    #[cfg(feature = "diesel")]
    use test_strategy::proptest;

    /// Test that the `Hash` implementation is consistent, as required by
    /// `Equivalent`.
    #[cfg(feature = "diesel")]
    #[proptest]
    fn test_hash_equality(id: ObjectUuid) {
        let db_id = DbTypedUuid::from(id);
        assert!(db_id.equivalent(&id));

        let hasher = RandomState::new();
        let id_hash = hasher.hash_one(&id);
        let db_id_hash = hasher.hash_one(&db_id);
        assert_eq!(id_hash, db_id_hash);
    }

    /// Test that the `compare` implementation is consistent, as required by
    /// `Comparable`.
    #[cfg(feature = "diesel")]
    #[proptest]
    fn test_compare_consistency(id1: ObjectUuid, id2: ObjectUuid) {
        let db_id1 = DbTypedUuid::from(id1);
        assert_eq!(db_id1.compare(&id2), id1.cmp(&id2));
    }

    #[test]
    fn test_with_limit_sets_limit() {
        let params = PaginationParams::default().with_limit(100);
        assert_eq!(params.limit, Some(100));
    }

    #[test]
    fn test_with_offset_sets_offset() {
        let params = PaginationParams::default().with_offset(10);
        assert_eq!(params.offset, Some(10));
    }
}
