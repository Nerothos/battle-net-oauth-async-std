use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use surf::Url;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for OAuthToken {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string1 = format!("access_token: {}\n", self.access_token);
        let string2 = format!("{} token_type: {}\n", string1, self.token_type);
        let string3 = format!("{} expires_in: {}\n", string2, self.expires_in);
        write!(f, "{}", string3)
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatedToken {
    pub scope: Vec<String>,
    pub exp: i64,
    pub authorities: Vec<Authority>,
    pub client_id: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authority {
    pub authority: String,
}
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for ValidatedToken {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string1 = format!("scope: {:?}\n", self.scope);
        let string2 = format!("{} exp: {:?}\n", string1, self.exp);
        let string3 = format!("{} authorities: {:?}\n", string2, self.authorities);
        let string4 = format!("{} client_id: {}\n", string3, self.client_id);
        write!(f, "{}", string4)
    }
}

/// To retrieve a token, you need to provide your client_id and client_secret as well as a region (US, EU, APAC or CN)
///
/// ```rust
/// let token = battle_net_oauth_async_std::get_oauth_token("client_id", "client_secret", "region");
/// ```
pub async fn get_oauth_token(
    client_id: &str,
    client_secret: &str,
    region: &str,
) -> Result<OAuthToken, Box<dyn std::error::Error>> {
    let url = if region.to_lowercase() == "cn" {
        "https://www.battlenet.com.cn/oauth/token".to_string()
    } else {
        format!("https://{}.battle.net/oauth/token", region.to_lowercase())
    };

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    let url = Url::parse_with_params(&url, &params).unwrap();

    let client = surf::Client::new();
    let req = surf::post(&url);
    let mut resp = client.send(req).await?;
    let token: OAuthToken = resp.body_json().await?;

    Ok(token)
}

/// To validate a token, you need to provide your access token
///
/// ```rust
/// let token = battle_net_oauth_async_std::validate_token("access_token", "region");
/// ```
pub async fn validate_token(
    access_token: &str,
    region: &str,
) -> Result<ValidatedToken, Box<dyn std::error::Error>> {
    let url = if region.to_lowercase() == "cn" {
        "https://www.battlenet.com.cn/oauth/check_token".to_string()
    } else {
        format!(
            "https://{}.battle.net/oauth/check_token",
            region.to_lowercase()
        )
    };
    let mut params = HashMap::new();
    params.insert("token", &access_token);
    let url = Url::parse_with_params(&url, &params).unwrap();

    let client = surf::Client::new();
    let req = surf::get(&url);
    let mut resp = client.send(req).await?;
    let validated_token: ValidatedToken = resp.body_json().await?;

    Ok(validated_token)
}
