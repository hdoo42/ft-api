use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::prelude::*;
use crate::to_param;

use libft_api_derive::HasVector;

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusIdJournalsRequest {
    pub user_id: Option<FtUserId>,
    pub campus_id: FtCampusId,
    pub begin_at: String,
    pub end_at: String,
    pub sort: Option<Vec<FtSortOption>>,
    pub range: Option<Vec<FtRangeOption>>,
    pub filter: Option<Vec<FtFilterOption>>,
    pub page: Option<usize>,
    pub per_page: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Builder, HasVector)]
#[serde(transparent)]
pub struct FtApiCampusIdJournalsResponse {
    pub journals: Vec<FtJournal>,
}

impl<FCHC> FtClientSession<'_, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    /// Get journals for a specific campus.
    ///
    /// This action requires the 'Advanced staff' role.
    /// This resource is paginated, with a default of 30 items per page.
    /// You have to provide parameters with FtApiCampusIdJournalsRequest structure
    ///
    /// # Parameters
    ///
    /// *   `begin_at`: **Required** (`String`). Must be before or equal to `end_at`. The date range must be 124 days maximum.
    /// *   `end_at`: **Required** (`String`). Must be after or equal to `begin_at`. The date range must be 124 days maximum.
    /// *   `campus_id`: **Required** (`String`). The campus ID or slug.
    /// *   `sort`: Optional. The sort field. Sorted by `id` desc by default.
    ///     Must be one of: `id`, `user_id`, `item_type`, `item_id`, `cursus_id`, `campus_id`, `reason`, `created_at`, `updated_at`, `event_at`, `alumni`, `closed`.
    /// *   `filter`: Optional. Filtering on one or more fields.
    ///     Must be one of: `id`, `user_id`, `item_type`, `item_id`, `cursus_id`, `campus_id`, `reason`, `created_at`, `updated_at`, `event_at`, `alumni`, `closed`, `event`.
    /// *   `page[size]`: Optional (`Integer`). The number of items per page. Defaults to 30, maximum 100.
    /// *   `page[number]`: Optional (`Integer`). The current page number.
    ///
    /// # Errors
    ///
    /// * This function will return an error if the authenticated user does not have the [`Advanced staff`] role.
    pub async fn campus_id_journals(
        &self,
        req: FtApiCampusIdJournalsRequest,
    ) -> ClientResult<FtApiCampusIdJournalsResponse> {
        let url = &format!("campus/{}/journals", req.campus_id);

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default()).unwrap();
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default()).unwrap();

        let params = vec![
            to_param!(req, page),
            to_param!(req, per_page),
            to_param!(req, user_id),
            ("begin_at".to_string(), Some(req.begin_at)),
            ("end_at".to_string(), Some(req.end_at)),
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

    use crate::info::ft_campus_id::GYEONGSAN;

    use super::*;

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
            .campus_id_journals(
                FtApiCampusIdJournalsRequest::new(
                    FtCampusId::new(GYEONGSAN),
                    "2025-1-1".to_string(),
                    "2025-1-2".to_string(),
                )
                .with_per_page(1),
            )
            .await;

        assert!(res.is_ok());
    }
}
