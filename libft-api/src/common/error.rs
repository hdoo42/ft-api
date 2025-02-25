use rsb_derive::Builder;
use std::error::Error;
use std::fmt::Formatter;
use std::time::Duration;
use url::ParseError;

use reqwest::StatusCode;

#[macro_export]
macro_rules! enum_into {
	($vis:vis $enum_ty:ident $($enum_item:ident $(,)?)*) => {
		#[derive(Debug)]
		$vis enum $enum_ty {
			$($enum_item(concat_idents!(Ft,$enum_item))),*
		}

		$(impl From<concat_idents!(Ft,$enum_item)> for $enum_ty {
			fn from(err: concat_idents!(Ft,$enum_item)) -> Self {
				$enum_ty::$enum_item(err)
			}
		})*
	};
}

enum_into!(pub FtClientError
    ReqwestError
    ApiError
    HttpError
    HttpProtocolError
    EndOfStream
    SystemError
    ProtocolError
    RateLimitError
);

impl FtClientError {
    fn option_to_string<T: ToString>(value: &Option<T>) -> String {
        value
            .as_ref()
            .map_or_else(|| "-".to_string(), std::string::ToString::to_string)
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
pub struct FtApiError {
    pub code: String,
    pub errors: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

impl std::fmt::Display for FtApiError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "42 API Error: {}\nBody: '{}'",
            self.code,
            FtClientError::option_to_string(&self.http_response_body)
        )
    }
}

impl std::error::Error for FtApiError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtHttpError {
    pub status_code: StatusCode,
    pub http_response_body: Option<String>,
}

impl std::fmt::Display for FtHttpError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ft HTTP error status: {}. Body: '{}'",
            self.status_code,
            FtClientError::option_to_string(&self.http_response_body)
        )
    }
}

impl std::error::Error for FtHttpError {}

#[derive(Debug, Builder)]
pub struct FtHttpProtocolError {
    pub cause: Option<Box<dyn std::error::Error + Sync + Send>>,
}

impl std::fmt::Display for FtHttpProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ft http protocol error: {:?}", self.cause)
    }
}

impl std::error::Error for FtHttpProtocolError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtEndOfStream {}

impl std::fmt::Display for FtEndOfStream {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ft end of stream error")
    }
}

impl std::error::Error for FtEndOfStream {}

#[derive(Debug, Builder)]
pub struct FtProtocolError {
    pub json_error: serde_json::Error,
    pub json_body: Option<String>,
}

impl std::fmt::Display for FtProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ft JSON protocol error: {}. Body: '{}'",
            self.json_error,
            FtClientError::option_to_string(&self.json_body)
        )
    }
}

impl std::error::Error for FtProtocolError {}

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
pub struct FtSystemError {
    pub message: Option<String>,
    pub cause: Option<Box<dyn std::error::Error + Sync + Send + 'static>>,
}

impl std::fmt::Display for FtSystemError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ft system protocol error. {}{:?}",
            self.message.as_ref().unwrap_or(&String::new()),
            self.cause
        )
    }
}

impl std::error::Error for FtSystemError {}

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
            FtHttpProtocolError::new().with_cause(Box::new(url_parse_error)),
        )
    }
}

impl From<Box<dyn std::error::Error + Sync + Send>> for FtClientError {
    fn from(err: Box<dyn Error + Sync + Send>) -> Self {
        FtClientError::SystemError(FtSystemError::new().with_cause(err))
    }
}

pub fn map_serde_error(err: serde_json::Error, tried_to_parse: Option<&str>) -> FtClientError {
    FtClientError::ProtocolError(
        FtProtocolError::new(err)
            .opt_json_body(tried_to_parse.map(std::string::ToString::to_string)),
    )
}
