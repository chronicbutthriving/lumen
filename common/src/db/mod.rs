#[cfg(feature = "diesel")]
pub mod schema;

#[cfg(feature = "diesel")]
mod db_typed_uuid;
#[cfg(feature = "diesel")]
pub use db_typed_uuid::*;

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

#[cfg(test)]
mod tests {
    use super::*;

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
