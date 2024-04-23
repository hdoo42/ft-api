use futures::{future::BoxFuture, FutureExt};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use url::Url;

use crate::{FtApiToken, FtClientError, FtClientReqwestConnector};

pub type ClientResult<T> = std::result::Result<T, FtClientError>;

pub type FtReqwestClient = FtClient<FtClientReqwestConnector>;

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

pub struct FtClientHttpApiUri;

#[derive(Debug)]
pub struct FtClientSession<'a, SCHC>
where
    SCHC: FtClientHttpConnector + Send,
{
    pub http_session_api: FtClientHttpSessionApi<'a, SCHC>,
}

#[derive(Debug)]
pub struct FtClientHttpSessionApi<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send,
{
    token: &'a FtApiToken,
    pub client: &'a FtClient<FCHC>,
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
        token: &'a FtApiToken,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + 'a + Send;

    fn http_get<'a, 'p, RS, PT, TS>(
        &'a self,
        method_relative_uri: &str,
        token: &'a FtApiToken,
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

        println!("{:?}", full_uri);
        match full_uri {
            Ok(full_uri) => self.http_get_uri(full_uri, token),
            Err(err) => std::future::ready(Err(err)).boxed(),
        }
    }

    fn http_post_uri<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        token: &'a FtApiToken,
        request_body: &'a RQ,
    ) -> BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a;

    fn http_post<'a, RQ, RS>(
        &'a self,
        method_relative_uri: &str,
        token: &'a FtApiToken,
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

impl<FCHC> FtClient<FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub fn new(http_connector: FCHC) -> Self {
        Self {
            http_api: FtClientHttpApi::new(Arc::new(http_connector)),
        }
    }

    pub fn open_session<'a>(&'a self, token: &'a FtApiToken) -> FtClientSession<'a, FCHC> {
        // TODO: Add tracer for LOGGING
        // let http_session_span = span!(Level::DEBUG, "Ft API request",);

        let http_session_api = FtClientHttpSessionApi {
            client: self,
            token,
        };

        FtClientSession { http_session_api }
    }
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

impl<'a, FCHC> FtClientHttpSessionApi<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn http_get_uri<RS, PT, TS>(&self, full_uri: Url) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        self.client
            .http_api
            .connector
            .http_get_uri(full_uri, self.token)
            .await
    }

    pub async fn http_get<'p, RS, PT, TS>(
        &self,
        method_relative_uri: &str,
        params: &'p PT,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send,
        PT: std::iter::IntoIterator<Item = (&'p str, Option<TS>)> + Clone,
        TS: AsRef<str> + 'p + Send,
    {
        self.client
            .http_api
            .connector
            .http_get(method_relative_uri, self.token, params)
            .await
    }

    pub async fn http_post<RQ, RS>(
        &self,
        method_relative_uri: &str,
        request: &RQ,
    ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        self.client
            .http_api
            .connector
            .http_post(method_relative_uri, self.token, request)
            .await
    }

    pub async fn http_post_uri<RQ, RS>(&self, full_uri: Url, request: &RQ) -> ClientResult<RS>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send,
    {
        self.client
            .http_api
            .connector
            .http_post_uri(full_uri, self.token, request)
            .await
    }
}

lazy_static! {
    pub static ref FT_HTTP_EMPTY_GET_PARAMS: Vec<(&'static str, Option<&'static String>)> = vec![];
}

impl FtClientHttpApiUri {
    pub const FT_API_URI_STR: &'static str = "https://api.intra.42.fr/v2";

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
