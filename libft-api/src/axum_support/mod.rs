use std::sync::Arc;

use crate::{FtClient, FtClientHttpConnector};

pub struct FtEventsAxumListener<SCHC>
where
    SCHC: FtClientHttpConnector + Send + Sync,
{
    pub client: Arc<FtClient<SCHC>>,
}

impl<SCHC> FtEventsAxumListener<SCHC>
where
    SCHC: FtClientHttpConnector + Send + Sync,
{
    pub fn new(client: Arc<FtClient<SCHC>>) -> Self {
        Self { client }
    }
}
