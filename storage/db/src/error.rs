use lumen_common::api::external::ResourceType;
use lumen_common::api::external::error::Error as ApiError;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("No results in set")]
    NoResults { resource_type: ResourceType },

    #[error("There was a conflict with the current state of the database")]
    Conflict,

    #[error("Database pool error: {0}")]
    PoolError(#[from] lumen_common::db::pool::DbPoolError),

    #[error("Internal error: {0}")]
    Internal(anyhow::Error),
}

pub type StoreResult<T> = Result<T, StoreError>;

impl From<StoreError> for ApiError {
    fn from(err: StoreError) -> ApiError {
        match err {
            StoreError::NoResults { resource_type } => {
                ApiError::ObjectNotFound { type_name: resource_type }
            }
            StoreError::Conflict => ApiError::invalid_request(err.to_string()),
            StoreError::Internal(e) => ApiError::internal_error(e.to_string()),
            StoreError::PoolError(e) => ApiError::internal_error(e.to_string()),
        }
    }
}
