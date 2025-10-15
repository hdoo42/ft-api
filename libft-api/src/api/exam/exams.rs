use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiExamsRequest {
    pub page: Option<u16>,
    pub per_page: Option<u8>,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiExamsUsersPostRequest {
    pub exams_user: FtApiExamsUsersPostBody,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtApiExamsUsersPostBody {
    pub user_id: FtUserId,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiExamsResponse {
    pub exams: Vec<FtExam>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiExamsUsersPostResponse {
    pub exam: FtExamUser,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Retrieves a list of exams from the 42 Intra API.
    ///
    /// This method fetches information about exams with various filtering and pagination options.
    ///
    /// # Parameters
    /// - `req`: A `FtApiExamsRequest` struct containing the query parameters.
    ///
    /// # Query Parameters
    /// - `sort`: Optional vector of sort options to order the results
    /// - `range`: Optional vector of range options to filter results by date ranges
    /// - `filter`: Optional vector of filter options to filter the results
    /// - `page`: Optional page number for pagination
    /// - `per_page`: Optional number of items per page for pagination
    ///
    /// # Returns
    /// - `ClientResult<FtApiExamsResponse>`: Contains a vector of `FtExam` objects
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
    ///     // Get all exams with pagination
    ///     let exams_response = session
    ///         .exams(
    ///             FtApiExamsRequest::new()
    ///                 .with_per_page(20)
    ///         )
    ///         .await?;
    ///     println!("Found {} exams", exams_response.exams.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn exams(&self, req: FtApiExamsRequest) -> ClientResult<FtApiExamsResponse> {
        let url = "exams";

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

    /// Creates an association between a user and an exam from the 42 Intra API.
    ///
    /// This method creates an exam-user association, typically used to register a user for an exam.
    ///
    /// # Parameters
    /// - `req`: A `FtApiExamsUsersPostRequest` struct containing the exam-user association data.
    /// - `exam_id`: The ID of the exam to create the association for (required)
    ///
    /// # Returns
    /// - `ClientResult<FtApiExamsUsersPostResponse>`: Contains the created `FtExamUser` object
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
    ///     // Create an exam-user association (requires appropriate permissions)
    ///     // let exam_user_request = FtApiExamsUsersPostRequest::new(
    ///     //     FtApiExamsUsersPostBody {
    ///     //         user_id: FtUserId::new(12345),
    ///     //     }
    ///     // );
    ///     // let exam_user_response = session
    ///     //     .exams_users_post(exam_user_request, FtExamId::new(12345))
    ///     //     .await?;
    ///     //
    ///     // println!("Created exam-user association with ID: {:?}", exam_user_response.exam.id);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn exams_users_post(
        &self,
        req: FtApiExamsUsersPostRequest,
        exam_id: FtExamId,
    ) -> ClientResult<FtApiExamsUsersPostResponse> {
        let url = &format!("exams/{exam_id}/exams_users");

        self.http_session_api.http_post(url, &req).await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn get_exams() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);

        session
            .exams(FtApiExamsRequest::new().with_per_page(1))
            .await
            .unwrap();
    }
}
