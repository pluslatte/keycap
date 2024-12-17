use std::str::FromStr;

use serde_json::json;

struct ApiEndpoint {
    server_url: String,
}

impl ApiEndpoint {
    fn new(server_url: &str) -> ApiEndpoint {
        ApiEndpoint {
            server_url: String::from_str(server_url).unwrap(),
        }
    }

    fn url_of(&self, endpoint_name: &str) -> String {
        format!("{}/api/{}", self.server_url, endpoint_name)
    }
}

pub struct MisskeyApi {
    server_domain: String,
    token: String,
}

impl MisskeyApi {
    pub fn new(server_domain: &str, token: &str) -> MisskeyApi {
        MisskeyApi {
            server_domain: server_domain.to_string(),
            token: token.to_string(),
        }
    }

    fn api_endpoint(&self) -> ApiEndpoint {
        ApiEndpoint::new(format!("https://{}", self.server_domain).as_str())
    }

    async fn post_misskey_api(
        &self,
        name_endpoint: &str,
        payload: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let endpoint = self.api_endpoint();

        let reqwest_client = reqwest::Client::new();
        let request = reqwest_client.post(endpoint.url_of(name_endpoint));
        let response = match payload {
            Some(body) => request
                .header("Content-Type", "application/json")
                .json(&body),
            None => request,
        }
        .send()
        .await
        .map_err(|error| error.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|error| error.to_string())?;

        Ok(response)
    }

    pub async fn create_note(&self, text: &str) -> Result<serde_json::Value, String> {
        let payload = json!({
            "visivility": "public",
            "text": text,
            "i": self.token,
        });
        self.post_misskey_api("notes/create", Some(payload)).await
    }

    pub async fn get_i(&self) -> Result<serde_json::Value, String> {
        let payload = json!({
            "i": self.token
        });
        self.post_misskey_api("i", Some(payload)).await
    }
}
