use rsb_derive::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    convert_filter_option_to_tuple, convert_range_option_to_tuple, ClientResult,
    FtClientHttpConnector, FtClientSession, FtCursusId, FtFilterOption, FtProject, FtProjectId,
    FtRangeOption, FtSortOption,
};

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

#[derive(Debug, Serialize, Deserialize, Builder)]
#[serde(transparent)]
pub struct FtApiProjectResponse {
    pub value: Vec<FtProject>,
}

impl<'a, FCHC> FtClientSession<'a, FCHC>
where
    FCHC: FtClientHttpConnector + Send + Sync,
{
    pub async fn projects(&self, req: FtApiProjectRequest) -> ClientResult<FtApiProjectResponse> {
        let url = "projects";

        let filters = convert_filter_option_to_tuple(req.filter.unwrap_or_default());
        let range = convert_range_option_to_tuple(req.range.unwrap_or_default());

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
        ];

        self.http_session_api
            .http_get(url, &[filters, range, params].concat())
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write, path::PathBuf};

    use crate::*;

    use super::*;

    #[tokio::test]
    async fn projects() {
        let token = FtApiToken::build(AuthInfo::build_from_env().unwrap())
            .await
            .unwrap();

        let client = FtClient::new(FtClientReqwestConnector::with_connector(
            reqwest::Client::new(),
        ));

        let session = client.open_session(&token);
        let res = session.projects(FtApiProjectRequest::new()).await;

        assert!(res.is_ok(), "{:?}", res);

        // Get the system's temp directory
        let mut temp_dir = PathBuf::new();

        // Create a temporary file path
        temp_dir.push("temp_project.txt");

        // Create the file
        let mut temp_file = File::create(&temp_dir).unwrap();

        // Write the data to the file
        let _ = temp_file.write_all(format!("{:?}", res.unwrap()).as_bytes());
    }
}
