use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{FtDateTimeUtc, FtLanguage, FtUrl};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtCampus {
    pub id: FtCampusId,
    pub active: Option<bool>,
    pub address: Option<FtAddress>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub default_hidden_phone: Option<bool>,
    pub language: Option<FtLanguage>,
    pub name: Option<FtCampusName>,
    pub public: Option<bool>,
    pub time_zone: Option<String>,
    pub zip: Option<FtZip>,
    pub users_count: Option<FtCampusUserCount>,
    pub twitter: Option<FtUrl>,
    pub email_extension: Option<String>,
    pub endpoint: Option<FtEndpoint>,
    pub facebook: Option<FtUrl>,
    pub vogsphere_id: Option<i32>,
    pub website: Option<FtUrl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtEndpoint {
    pub id: FtEndpointId,
    pub url: FtUrl,
    pub description: String,
    pub created_at: FtDateTimeUtc,
    pub updated_at: FtDateTimeUtc,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtEndpointId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtAddress(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCity(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCountry(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtEmailExtension(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCampusId(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCampusName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCampusUserCount(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtZip(pub String);
