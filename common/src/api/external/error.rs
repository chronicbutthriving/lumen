use std::fmt::Display;

use dropshot::HttpError;
use serde::{Deserialize, Serialize};

use crate::api::external::ResourceType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An object needed as part of this operation was not found.
    #[error("Object not found: {type_name}")]
    ObjectNotFound { type_name: ResourceType },
    /// An object already exists with the specified name or identifier.
    #[error("Object (of type {type_name:?}) already exists")]
    ObjectAlreadyExists { type_name: ResourceType },
    /// The request was well-formed, but the operation cannot be completed given
    /// the current state of the system.
    #[error("Invalid request: {}", .message.display_internal())]
    InvalidRequest { message: MessagePair },
    /// Authentication credentials were required but either missing or invalid.
    /// The HTTP status code is called "Unauthorized", but it's more accurate to
    /// call it "Unauthenticated".
    #[error("Missing or invalid credentials")]
    Unauthenticated { internal_message: String },
    /// The specified input field is not valid.
    #[error("Invalid Value: {label}, {}", .message.display_internal())]
    InvalidValue { label: String, message: MessagePair },
    /// The request is not authorized to perform the requsted operation.
    #[error("Forbidden")]
    Forbidden,

    /// The system encountered an unhandled operational error.
    #[error("Internal Error: {internal_message}")]
    InternalError { internal_message: String },
    /// The system (or part of it) is unavailable.
    #[error("Service Unavailable: {internal_message}")]
    ServiceUnavailable { internal_message: String },

    /// A generic 404 response. If there is an applicable ResourceType, use
    /// ObjectNotFound instead.
    #[error("Not Found: {}", .message.display_internal())]
    NotFound { message: MessagePair },
}

/// Represents an error message which has an external component, along with
/// some internal context possibly attached to it.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct MessagePair {
    external_message: String,
    internal_context: String,
}

impl MessagePair {
    pub fn new(external_message: String) -> Self {
        Self { external_message, internal_context: String::new() }
    }

    pub fn new_full(
        external_message: String,
        internal_context: String,
    ) -> Self {
        Self { external_message, internal_context }
    }

    pub fn external_message(&self) -> &str {
        &self.external_message
    }

    pub fn internal_context(&self) -> &str {
        &self.internal_context
    }

    // fn with_internal_context<C>(self, context: C) -> Self
    // where
    //     C: Display + Send + Sync + 'static,
    // {
    //     let internal_context = if self.internal_context.is_empty() {
    //         context.to_string()
    //     } else {
    //         format!("{}: {}", context, self.internal_context)
    //     };
    //     Self { external_message: self.external_message, internal_context }
    // }

    pub fn into_internal_external(self) -> (String, String) {
        let internal = self.display_internal().to_string();
        (internal, self.external_message)
    }

    fn display_internal(&self) -> MessagePairDisplayInternal<'_> {
        MessagePairDisplayInternal(self)
    }
}


struct MessagePairDisplayInternal<'a>(&'a MessagePair);

impl Display for MessagePairDisplayInternal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.external_message)?;
        if !self.0.internal_context.is_empty() {
            write!(f, " (with internal context: {})", self.0.internal_context)?;
        }
        Ok(())
    }
}

impl Error {
    /// Returns whether the error is likely transient and could reasonably be
    /// retried
    pub fn retryable(&self) -> bool {
        match self {
            Error::ServiceUnavailable { .. } => true,

            Error::ObjectNotFound { .. }
            | Error::ObjectAlreadyExists { .. }
            | Error::Unauthenticated { .. }
            | Error::InvalidRequest { .. }
            | Error::InvalidValue { .. }
            | Error::Forbidden { .. }
            | Error::InternalError { .. }
            | Error::NotFound { .. } => false,
        }
    }

    /// Generates an [`Error::InternalError`] error with the specific message
    ///
    /// InternalError should be used for operational conditions that should not
    /// happen but that we cannot reasonably handle at runtime (e.g.,
    /// deserializing a value from the database, or finding two records for
    /// something that is supposed to be unique).
    pub fn internal_error(internal_message: impl Into<String>) -> Error {
        Error::InternalError { internal_message: internal_message.into() }
    }

    /// Generates an [`Error::InvalidRequest`] error with the specific message
    ///
    /// This should be used for failures due possibly to invalid client input
    /// or malformed requests.
    pub fn invalid_request(message: impl Into<String>) -> Error {
        Error::InvalidRequest { message: MessagePair::new(message.into()) }
    }

    /// Generates an [`Error::InvalidValue`] error with the specific label and
    /// message
    pub fn invalid_value(
        label: impl Into<String>,
        message: impl Into<String>,
    ) -> Error {
        Error::InvalidValue {
            label: label.into(),
            message: MessagePair::new(message.into()),
        }
    }

    /// Generates an [`Error::ServiceUnavailable`] error with the specific
    /// message
    ///
    /// This should be used for transient failures where the caller might be
    /// expected to retry. Logic errors or other problems indicating that a
    /// retry might not work should probably be an InternalError (if it's a
    /// server problem) or InvalidRequest (if it's a client problem) instead.
    pub fn unavail(message: &str) -> Error {
        Error::ServiceUnavailable { internal_message: message.to_owned() }
    }
}

impl From<Error> for HttpError {
    fn from(error: Error) -> HttpError {
        match error {
            Error::ObjectNotFound { type_name } => {
                HttpError::for_client_error(
                    Some(String::from("ObjectNotFound")),
                    dropshot::ClientErrorStatusCode::NOT_FOUND,
                    format!("not found: {}", type_name.to_string()),
                )
            },

            Error::ObjectAlreadyExists { type_name } => {
                HttpError::for_bad_request(
                    Some(String::from("ObjectAlreadyExists")),
                    format!("object already exists: {}", type_name.to_string()),
                )
            },

            Error::Unauthenticated { internal_message } => HttpError {
                status_code: dropshot::ErrorStatusCode::UNAUTHORIZED,
                error_code: Some(String::from("Unauthorized")),
                external_message: String::from(
                    "credentials invalid or missing",
                ),
                internal_message,
                headers: None,
            },

            Error::InvalidRequest { message } => {
                let (internal_message, external_message) =
                    message.into_internal_external();
                HttpError {
                    status_code: dropshot::ErrorStatusCode::BAD_REQUEST,
                    error_code: Some(String::from("InvalidRequest")),
                    external_message,
                    internal_message,
                    headers: None,
                }
            }

            Error::InvalidValue { label, message } => {
                let (internal_message, external_message) =
                    message.into_internal_external();
                HttpError {
                    status_code: dropshot::ErrorStatusCode::BAD_REQUEST,
                    error_code: Some(String::from("InvalidValue")),
                    external_message: format!(
                        "unsupported value for \"{}\": {}",
                        label, external_message
                    ),
                    internal_message,
                    headers: None,
                }
            }

            Error::Forbidden => HttpError::for_client_error(
                Some(String::from("Forbidden")),
                dropshot::ClientErrorStatusCode::FORBIDDEN,
                String::from("Forbidden"),
            ),

            Error::InternalError { internal_message } =>
                HttpError::for_internal_error(internal_message),

            Error::ServiceUnavailable { internal_message } => {
                HttpError::for_unavail(
                    Some(String::from("ServiceNotAvailable")),
                    internal_message,
                )
            }

            Error::NotFound { message } => {
                let (internal_message, external_message) =
                    message.into_internal_external();
                HttpError {
                    status_code: dropshot::ErrorStatusCode::NOT_FOUND,
                    error_code: Some(String::from("Not Found")),
                    external_message,
                    internal_message,
                    headers: None,
                }
            }
        }
    }
}
