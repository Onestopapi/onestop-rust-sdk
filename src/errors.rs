/// Error returned by every fallible SDK call.
///
/// API failures display as `onestop: HTTP <status> (<requestId>): <message>`
/// (or without the request ID when the API sent none). Transport and local
/// errors have `status_code` 0 and display `message` unchanged.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{}", render(*.status_code, .request_id, .message))]
pub struct Error {
    /// HTTP status, or 0 for transport and local errors.
    pub status_code: u16,
    /// API request ID for support; empty when unavailable.
    pub request_id: String,
    /// API error message, or the full error text when `status_code` is 0.
    pub message: String,
}

impl Error {
    pub(crate) fn api(status_code: u16, request_id: &str, message: &str) -> Self {
        Self { status_code, request_id: request_id.to_string(), message: message.to_string() }
    }

    pub(crate) fn local(message: impl Into<String>) -> Self {
        Self { status_code: 0, request_id: String::new(), message: message.into() }
    }
}

fn render(status_code: u16, request_id: &str, message: &str) -> String {
    match (status_code, request_id) {
        (0, _) => message.to_string(),
        (_, "") => format!("onestop: HTTP {status_code}: {message}"),
        _ => format!("onestop: HTTP {status_code} ({request_id}): {message}"),
    }
}
