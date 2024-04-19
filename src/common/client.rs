use futures::{future::BoxFuture, FutureExt};
use lazy_static::lazy_static;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::{error::Error, time::Duration};
use std::{fmt::Display, sync::Arc};
use url::{ParseError, Url};

use reqwest::StatusCode;
use util::config_env_var;

use crate::{AccessToken, AuthInfo};

#[derive(Clone, Debug)]
pub struct FtClient<FCHC>
where
    FCHC: FtClientHttpConnector + Send,
{
    pub http_api: FtClientHttpApi<FCHC>,
}

#[derive(Clone, Debug)]
pub struct FtClientHttpApi<FCHC>
where
    FCHC: FtClientHttpConnector + Send,
{
    pub connector: Arc<FCHC>,
}

impl<FCHC> FtClientHttpApi<FCHC>
where
    FCHC: FtClientHttpConnector + Send,
{
    pub fn new(http_connector: Arc<FCHC>) -> Self {
        Self {
            connector: http_connector,
        }
    }
}

pub type ClientResult<T> = std::result::Result<T, FtClientError>;

impl<FCHC> FtClient<FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub fn new(http_connector: FCHC) -> Self {
        Self {
            http_api: FtClientHttpApi::new(Arc::new(http_connector)),
        }
    }
}

pub struct FtApiClientHttpSessionApi<FCHC>
where
    FCHC: FtClientHttpConnector + Send,
{
    token: AccessToken,
    pub client: FtClient<FCHC>,
}

impl<FCHC> FtApiClientHttpSessionApi<FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn new(http_connector: FCHC) -> Self {
        let token = AccessToken::try_get(AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").unwrap(),
            config_env_var("FT_API_CLIENT_SECRET").unwrap(),
        ))
        .await
        .unwrap();

        let client = FtClient::new(http_connector);

        FtApiClientHttpSessionApi { token, client }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FtEnvelopeMessage {
    pub ok: bool,
    pub error: Option<String>,
    // Slack may return validation errors in `errors` field with `ok: false` for some methods (such as `apps.manifest.validate`.
    pub errors: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
}

pub trait FtClientHttpConnector {
    fn http_get_uri<'a, RS>(
        &'a self,
        full_uri: Url,
        token: &'a AccessToken,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + 'a + Send;

    fn http_get<'a, 'p, RS, PT, TS>(
        &'a self,
        method_relative_uri: &str,
        token: &'a AccessToken,
        params: &'p PT,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<TS>)> + Clone,
        TS: AsRef<str> + 'p + Send,
    {
        let full_uri = self
            .create_method_uri_path(method_relative_uri)
            .and_then(|url| FtClientHttpApiUri::create_url_with_params(url, params));

        match full_uri {
            Ok(full_uri) => self.http_get_uri(full_uri, token),
            Err(err) => std::future::ready(Err(err)).boxed(),
        }
    }

    fn http_post_uri<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        token: &'a AccessToken,
        request_body: &'a RQ,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a;

    fn http_post<'a, RQ, RS>(
        &'a self,
        method_relative_uri: &str,
        token: &'a AccessToken,
        request: &'a RQ,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
    {
        match self.create_method_uri_path(method_relative_uri) {
            Ok(full_uri) => self.http_post_uri(full_uri, token, request),
            Err(err) => std::future::ready(Err(err)).boxed(),
        }
    }

    fn create_method_uri_path(&self, method_relative_uri: &str) -> ClientResult<Url> {
        Ok(FtClientHttpApiUri::create_method_uri_path(method_relative_uri).parse()?)
    }
}

lazy_static! {
    pub static ref FT_HTTP_EMPTY_GET_PARAMS: Vec<(&'static str, Option<&'static String>)> = vec![];
}

pub struct FtClientHttpApiUri;

impl FtClientHttpApiUri {
    pub const FT_API_URI_STR: &'static str = "https://api.intra.42.fr";

    pub fn create_method_uri_path(method_relative_uri: &str) -> String {
        format!("{}/{}", Self::FT_API_URI_STR, method_relative_uri)
    }

    pub fn create_url_with_params<'p, PT, TS>(base_url: Url, params: &'p PT) -> ClientResult<Url>
    where
        PT: std::iter::IntoIterator<Item = (&'p str, Option<TS>)> + Clone,
        TS: AsRef<str> + 'p,
    {
        let url_query_params: Vec<(String, String)> = params
            .clone()
            .into_iter()
            .filter_map(|(k, vo)| vo.map(|v| (k.to_string(), v.as_ref().to_string())))
            .collect();

        Ok(Url::parse_with_params(base_url.as_str(), url_query_params)?)
    }
}

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

impl Display for FtClientError {
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

impl Display for FtReqwestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reqwest Error: {}", self.error.to_string(),)
    }
}

impl Error for FtReqwestError {}

#[derive(Debug, Builder)]
pub struct FtClientApiError {
    pub code: String,
    pub errors: Option<Vec<String>>,
    pub warnings: Option<Vec<String>>,
    pub http_response_body: Option<String>,
}

impl Display for FtClientApiError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "42 API Error: {}\nBody: '{}'",
            self.code,
            FtClientError::option_to_string(&self.http_response_body)
        )
    }
}

impl Error for FtClientApiError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtClientHttpError {
    pub status_code: StatusCode,
    pub http_response_body: Option<String>,
}

impl Display for FtClientHttpError {
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

impl Display for FtClientHttpProtocolError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Ft http protocol error: {:?}", self.cause)
    }
}

impl std::error::Error for FtClientHttpProtocolError {}

#[derive(Debug, PartialEq, Eq, Clone, Builder)]
pub struct FtClientEndOfStreamError {}

impl Display for FtClientEndOfStreamError {
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

impl Display for FtClientProtocolError {
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

impl Display for FtClientSocketModeProtocolError {
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

impl Display for FtClientSystemError {
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

impl Display for FtRateLimitError {
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

impl Error for FtRateLimitError {}

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
