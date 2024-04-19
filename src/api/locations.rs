use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{ClientResult, FtApiClientHttpSessionApi, FtClientHttpConnector, FtLocations};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusLocationsResponse {
    pub usergroups: Vec<FtLocations>,
}

impl<FCHC> FtApiClientHttpSessionApi<FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn campus_gs_locations(&self) -> ClientResult<FtApiCampusLocationsResponse> {
        self.client
            .http_api
            .connector
            .http_get(
                "campus/69/locations",
                &crate::FT_HTTP_EMPTY_GET_PARAMS.clone(),
            )
            .await
    }
}
