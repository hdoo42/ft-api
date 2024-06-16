use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    ClientResult, FtCampusId, FtClientHttpConnector, FtClientSession, FtFilterField,
    FtFilterOption, FtLocation, FtLocationSortOption, FtUserId,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusLocationsRequest {
    pub user_id: Option<FtUserId>,
    pub campus_id: FtCampusId,
    pub sort: Option<Vec<FtLocationSortOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiCampusLocationsResponse {
    pub location: Vec<FtLocation>,
}

// Function to convert Vec<FtLocationFilterOption> to Vec<(&str, Option<String>)>
fn convert_to_tuples(filter_options: Vec<FtFilterOption>) -> Vec<(&'static str, Option<String>)> {
    filter_options
        .into_iter()
        .map(|option| {
            let field = match option.field {
                FtFilterField::Id => "filter[id]",
                FtFilterField::UserId => "filter[user_id]",
                FtFilterField::BeginAt => "filter[begin_at]",
                FtFilterField::EndAt => "filter[end_at]",
                FtFilterField::Primary => "filter[primary]",
                FtFilterField::Host => "filter[host]",
                FtFilterField::CampusId => "filter[campus_id]",
                FtFilterField::Active => "filter[active]",
                FtFilterField::Inactive => "filter[inactive]",
                FtFilterField::Future => "filter[future]",
                FtFilterField::End => "filter[end]",
            };
            let values = if option.value.is_empty() {
                None
            } else {
                Some(option.value.join(","))
            };
            (field, values)
        })
        .collect()
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn campus_id_locations(
        &self,
        req: FtApiCampusLocationsRequest,
    ) -> ClientResult<FtApiCampusLocationsResponse> {
        let url = &format!("campus/{}/locations", req.campus_id);

        let filters = convert_to_tuples(req.filter.unwrap_or_default());

        let params = vec![
            ("page", req.page.as_ref().map(|v| v.to_string())),
            ("per_page", req.per_page.as_ref().map(|v| v.to_string())),
            (
                "sort",
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
            ("user_id", req.user_id.as_ref().map(|v| v.to_string())),
        ];

        self.http_session_api
            .http_get(url, &[filters, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        locations::FtApiCampusLocationsRequest, AuthInfo, FtApiToken, FtCampusId, FtClient,
        FtClientReqwestConnector, FtFilterField, FtFilterOption, GS_CAMPUS_ID,
    };

    #[tokio::test]
    async fn location_with_params() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session
            .campus_id_locations(
                FtApiCampusLocationsRequest::new(FtCampusId::new(GS_CAMPUS_ID)).with_filter(vec![
                    FtFilterOption::new(FtFilterField::Active, vec!["true".to_string()]),
                ]),
            )
            .await;

        assert!(res.is_ok(), "{:?}", res);
    }
}
