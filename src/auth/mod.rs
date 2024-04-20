use serde_json::Error as SerdeError;
use std::{
    fmt::Display,
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

    pub fn build_from_env() -> Result<AuthInfo, String> {
        let uid = config_env_var("FT_API_CLIENT_UID")?;
        let secret = config_env_var("FT_API_CLIENT_SECRET")?;

        Ok(AuthInfo { uid, secret })
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
pub struct FtApiToken {
    access_token: String,
    token_type: AccessTokenType,
    expires_in: i64,
    scope: AccessTokenScope,
    created_at: i64,
    secret_valid_until: i64,
}

impl FtApiToken {
    pub fn get_token_value(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
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

impl Display for AccessTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bearer")
    }
}

#[derive(Debug)]
pub enum TokenError {
    IOError(io::Error),
    SerdeError(SerdeError),
    TokenExpired,
    TokenLiftTimeParsingFailed,
    TempTokenNotFound,
    BuildError(String),
}
impl From<io::Error> for TokenError {
    fn from(err: io::Error) -> Self {
        TokenError::IOError(err)
    }
}

impl From<SerdeError> for TokenError {
    fn from(err: SerdeError) -> Self {
        TokenError::SerdeError(err)
    }
}

impl FtApiToken {
    pub async fn try_get(info: AuthInfo) -> Result<FtApiToken, TokenError> {
        let tmpdir = std::env::temp_dir().join(".ft_api_auth_token");
        let temp_token = if tmpdir.is_file() {
            let file = File::open(tmpdir)?;
            let reader = BufReader::new(file);
            let token: FtApiToken = serde_json::from_reader(reader)?;

            let expire_date: DateTime<Utc> = Utc
                .timestamp_opt(token.created_at + token.expires_in, 0)
                .single()
                .ok_or(TokenError::TokenLiftTimeParsingFailed)?;

            if Utc::now() < expire_date {
                Ok(token)
            } else {
                Err(TokenError::TokenExpired)
            }
        } else {
            Err(TokenError::TempTokenNotFound)
        };

        if temp_token.is_ok() {
            temp_token
        } else {
            let token = FtApiToken::build(info)
                .await
                .map_err(TokenError::BuildError)?;

            match token.save() {
                Ok(_) => Ok(token),
                Err(e) => {
                    println!("WARNING: Failed to save token: {:?}", e);
                    Ok(token)
                }
            }
        }
    }

    pub fn save(&self) -> Result<(), TokenError> {
        let tmpdir = std::env::temp_dir().join(".ft_api_auth_token");
        let mut token = File::create_new(tmpdir).unwrap();
        token.write_all(serde_json::to_string(self).unwrap().as_bytes())?;
        Ok(())
    }

    pub async fn build(info: AuthInfo) -> Result<FtApiToken, String> {
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
                .json::<FtApiToken>()
                .await
                .map_err(|e| format!("Error in parsing json: {e}")),
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(format!("Error: {:?}", res.error_for_status()))
            }
            reqwest::StatusCode::NOT_FOUND => Err(format!("Error: {:?}", res.error_for_status())),
            other => panic!("Uh oh! Something unexpected happened: {:?}", other),
        }
    }
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn auth_fail() {
        let info = AuthInfo::from_env(String::from("test for fail"), String::from("test for fail"));
        let res = FtApiToken::build(info).await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn auth_success() {
        let info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").unwrap(),
            config_env_var("FT_API_CLIENT_SECRET").unwrap(),
        );
        let res = FtApiToken::build(info).await;

        assert!(res.is_ok(), "{:?}", res);
    }

    #[tokio::test]
    async fn try_to_get_token() {
        let info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").unwrap(),
            config_env_var("FT_API_CLIENT_SECRET").unwrap(),
        );
        let res = FtApiToken::try_get(info).await;

        assert!(res.is_ok());
    }
}
