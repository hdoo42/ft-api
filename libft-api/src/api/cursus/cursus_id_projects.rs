use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCursusIdProjectsRequest {
    pub cursus_id: FtCursusId,
    pub project_id: Option<FtProjectId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiCursusIdProjectsResponse {
    pub projects: Vec<FtProject>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Retrieves projects associated with a specific cursus from the 42 Intra API.
    ///
    /// # Parameters
    /// - `req`: A `FtApiCursusIdProjectsRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `cursus_id`: The ID of the cursus to retrieve projects for (required)
    /// - `project_id`: Optional project ID to filter results
    /// - `sort`: Optional vector of sort options
    /// - `range`: Optional vector of range options
    /// - `filter`: Optional vector of filter options
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiCursusIdProjectsResponse>`: Contains a vector of `FtProject` objects
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
    ///     // Get projects for the common core cursus
    ///     let projects = session
    ///         .cursus_id_projects(
    ///             FtApiCursusIdProjectsRequest::new(FtCursusId::new(FT_CURSUS_ID))
    ///         )
    ///         .await?;
    ///     println!("Found {} projects", projects.projects.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn cursus_id_projects(
        &self,
        req: FtApiCursusIdProjectsRequest,
    ) -> ClientResult<FtApiCursusIdProjectsResponse> {
        let url = &format!("cursus/{}/projects", req.cursus_id);

        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();
        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();

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
    async fn basic() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let res = session
            .cursus_id_projects(
                FtApiCursusIdProjectsRequest::new(FtCursusId::new(FT_CURSUS_ID)).with_per_page(1),
            )
            .await;

        assert!(res.is_ok());
    }
}
