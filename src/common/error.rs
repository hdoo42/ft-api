use rsb_derive::Builder;
use std::error::Error;
use std::fmt::Formatter;
use std::time::Duration;
use url::ParseError;

use reqwest::StatusCode;

#[derive(Debug)]
pub enum FtClientError {
    ReqwestError(FtReqwestError),
    ApiError(FtClientApiError),
    HttpError(FtClientHttpError),
    HttpProtocolError(FtClientHttpProtocolError),
    EndOfStream(FtClientEndOfStreamError),
    SystemError(FtClientSystemError),
    ProtocolError(FtClientProtocolError),
    RateLimitError(FtRateLimitError),
}

impl From<FtReqwestError> for FtClientError {
    fn from(err: FtReqwestError) -> Self {
        FtClientError::ReqwestError(err)
    }
}

impl From<FtClientApiError> for FtClientError {
    fn from(err: FtClientApiError) -> Self {
        FtClientError::ApiError(err)
    }
}

impl From<FtClientHttpError> for FtClientError {
    fn from(err: FtClientHttpError) -> Self {
        FtClientError::HttpError(err)
    }
}

impl From<FtClientHttpProtocolError> for FtClientError {
    fn from(err: FtClientHttpProtocolError) -> Self {
        FtClientError::HttpProtocolError(err)
    }
}

impl From<FtClientEndOfStreamError> for FtClientError {
    fn from(err: FtClientEndOfStreamError) -> Self {
        FtClientError::EndOfStream(err)
    }
}

impl From<FtClientSystemError> for FtClientError {
    fn from(err: FtClientSystemError) -> Self {
        FtClientError::SystemError(err)
    }
}

impl From<FtClientProtocolError> for FtClientError {
    fn from(err: FtClientProtocolError) -> Self {
        FtClientError::ProtocolError(err)
    }
}

impl From<FtRateLimitError> for FtClientError {
    fn from(err: FtRateLimitError) -> Self {
        FtClientError::RateLimitError(err)
    }
}

impl FtClientError {
    fn option_to_string<T: ToString>(value: &Option<T>) -> String {
        value
            .as_ref()
            .map_or_else(|| "-".to_string(), |v| v.to_string())
    }
}

impl std::fmt::Display for FtClientError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            FtClientError::ReqwestError(ref err) => err.fmt(f),
            FtClientError::ApiError(ref err) => err.fmt(f),
            FtClientError::HttpError(ref err) => err.fmt(f),
            FtClientError::HttpProtocolError(ref err) => err.fmt(f),
            FtClientError::EndOfStream(ref err) => err.fmt(f),
            FtClientError::SystemError(ref err) => err.fmt(f),
            FtClientError::ProtocolError(ref err) => err.fmt(f),
            FtClientError::RateLimitError(ref err) => err.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct FtReqwestError {
    pub error: reqwest::Error,
}

impl std::fmt::Display for FtReqwestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reqwest Error: {}", self.error,)
    }
}

impl std::error::Error for FtReqwestError {}

#[derive(Debug, Builder)]
pub struct FtClientApiError {
    pub code: String,
    pub errors: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

impl std::fmt::Display for FtClientApiError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "42 API Error: {}\nBody: '{}'",
            self.code,
            FtClientError::option_to_string(&self.http_response_body)
        )
    }
}

impl std::error::Error for FtClientApiError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtClientHttpError {
    pub status_code: StatusCode,
    pub http_response_body: Option<String>,
}

impl std::fmt::Display for FtClientHttpError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ft HTTP error status: {}. Body: '{}'",
            self.status_code,
            FtClientError::option_to_string(&self.http_response_body)
        )
    }
}

impl std::error::Error for FtClientHttpError {}

#[derive(Debug, Builder)]
pub struct FtClientHttpProtocolError {
    pub cause: Option<Box<dyn std::error::Error + Sync + Send>>,
}

impl std::fmt::Display for FtClientHttpProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ft http protocol error: {:?}", self.cause)
    }
}

impl std::error::Error for FtClientHttpProtocolError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtClientEndOfStreamError {}

impl std::fmt::Display for FtClientEndOfStreamError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ft end of stream error")
    }
}

impl std::error::Error for FtClientEndOfStreamError {}

#[derive(Debug, Builder)]
pub struct FtClientProtocolError {
    pub json_error: serde_json::Error,
    pub json_body: Option<String>,
}

impl std::fmt::Display for FtClientProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ft JSON protocol error: {}. Body: '{}'",
            self.json_error,
            FtClientError::option_to_string(&self.json_body)
        )
    }
}

impl std::error::Error for FtClientProtocolError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtClientSocketModeProtocolError {
    pub message: String,
}

impl std::fmt::Display for FtClientSocketModeProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ft socket mode protocol error: {}", self.message)
    }
}

impl std::error::Error for FtClientSocketModeProtocolError {}

#[derive(Debug, Builder)]
pub struct FtClientSystemError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error + Sync + Send + 'static>>,
}

impl std::fmt::Display for FtClientSystemError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ft system protocol error. {}{:?}",
            self.message.as_ref().unwrap_or(&"".to_string()),
            self.cause
        )
    }
}

impl std::error::Error for FtClientSystemError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtRateLimitError {
    pub retry_after: Option<Duration>,
    pub code: Option<String>,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

impl std::fmt::Display for FtRateLimitError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ft API rate limit error: {}\nBody: '{}'. Retry after: `{:?}`",
            FtClientError::option_to_string(&self.code),
            FtClientError::option_to_string(&self.http_response_body),
            self.retry_after,
        )
    }
}

impl std::error::Error for FtRateLimitError {}

impl From<url::ParseError> for FtClientError {
    fn from(url_parse_error: ParseError) -> Self {
        FtClientError::HttpProtocolError(
            FtClientHttpProtocolError::new().with_cause(Box::new(url_parse_error)),
        )
    }
}

impl From<Box<dyn std::error::Error + Sync + Send>> for FtClientError {
    fn from(err: Box<dyn Error + Sync + Send>) -> Self {
        FtClientError::SystemError(FtClientSystemError::new().with_cause(err))
    }
}

pub fn map_serde_error(err: serde_json::Error, tried_to_parse: Option<&str>) -> FtClientError {
    FtClientError::ProtocolError(
        FtClientProtocolError::new(err).opt_json_body(tried_to_parse.map(|s| s.to_string())),
    )
}
