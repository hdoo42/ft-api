use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{ClientResult, FtClientHttpConnector, FtClientSession, FtLocation, GsInfo};

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiCampusLocationsResponse {
    pub location: FtLocation,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn campus_gs_locations(&self) -> ClientResult<Vec<FtApiCampusLocationsResponse>> {
        println!("campus_gs_locations");
        let url = &format!("campus/{}/locations", GsInfo::CAMPUS_ID);
        println!("url: {}", url);

        self.http_session_api
            .http_get(url, &crate::common::FT_HTTP_EMPTY_GET_PARAMS.clone())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gs_locations() {}
}
