use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
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
    let client = surf::Client::new();
    let url = if region.to_lowercase() == "cn" {
        "https://www.battlenet.com.cn/oauth/token".to_string()
    } else {
        format!("https://{}.battle.net/oauth/token", region.to_lowercase())
    };
    let auth = format!(
        "Basic {}",
        base64::encode(&format!("{}:{}", client_id, client_secret))
    );

    let resp: OAuthToken = client
        .post(&url)
        .set_header("authorization".parse().unwrap(), &auth)
        .body_form(&[("grant_type", "client_credentials")])?
        .recv_json()
        .await?;

    Ok(resp)
}
