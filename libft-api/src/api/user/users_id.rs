use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{prelude::*, to_param};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtUserExt {
    pub id: Option<FtUserId>,
    pub email: Option<FtEmail>,
    pub login: Option<FtLoginId>,
    pub first_name: Option<FtFirstName>,
    pub last_name: Option<FtLastName>,
    pub url: Option<FtUrl>,
    pub phone: Option<FtPhone>,
    pub displayname: Option<FtDisplayName>,
    pub kind: Option<FtKind>,
    pub active: Option<bool>,
    pub alumni: Option<bool>,
    pub alumnized_at: Option<FtDateTimeFixedOffset>,
    pub anonymize_date: Option<FtDateTimeFixedOffset>,
    pub correction_point: Option<FtCorrectionPoint>,
    pub created_at: Option<FtDateTimeUtc>,
    pub data_erasure_date: Option<FtDateTimeUtc>,
    pub image: Option<FtImage>,
    pub location: Option<FtHost>,
    pub pool_month: Option<FtPoolMonth>,
    pub pool_year: Option<FtPoolYear>,
    pub staff: Option<bool>,
    pub updated_at: Option<FtDateTimeUtc>,
    pub usual_first_name: Option<FtUsualFirstName>,
    pub usual_full_name: Option<FtUsualFullName>,
    pub wallet: Option<FtWallet>,
    pub cursus_users: Option<Vec<FtCursusUser>>,
    pub projects_users: Option<Vec<FtProjectsUser>>,
    pub campus: Option<Vec<FtCampus>>,
    pub campus_users: Option<Vec<FtCampusUser>>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiUsersIdRequest {
    pub id: FtUserIdentifier,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiUsersIdResponse {
    pub user: FtUserExt,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn users_id(&self, req: FtApiUsersIdRequest) -> ClientResult<FtApiUsersIdResponse> {
        let url = &format!(
            "users/{}",
            match req.id {
                FtUserIdentifier::Login(ft_login_id) => ft_login_id.to_string(),
                FtUserIdentifier::UserId(ft_user_id) => ft_user_id.to_string(),
            }
        );
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
    use crate::*;

    #[tokio::test]
    async fn basic() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .users_id(FtApiUsersIdRequest::new(FtUserIdentifier::Login(
                FtLoginId::new("taejikim".to_owned()),
            )))
            .await
            .unwrap();

        // assert!(res.is_ok());
    }
}
