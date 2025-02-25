use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, to_param, ClientResult,
    FtClientHttpConnector, FtClientSession, FtExam, FtExamId, FtExamUser, FtFilterOption,
    FtRangeOption, FtSortOption, FtUserId,
};

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
    pub exam_id: FtExamId,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiExamsResponse {
    pub exams: Vec<FtExam>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiExamsUsersPostResponse {
    pub exam: FtExamUser,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// ```
    /// #[tokio::test]
    /// async fn post_exams() {
    ///     let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
    ///         .await
    ///         .unwrap();
    ///
    ///     let client = FtClient::new(FtClientReqwestConnector::with_connector(
    ///         reqwest::Client::new(),
    ///     ));
    ///
    ///     let session = client.open_session(&token);
    ///
    ///     let res = session
    ///         .exams_users_post(
    ///             FtApiExamsUsersPostRequest::new(FtApiExamsUsersPostBody {
    ///                 user_id: FtUserId::new(212_750),
    ///             }),
    ///             FtExamId::new(22085),
    ///         )
    ///         .await
    ///         .unwrap();
    ///
    ///     assert_eq!(res.group.id, FtGroupId::new(FT_GROUP_ID_TEST_ACCOUNT));
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

    pub async fn exams_users_post(
        &self,
        req: FtApiExamsUsersPostRequest,
    ) -> ClientResult<FtApiExamsUsersPostResponse> {
        let url = &format!("exams/{}/exams_users", req.exam_id);

        self.http_session_api.http_post(url, &req).await
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use super::*;

    #[tokio::test]
    async fn get_exams() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);

        session.exams(FtApiExamsRequest::new()).await.unwrap();
    }
}
