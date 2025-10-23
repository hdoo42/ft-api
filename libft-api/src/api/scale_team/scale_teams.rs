use crate::prelude::*;
use crate::to_param;
use libft_api_derive::HasItems;
use rsb_derive::Builder;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiScaleTeamsRequest {
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<u16>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasItems)]
#[serde(transparent)]
pub struct FtApiScaleTeamsResponse {
    pub scale_teams: Vec<FtScaleTeam>,
}
#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiScaleTeamsMultipleCreateRequest {
    pub scale_teams: Vec<FtApiScaleTeamsMultipleCreateBody>,
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FtApiScaleTeamsMultipleCreateBody {
    pub begin_at: FtDateTimeUtc,
    pub user_id: FtUserId,
    pub team_id: FtTeamId,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasItems)]
#[serde(transparent)]
pub struct FtApiScaleTeamsMultipleCreateResponse {
    pub scale_teams: Vec<FtScaleTeam>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn scale_teams_multiple_create_post(
        &self,
        req: FtApiScaleTeamsMultipleCreateRequest,
    ) -> ClientResult<FtApiScaleTeamsMultipleCreateResponse> {
        let url = "scale_teams/multiple_create";

        self.http_session_api.http_post(url, &req).await
    }

    pub async fn scale_teams(
        &self,
        req: FtApiScaleTeamsRequest,
    ) -> ClientResult<FtApiScaleTeamsResponse> {
        let url = "scale_teams";

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

    use crate::info::ft_campus_id::GYEONGSAN;

    use super::*;

    #[tokio::test]
    async fn with_filter() {
        let token = FtApiToken::try_get(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(token);
        let _ = session
            .scale_teams(
                FtApiScaleTeamsRequest::new()
                    .with_per_page(1)
                    .with_filter(vec![
                        FtFilterOption::new(FtFilterField::CampusId, vec![GYEONGSAN.to_string()]),
                        FtFilterOption::new(
                            FtFilterField::CursusId,
                            vec![FT_PISCINE_CURSUS_ID.to_string()],
                        ),
                    ]),
            )
            .await
            .unwrap();
    }
}
