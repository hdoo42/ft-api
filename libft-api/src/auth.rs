use serde_json::Error as SerdeError;
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufReader, Write},
    path::PathBuf,
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

    #[inline]
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
    TokenLifeTimeParsingFailed,
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
    fn __get_tmp_path() -> PathBuf {
        std::env::temp_dir().join(".ft_api_auth_token")
    }

    fn __try_get() -> Result<FtApiToken, TokenError> {
        let tmpdir = Self::__get_tmp_path();

        if !tmpdir.is_file() {
            return Err(TokenError::TempTokenNotFound);
        }

        let file = File::open(tmpdir)?;
        let reader = BufReader::new(file);
        let token: FtApiToken = serde_json::from_reader(reader)?;

        let expire_date: DateTime<Utc> = Utc
            .timestamp_opt(token.created_at + token.expires_in, 0)
            .single()
            .ok_or(TokenError::TokenLifeTimeParsingFailed)?;

        match Utc::now() >= expire_date {
            true => Err(TokenError::TokenExpired),
            false => Ok(token),
        }
    }

    pub async fn try_get(info: AuthInfo) -> Result<FtApiToken, TokenError> {
        if let Ok(token) = Self::__try_get() {
            return Ok(token);
        }

        let _ = std::fs::remove_file(Self::__get_tmp_path());

        let token = FtApiToken::build(info)
            .await
            .map_err(TokenError::BuildError)?;

        let _ = token.save();

        Ok(token)
    }

    pub fn save(&self) -> Result<(), TokenError> {
        let tmpdir = std::env::temp_dir().join(".ft_api_auth_token");
        let mut token = File::create_new(tmpdir)?;
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
            .map_err(|e| format!("Error: {e}"))?;

        match res.status() {
            reqwest::StatusCode::OK => res
                .json::<FtApiToken>()
                .await
                .map_err(|e| format!("Error in parsing json: {e}")),
            reqwest::StatusCode::UNAUTHORIZED => {
                Err(format!("UNAUTHORIZED: {:?}", res.error_for_status()))
            }
            reqwest::StatusCode::NOT_FOUND => Err(format!("Error: {:?}", res.error_for_status())),
            other => Err(other.to_string()),
        }
    }
}

/// Get the value of an environment variable.
///
/// # Errors
///
/// This function will return an error if the environment variable is not set.
pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{name}: {e}"))
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
        let info = AuthInfo::build_from_env().unwrap();
        let res = FtApiToken::try_get(info).await;

        assert!(res.is_ok());
    }
}
