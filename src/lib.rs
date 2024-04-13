use std::collections::{hash_map, HashMap};

use serde::{Deserialize, Serialize};

struct AuthInfo {
    uid: String,
    secret: String,
}

impl AuthInfo {
    fn from_env(uid: String, secret: String) -> AuthInfo {
        AuthInfo { uid, secret }
    }
}

#[derive(Deserialize, Serialize)]
struct AccessToken {
    access_token: String,
    token_type: AccessTokenType,
    expires_in: i32,
    scope: AccessTokenScope,
    created_at: i32,
}

#[derive(Deserialize, Serialize)]
enum AccessTokenScope {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "projects")]
    Projects,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "elearning")]
    Elearning,
    #[serde(rename = "tig")]
    Tig,
    #[serde(rename = "forum")]
    Forum,
}

#[derive(Deserialize, Serialize)]
enum AccessTokenType {
    #[serde(rename = "bearer")]
    Bearer,
}

impl AccessToken {
    async fn build(info: AuthInfo) -> Result<AccessToken, String> {
        let mut map = HashMap::new();
        map.insert(String::from("client_id"), info.uid);
        map.insert(String::from("client_secret"), info.secret);

        let client = reqwest::Client::new();
        let res = client
            .post("http://api.intra.42.fr/oauth/token")
            .form(&map)
            .send()
            .await
            .map_err(|e| format!("Error: {e}"))
            .expect("Access token request itself failed");

        match res.status() {
            reqwest::StatusCode::OK => res
                .json::<AccessToken>()
                .await
                .map_err(|e| format!("Error: {e}")),
            reqwest::StatusCode::UNAUTHORIZED => Err(format!(
                "Error: got UNAUTHORIZED code: {:?}",
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

    #[test]
    fn build() {
        let auth_info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").expect("UID not exist"),
            config_env_var("FT_API_CLIENT_SECRET").expect("SECRET not exist"),
        );
    }

    #[test]
    fn get_token() {
        let info = AuthInfo::from_env(
            config_env_var("FT_API_CLIENT_UID").expect("UID not exist"),
            config_env_var("FT_API_CLIENT_SECRET").expect("SECRET not exist"),
        );
        let res = AccessToken::build(info);
    }
}
