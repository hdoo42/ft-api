use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    ClientResult, FtClientHttpConnector, FtClientHttpSessionApi, FtClientSession, FtLocations,
    GsInfo,
};

#[derive(Debug, Serialize, Deserialize, Builder)]
pub struct FtApiCampusLocationsResponse {
    pub usergroups: Vec<FtLocations>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn campus_gs_locations(&self) -> ClientResult<FtApiCampusLocationsResponse> {
        self.http_session_api
            .http_get(
                &format!("campus/{}/locations", GsInfo::CAMPUS_ID),
                &crate::common::FT_HTTP_EMPTY_GET_PARAMS.clone(),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gs_locations() {}
}
