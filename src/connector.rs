use std::time::Duration;

use futures::FutureExt;
use reqwest::{
    header::{self, AUTHORIZATION},
    Client, Method, RequestBuilder, StatusCode,
};
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

impl FtClientReqwestConnector {
    pub fn new() -> Self {
        Self::with_connector(reqwest::Client::new())
    }

    pub fn with_connector(connector: Client) -> Self {
        Self {
            ft_api_url: FtClientHttpApiUri::FT_API_URI_STR.to_string(),
            reqwest_connector: connector,
        }
    }
    pub fn with_ft_api_url(self, ft_api_url: &str) -> Self {
        Self {
            ft_api_url: ft_api_url.to_string(),
            ..self
        }
    }

    async fn send_http_request<'a, RS>(&'a self, reqwest: RequestBuilder) -> ClientResult<RS>
    where
        RS: for<'de> serde::de::Deserialize<'de>,
    {
        let http_res = reqwest
            .send()
            .await
            .map_err(|error| FtReqwestError { error })?;
        let http_status = http_res.status();
        let http_headers = http_res.headers().clone();
        let http_content_type = http_headers.get(header::CONTENT_TYPE);
        let http_body_str = http_res
            .text()
            .await
            .map_err(|error| FtReqwestError { error })?;

        let http_content_is_json = matches!(
            http_content_type.map(|content_type| content_type.to_str()),
            Some(Ok("application/json; charset=utf-8"))
        );

        println!("{:?} {http_content_is_json}", http_content_type);

        match http_status {
            StatusCode::OK if http_content_is_json => {
                let decoded_body = serde_json::from_str(http_body_str.as_str())
                    .map_err(|err| map_serde_error(err, Some(http_body_str.as_str())))?;
                Ok(decoded_body)
            }
            StatusCode::OK | StatusCode::NO_CONTENT => {
                println!("no contents");
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
    ) -> futures::prelude::future::BoxFuture<'a, crate::ClientResult<RS>>
    where
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + 'a + Send,
    {
        async move {
            let request = self
                .reqwest_connector
                .request(Method::GET, full_uri)
                .header(AUTHORIZATION, token.get_token_value());

            self.send_http_request(request).await
        }
        .boxed()
    }

    fn http_post_uri<'a, RQ, RS>(
        &'a self,
        full_uri: url::Url,
        token: &'a FtApiToken,
        request_body: &'a RQ,
    ) -> futures::prelude::future::BoxFuture<'a, crate::ClientResult<RS>>
    where
        RQ: serde::ser::Serialize + Send + Sync,
        RS: for<'de> serde::de::Deserialize<'de> + Send + 'a + Send + 'a,
    {
        async move {
            let post_json =
                serde_json::to_string(&request_body).map_err(|err| map_serde_error(err, None))?;

            let request = self
                .reqwest_connector
                .post(full_uri)
                .header(AUTHORIZATION, token.get_token_value())
                .body(post_json);

            self.send_http_request(request).await
        }
        .boxed()
    }
}
