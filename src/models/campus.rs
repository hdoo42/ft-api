use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};

use crate::{FtDateTimeUtc, FtLanguage, FtUrl};

#[derive(Debug, Serialize, Deserialize)]
pub struct FtCampus {
    pub active: bool,
    pub address: FtAddress,
    pub city: String,
    pub country: String,
    pub default_hidden_phone: bool,
    pub email_extension: Option<String>,
    pub endpoint: Option<FtEndpoint>,
    pub facebook: Option<FtUrl>,
    pub id: FtCampusId,
    pub language: FtLanguage,
    pub name: FtCampusName,
    pub public: bool,
    pub time_zone: String,
    pub twitter: Option<FtUrl>,
    pub users_count: FtCampusUserCount,
    pub vogsphere_id: Option<i32>,
    pub website: Option<FtUrl>,
    pub zip: FtZip,
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
