use serde_json::Error as SerdeError;
use std::{
    fs::File,
    io::{self, BufReader, Write},
};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub struct AuthInfo {
    uid: String,
    secret: String,
}

impl AuthInfo {
    pub fn from_env(uid: String, secret: String) -> AuthInfo {
        AuthInfo { uid, secret }
    }

    pub fn get_params(&self) -> [(&str, &str); 3] {
        [
            ("grant_type", "client_credentials"),
            ("client_id", &self.uid),
            ("client_secret", &self.secret),
        ]
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    access_token: String,
    token_type: AccessTokenType,
    expires_in: i64,
    scope: AccessTokenScope,
    created_at: i64,
    secret_valid_until: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum AccessTokenScope {
    Public,
    Projects,
    Profile,
    Elearning,
    Tig,
    Forum,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AccessTokenType {
    #[serde(rename = "bearer")]
    Bearer,
}

pub enum LoadError {
    IOError(io::Error),
    SerdeError(SerdeError),
    TokenExpired,
    TokenLiftTimeParsingFailed,
    TempTokenNotFound,
    BuildError(String),
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        LoadError::IOError(err)
    }
}

impl From<SerdeError> for LoadError {
    fn from(err: SerdeError) -> Self {
        LoadError::SerdeError(err)
    }
}

impl AccessToken {
    pub async fn try_get(info: AuthInfo) -> Result<AccessToken, LoadError> {
        let tmpdir = std::env::temp_dir().join(".ft_api_auth_token");
        let temp_token = if tmpdir.is_file() {
            let file = File::open(tmpdir)?;
            let reader = BufReader::new(file);
            let token: AccessToken = serde_json::from_reader(reader)?;

            let expire_date: DateTime<Utc> = Utc
                .timestamp_opt(token.created_at + token.expires_in, 0)
                .single()
                .ok_or(LoadError::TokenLiftTimeParsingFailed)?;

            if Utc::now() < expire_date {
                Ok(token)
            } else {
                Err(LoadError::TokenExpired)
            }
        } else {
            Err(LoadError::TempTokenNotFound)
        };

        if temp_token.is_ok() {
            temp_token
        } else {
            let token = AccessToken::build(info)
                .await
                .map_err(LoadError::BuildError)?;
            token.save()?;
            Ok(token)
        }
    }

    pub fn save(&self) -> Result<(), LoadError> {
        let tmpdir = std::env::temp_dir().join(".ft_api_auth_token");
        let mut token = File::create_new(tmpdir).unwrap();
        token.write_all(serde_json::to_string(self).unwrap().as_bytes())?;
        Ok(())
    }

    pub async fn build(info: AuthInfo) -> Result<AccessToken, String> {
        let params = info.get_params();

        let client = reqwest::Client::new();
        let res = client
            .post("https://api.intra.42.fr/oauth/token")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Error: {e}"))
            .expect("Access token request itself failed");

        match res.status() {
            reqwest::StatusCode::OK => res
                .json::<AccessToken>()
                .await
                .map_err(|e| format!("Error in parsing json: {e}")),
            reqwest::StatusCode::UNAUTHORIZED => Err(format!(
                "Error: got UNAUTHORIZED code: {:?}",
                res.error_for_status()
            )),
            reqwest::StatusCode::NOT_FOUND => Err(format!(
                "Error: 404 not found: {:?}",
                res.error_for_status()
            )),
            other => panic!("Uh oh! Something unexpected happened: {:?}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use util::config_env_var;

    use super::*;

    #[tokio::test]
    async fn auth_success() {
        let info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").unwrap(),
            config_env_var("FT_API_CLIENT_SECRET").unwrap(),
        );
        let res = AccessToken::build(info).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn try_to_get_token() {
        let info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").unwrap(),
            config_env_var("FT_API_CLIENT_SECRET").unwrap(),
        );
        let res = AccessToken::try_get(info).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn auth_fail() {
        let info = AuthInfo::from_env(String::from("test for fail"), String::from("test for fail"));
        let res = AccessToken::build(info).await;

        assert!(res.is_err());
    }
}
