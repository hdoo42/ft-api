use std::time::Duration;

use futures::FutureExt;
use reqwest::{
    header::{self, AUTHORIZATION},
    Client, RequestBuilder, StatusCode,
};
use tracing::{debug, info, Span};
use url::Url;

use crate::*;

pub struct FtClientReqwestConnector {
    reqwest_connector: Client,
    ft_api_url: String,
}

impl Default for FtClientReqwestConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct FtClientApiCallContext<'a> {
    pub tracing_span: &'a Span,
    pub current_page: Option<usize>,
}

impl FtClientReqwestConnector {
    #[must_use]
    pub fn new() -> Self {
        Self::with_connector(reqwest::Client::new())
    }

    #[must_use]
    pub fn with_connector(connector: Client) -> Self {
        Self {
            ft_api_url: FtClientHttpApiUri::FT_API_URI_STR.to_string(),
            reqwest_connector: connector,
        }
    }

    #[must_use]
    pub fn with_ft_api_url(self, ft_api_url: &str) -> Self {
        Self {
            ft_api_url: ft_api_url.to_string(),
            ..self
        }
    }

    // TODO: chagne to hyper, remove url
    async fn send_http_request<'a, RS>(
        &'a self,
        reqwest: RequestBuilder,
        url: Url,
    ) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let url_str = url.to_string();
        info!(ft_url = url_str, "Sending HTTP request to");
        let http_res = reqwest
            .send()
            .await
            .map_err(|error| FtReqwestError { error })?;
        let http_status = http_res.status();
        let http_headers = http_res.headers().clone();
        debug!("headers: {:#?}", http_headers);
        let http_content_type = http_headers.get(header::CONTENT_TYPE);
        let http_body_str = http_res
            .text()
            .await
            .map_err(|error| FtReqwestError { error })?;

        info!(ft_url = url_str, "Received HTTP response {}", http_status);
        let http_content_is_json = matches!(
            http_content_type.map(|content_type| content_type.to_str()),
            Some(Ok("application/json; charset=utf-8"))
        );

        match http_status {
            StatusCode::OK if http_content_is_json => {
                let decoded_body = serde_json::from_str(http_body_str.as_str())
                    .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;
                Ok(decoded_body)
            }
            StatusCode::CREATED if http_content_is_json => {
                let decoded_body = serde_json::from_str(http_body_str.as_str())
                    .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;
                Ok(decoded_body)
            }
            StatusCode::OK | StatusCode::NO_CONTENT => {
                serde_json::from_str("{}").map_err(|err| map_serde_error(err, Some("{}")))
            }
            StatusCode::TOO_MANY_REQUESTS if http_content_is_json => {
                let ft_message: FtEnvelopeMessage = serde_json::from_str(http_body_str.as_str())
                    .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;

                Err(FtClientError::RateLimitError(
                    FtRateLimitError::new()
                        .opt_retry_after(
                            http_headers
                                .get(header::RETRY_AFTER)
                                .and_then(|ra| ra.to_str().ok().and_then(|s| s.parse().ok()))
                                .map(Duration::from_secs),
                        )
                        .opt_code(ft_message.error)
                        .opt_warnings(ft_message.warnings)
                        .with_http_response_body(http_body_str),
                ))
            }
            StatusCode::TOO_MANY_REQUESTS => Err(FtClientError::RateLimitError(
                FtRateLimitError::new()
                    .opt_retry_after(
                        http_headers
                            .get(header::RETRY_AFTER)
                            .and_then(|ra| ra.to_str().ok().and_then(|s| s.parse().ok()))
                            .map(Duration::from_secs),
                    )
                    .with_http_response_body(http_body_str),
            )),
            _ => Err(FtClientError::HttpError(
                FtHttpError::new(http_status).with_http_response_body(http_body_str),
            )),
        }
    }
}

impl FtClientHttpConnector for FtClientReqwestConnector {
    fn create_method_uri_path(&self, method_relative_uri: &str) -> ClientResult<Url> {
        Ok(format!("{}/{}", self.ft_api_url, method_relative_uri).parse()?)
    }

    fn http_get_uri<'a, RS>(
        &'a self,
        full_uri: url::Url,
        token: &'a FtApiToken,
    ) -> futures::prelude::future::BoxFuture<'a, ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
    {
        async move {
            let request = self
                .reqwest_connector
                //TODO: remove clone after migrate to hyper
                .get(full_uri.clone())
                .header(AUTHORIZATION, token.get_token_value());

            self.send_http_request(request, full_uri).await
        }
        .boxed()
    }

    fn http_post_uri<'a, RQ, RS>(
        &'a self,
        full_uri: url::Url,
        token: &'a FtApiToken,
        request_body: &'a RQ,
    ) -> futures::prelude::future::BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
    {
        async move {
            let request = self
                .reqwest_connector
                //TODO: remove clone after migrate to hyper
                .post(full_uri.clone())
                .header(AUTHORIZATION, token.get_token_value())
                .json(&request_body);

            self.send_http_request(request, full_uri).await
        }
        .boxed()
    }

    fn http_patch_uri<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        token: &'a FtApiToken,
        request_body: &'a RQ,
    ) -> futures::prelude::future::BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
    {
        async move {
            let request = self
                .reqwest_connector
                //TODO: remove clone after migrate to hyper
                .patch(full_uri.clone())
                .header(AUTHORIZATION, token.get_token_value())
                .json(&request_body);

            self.send_http_request(request, full_uri).await
        }
        .boxed()
    }

    fn http_delete_uri<'a, RQ, RS>(
        &'a self,
        full_uri: Url,
        token: &'a FtApiToken,
        request_body: &'a RQ,
    ) -> futures::future::BoxFuture<'a, ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a,
    {
        async move {
            let request = self
                .reqwest_connector
                //TODO: remove clone after migrate to hyper
                .delete(full_uri.clone())
                .header(AUTHORIZATION, token.get_token_value())
                .json(&request_body);

            self.send_http_request(request, full_uri).await
        }
        .boxed()
    }
}
