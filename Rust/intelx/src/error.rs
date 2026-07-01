//! Error types returned by every fallible operation in this crate.

use thiserror::Error;

/// The crate-wide result alias: every public, fallible function in `intelx`
/// returns `Result<T>` rather than Python's mixed bool/int/status-code return values.
pub type Result<T> = std::result::Result<T, IntelXError>;

/// Errors that can occur while talking to the Intelligence X API.
#[derive(Debug, Error)]
pub enum IntelXError {
    /// The underlying HTTP transport failed (connection, TLS, etc).
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// A response body could not be deserialized as the expected JSON shape.
    #[error("JSON (de)serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// A local file system operation failed (writing a downloaded file, etc).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The configured base URL could not be parsed.
    #[error("invalid base URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// The API responded with a non-success HTTP status code.
    #[error("API error {status}: {message}")]
    Api {
        /// The HTTP status code returned by the server.
        status: u16,
        /// A human-readable description, see [`describe_status`].
        message: String,
    },

    /// `/intelligent/search` (or `/phonebook/search`) rejected the search term
    /// as not being a supported "strong selector" (status == 1 in the API).
    #[error("invalid search term: the API rejected this term as not a supported selector")]
    InvalidTerm,

    /// The account has reached its maximum number of concurrent searches.
    #[error("maximum concurrent searches reached")]
    MaxConcurrentSearches,

    /// The given search id is unknown to the API (it may have expired or already finished).
    #[error("search id {0} not found")]
    SearchNotFound(uuid::Uuid),

    /// A file download response had no usable filename in its `Content-Disposition` header.
    #[error("download response had no usable filename in the Content-Disposition header")]
    MissingFilename,

    /// `IntelXClientBuilder::build` was called without an API key.
    #[error("an API key is required: call `.api_key(..)` on the builder before `.build()`")]
    MissingApiKey,

    /// A polling/search operation exceeded a caller-supplied deadline.
    #[error("operation timed out")]
    Timeout(#[from] tokio::time::error::Elapsed),
}

/// Maps an HTTP/API status code to a short human-readable description, mirroring the table
/// used by the Python SDK's `intelx.get_error()`.
pub fn describe_status(code: u16) -> &'static str {
    match code {
        200 => "200 | Success",
        204 => "204 | No Content",
        400 => "400 | Bad Request",
        401 => "401 | Unauthorized",
        402 => "402 | Payment required",
        404 => "404 | Not Found",
        1 => "1 | Invalid term",
        _ => "Unknown status code",
    }
}

pub(crate) fn api_error_from_status(status: reqwest::StatusCode) -> IntelXError {
    IntelXError::Api {
        status: status.as_u16(),
        message: describe_status(status.as_u16()).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_status_matches_known_codes() {
        assert_eq!(describe_status(200), "200 | Success");
        assert_eq!(describe_status(204), "204 | No Content");
        assert_eq!(describe_status(400), "400 | Bad Request");
        assert_eq!(describe_status(401), "401 | Unauthorized");
        assert_eq!(describe_status(402), "402 | Payment required");
        assert_eq!(describe_status(404), "404 | Not Found");
        assert_eq!(describe_status(1), "1 | Invalid term");
    }

    #[test]
    fn describe_status_falls_back_for_unmapped_codes() {
        assert_eq!(describe_status(418), "Unknown status code");
    }

    #[test]
    fn api_error_from_status_carries_code_and_message() {
        let err = api_error_from_status(reqwest::StatusCode::UNAUTHORIZED);
        match err {
            IntelXError::Api { status, message } => {
                assert_eq!(status, 401);
                assert_eq!(message, "401 | Unauthorized");
            }
            other => panic!("expected Api variant, got {other:?}"),
        }
    }
}
