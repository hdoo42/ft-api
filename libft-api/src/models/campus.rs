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
    pub created_at: Option<FtDateTimeUtc>,
    pub facebook: Option<FtUrl>,
    pub vogsphere_id: Option<FtVogsphereId>,
    pub website: Option<FtUrl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtEndpoint {
    pub id: FtEndpointId,
    pub url: FtUrl,
    pub description: String,
    pub created_at: Option<FtDateTimeUtc>,
    pub updated_at: Option<FtDateTimeUtc>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtVogsphereId(i32);

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
pub struct FtCampusId(pub i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCampusName(pub String);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtCampusUserCount(i32);

#[derive(Debug, Eq, Hash, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct FtZip(pub String);

#[test]
fn deser() {
    let raw_string = r#"
    [  {
    "id": 75,
    "name": "Rabat",
    "time_zone": "Africa/Casablanca",
    "language": {
      "id": 1,
      "name": "Français",
      "identifier": "fr"
    },
    "users_count": 491,
    "vogsphere_id": 67,
    "country": "Morocco",
    "address": "MohammedVIPolytechnicUniversity,RocadeRabat-Salé,Rabat",
    "zip": "1103",
    "city": "Rabat",
    "website": "https://1337.ma",
    "facebook": "",
    "twitter": "",
    "active": true,
    "public": true,
    "email_extension": "1337.ma",
    "default_hidden_phone": false,
    "endpoint": {
      "id": 65,
      "url": "https://endpoint-rab.1337.ma",
      "description": "Endpoint1337Rabat",
      "created_at": "2024-08-13T12:24:04.471Z",
      "updated_at": "2024-08-22T08:34:41.091Z"
    }
  },
  {
    "id": 73,
    "name": "IskandarPuteri",
    "time_zone": "Asia/Kuala_Lumpur",
    "language": {
      "id": 2,
      "name": "English",
      "identifier": "en"
    },
    "users_count": 65,
    "vogsphere_id": 66,
    "country": "Malaysia",
    "address": "G-01\u0026G-02HabCitrine,SunwayCitrine,SunwayIskandar.",
    "zip": "79250",
    "city": "IskandarPuteri",
    "website": "https://www.42.fr/",
    "facebook": "",
    "twitter": "",
    "active": true,
    "public": true,
    "email_extension": "42iskandarputeri.edu.my",
    "default_hidden_phone": false,
    "endpoint": {
      "id": 64,
      "url": "https://endpoint.42iskandarputeri.edu.my:4000",
      "description": "Endpointfor42IskandarPuteri",
      "created_at": "2024-04-12T02:26:36.784Z",
      "updated_at": "2024-04-12T08:16:18.665Z"
    }
  }]
    "#;

    serde_json::from_str::<Vec<FtCampus>>(raw_string).unwrap();
}
