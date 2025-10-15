//! Authentication module for the 42 Intra API.
//!
//! This module provides functionality for:
//! * Managing OAuth2 client credentials
//! * Building API tokens from environment variables
//! * Caching tokens to disk
//! * Handling token expiration and renewal

use serde_json::Error as SerdeError;
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufReader, Write},
    path::PathBuf,
};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

//   TODO: add scope
/// Authentication information for the 42 API.
///
/// Contains the client credentials (UID and secret) required to obtain an API token.
///
/// # Example
///
/// ```rust
/// use libft_api::prelude::*;
///
/// // Create from environment variables
/// let auth_info = AuthInfo::build_from_env().unwrap();
///
/// // Or create directly with credentials
/// let auth_info = AuthInfo::from_env(
///     "your_client_id".to_string(),
///     "your_client_secret".to_string()
/// );
/// ```
pub struct AuthInfo {
    uid: String,
    secret: String,
}

impl AuthInfo {
    /// Create a new `AuthInfo` from the given UID and secret.
    ///
    /// # Arguments
    ///
    /// * `uid` - The client ID for the 42 API application
    /// * `secret` - The client secret for the 42 API application
    ///
    /// # Example
    ///
    /// ```rust
    /// use libft_api::auth::AuthInfo;
    ///
    /// let auth_info = AuthInfo::from_env(
    ///     "your_client_id".to_string(),
    ///     "your_client_secret".to_string()
    /// );
    /// ```
    pub fn from_env(uid: String, secret: String) -> AuthInfo {
        AuthInfo { uid, secret }
    }

    /// Build `AuthInfo` from environment variables.
    ///
    /// This function reads the `FT_API_CLIENT_UID` and `FT_API_CLIENT_SECRET` environment variables
    /// to create an `AuthInfo` instance.
    ///
    /// # Environment Variables
    ///
    /// * `FT_API_CLIENT_UID` - The client ID for the 42 API application
    /// * `FT_API_CLIENT_SECRET` - The client secret for the 42 API application
    ///
    /// # Errors
    ///
    /// This function will return an error if the `FT_API_CLIENT_UID` or `FT_API_CLIENT_SECRET` environment variables are not set.
    ///
    /// # Example
    ///
    /// ```rust
    /// use libft_api::auth::AuthInfo;
    ///
    /// // Requires FT_API_CLIENT_UID and FT_API_CLIENT_SECRET to be set in the environment
    /// let auth_info = AuthInfo::build_from_env().unwrap();
    /// ```
    pub fn build_from_env() -> Result<AuthInfo, String> {
        let uid = config_env_var("FT_API_CLIENT_UID")?;
        let secret = config_env_var("FT_API_CLIENT_SECRET")?;

        Ok(AuthInfo { uid, secret })
    }

    #[inline]
    // TODO: replace scope to field 'scope'
    /// Get the parameters for the API token request.
    ///
    /// Returns the form parameters required to request an OAuth2 token from the 42 API.
    ///
    /// # Returns
    ///
    /// An array of key-value pairs representing the form parameters for the token request.
    pub fn get_params(&self) -> [(&str, &str); 4] {
        [
            ("grant_type", "client_credentials"),
            ("client_id", &self.uid),
            ("client_secret", &self.secret),
            ("scope", "public profile projects"),
        ]
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
/// Represents an API token from the 42 API.
///
/// This struct holds the OAuth2 access token and related metadata required to make authenticated
/// requests to the 42 Intra API. It includes expiration information and token type.
///
/// The token is automatically cached to disk and reused until expiration.
pub struct FtApiToken {
    access_token: String,
    token_type: AccessTokenType,
    expires_in: i64,
    scope: String,
    created_at: i64,
    secret_valid_until: i64,
}

impl FtApiToken {
    /// Get the token value as a string.
    ///
    /// Returns the token in the format "TokenType AccessToken", which is the format required
    /// for the Authorization header in API requests.
    ///
    /// # Returns
    ///
    /// A formatted string containing the token type and access token.
    pub fn get_token_value(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
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
/// Represents an error that can occur when handling an API token.
///
/// This enum covers various error conditions that can occur during token management,
/// including I/O errors, serialization errors, token expiration, and build failures.
pub enum TokenError {
    /// An I/O error occurred.
    IOError(io::Error),
    /// An error occurred during JSON serialization or deserialization.
    SerdeError(SerdeError),
    /// The token has expired.
    TokenExpired,
    /// The token lifetime could not be parsed.
    TokenLifeTimeParsingFailed,
    /// The temporary token was not found.
    TempTokenNotFound,
    /// An error occurred while building the token.
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

    /// Try to get a token from the cache, or build a new one if it's not available.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to build a new token.
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

    /// This function always remove saved token, and try to build new token from given auth info.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to build a new token.
    /// This function will `NOT` return an error if it fails to remove `previous token` or to build a
    /// `new token`.
    pub async fn revoke(info: AuthInfo) -> Result<FtApiToken, TokenError> {
        let _ = std::fs::remove_file(Self::__get_tmp_path());

        let token = FtApiToken::build(info)
            .await
            .map_err(TokenError::BuildError)?;

        let _ = token.save();

        Ok(token)
    }

    /// Save the token to the cache.
    ///
    /// # Errors
    ///
    /// This function will return an error if it fails to create the cache file or write to it.
    pub fn save(&self) -> Result<(), TokenError> {
        let tmpdir = std::env::temp_dir().join(".ft_api_auth_token");
        let mut token = File::create_new(tmpdir)?;
        token.write_all(serde_json::to_string(self).unwrap().as_bytes())?;
        Ok(())
    }

    /// Build a new token from the given `AuthInfo`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the request to the API fails or if the response cannot be parsed.
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
