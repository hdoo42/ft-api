use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiProjectRequest {
    pub cursus_id: Option<FtCursusId>,
    pub project_id: Option<FtProjectId>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiProjectResponse {
    pub projects: Vec<FtProject>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Retrieves a list of projects from the 42 Intra API.
    ///
    /// # Parameters
    /// - `req`: A `FtApiProjectRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `cursus_id`: Optional cursus ID to filter projects by cursus
    /// - `project_id`: Optional project ID to filter results
    /// - `sort`: Optional vector of sort options
    /// - `range`: Optional vector of range options
    /// - `filter`: Optional vector of filter options
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiProjectResponse>`: Contains a vector of `FtProject` objects
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
    ///     // Get all projects with pagination
    ///     let projects = session
    ///         .projects(
    ///             FtApiProjectRequest::new()
    ///                 .with_per_page(50)
    ///         )
    ///         .await?;
    ///     println!("Found {} projects", projects.projects.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn projects(&self, req: FtApiProjectRequest) -> ClientResult<FtApiProjectResponse> {
        let url = "projects";

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
    async fn projects() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let res = session
            .projects(FtApiProjectRequest::new().with_per_page(1))
            .await;

        assert!(res.is_ok());
    }
}
