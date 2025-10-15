use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusIdLocationsRequest {
    pub user_id: Option<FtUserId>,
    pub campus_id: FtCampusId,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiCampusIdLocationsResponse {
    pub location: Vec<FtLocation>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Retrieves location information for a specific campus from the 42 Intra API.
    ///
    /// This method fetches location data for a specific campus, including information about
    /// where users are currently located on that campus.
    ///
    /// # Parameters
    /// - `req`: A `FtApiCampusIdLocationsRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `campus_id`: The ID of the campus to retrieve location information for (required)
    /// - `user_id`: Optional user ID to filter locations for a specific user
    /// - `sort`: Optional vector of sort options to order the results
    /// - `range`: Optional vector of range options to filter results by date ranges
    /// - `filter`: Optional vector of filter options to filter the results
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiCampusIdLocationsResponse>`: Contains a vector of `FtLocation` objects
    ///
    /// # Example
    /// ```rust
    /// use libft_api::prelude::*;
    ///
    /// async fn example() -> ClientResult<()> {
    ///     let token = FtApiToken::try_get(AuthInfo::build_from_env()?).await?;
    ///     let client = FtClient::new(FtClientReqwestConnector::new());
    ///     let session = client.open_session(token);
    ///
    ///     // Get all locations for a specific campus (e.g., GyeongSan campus with ID 69)
    ///     let locations_response = session
    ///         .campus_id_locations(
    ///             FtApiCampusIdLocationsRequest::new(FtCampusId::new(69))
    ///                 .with_per_page(50)
    ///         )
    ///         .await?;
    ///     println!("Found {} locations", locations_response.location.len());
    ///
    ///     // Get locations for a specific user in a specific campus
    ///     let user_locations = session
    ///         .campus_id_locations(
    ///             FtApiCampusIdLocationsRequest::new(FtCampusId::new(69))
    ///                 .with_user_id(FtUserId::new(12345))
    ///         )
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn campus_id_locations(
        &self,
        req: FtApiCampusIdLocationsRequest,
    ) -> ClientResult<FtApiCampusIdLocationsResponse> {
        let url = &format!("campus/{}/locations", req.campus_id);

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

        let params = vec![
            to_param!(req, page),
            to_param!(req, per_page),
            to_param!(req, user_id),
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
        debug!("{:#?}", params);

        self.http_session_api
            .http_get(url, &[filters, range, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::ft_campus_id::GYEONGSAN;

    #[tokio::test]
    async fn location_with_params() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let res = session
            .campus_id_locations(
                FtApiCampusIdLocationsRequest::new(FtCampusId::new(GYEONGSAN)).with_per_page(1),
            )
            .await;

        assert!(res.is_ok());
    }
}
