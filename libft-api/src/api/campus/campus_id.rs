use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusIdRequest {
    pub campus_id: Option<FtCampusId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiCampusIdResponse {
    pub campus: Vec<FtCampus>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Retrieves information about campuses from the 42 Intra API.
    ///
    /// # Parameters
    /// - `req`: A `FtApiCampusIdRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `campus_id`: Optional campus ID to retrieve information about a specific campus
    /// - `sort`: Optional vector of sort options to order the results
    /// - `range`: Optional vector of range options to filter results by date ranges
    /// - `filter`: Optional vector of filter options to filter the results
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiCampusIdResponse>`: Contains a vector of `FtCampus` objects
    ///
    /// # Example
    ///
    /// See Test code
    pub async fn campus_id(
        &self,
        req: FtApiCampusIdRequest,
    ) -> ClientResult<FtApiCampusIdResponse> {
        let url = match req.campus_id {
            Some(campus_id) => &format!("campus/{campus_id}"),
            None => "campus",
        };

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

        let params = vec![
            to_param!(req, page),
            to_param!(req, per_page),
            (
                "sort".to_string(),
                req.sort.as_ref().map(|v| {
                    v.iter()
                        .map(|v| {
                            format!(
                                "{}{}",
                                if v.descending { "-" } else { "" },
                                serde_plain::to_string(&v.field).unwrap()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                }),
            ),
        ];

        self.http_session_api
            .http_get(url, &[filters, range, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn basic() -> ClientResult<()> {
        let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));
        let session = client.open_session(token);

        let _ = session
            .campus_id(FtApiCampusIdRequest::new().with_per_page(1))
            .await?;

        Ok(())
    }
}
