use std::{collections::HashMap, str::FromStr};

use serde_json::json;
use warp::{reply::Response, Filter};

#[tokio::main]
async fn main() {
    // HEY! keep in mind that warp::path("hoge").and(warp::fs::dir("somewhere/something")) WON'T WORK!
    let front = warp::fs::dir("front/build");

    let post =
        warp::post()
            .and(warp::body::json())
            .and_then(|body: HashMap<String, String>| async move {
                if let Some(token) = body.get("token") {
                    // Target server and access token
                    let server = MisskeyApi::new("virtualkemomimi.net", token);

                    // If request from front-end had "text" in its body
                    if let Some(text) = body.get("text") {
                        match server.create_note(text).await {
                            Ok(_) => {
                                println!("note created");
                            }
                            Err(error) => {
                                println!("note creation failure: {}", error)
                            }
                        };
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new("ok".into()),
                        );
                    };

                    // If request from front-end had "request_type" in its body
                    if let Some(request_type) = body.get("request_type") {
                        // If "request_type" was "username"
                        if request_type == "username" {
                            return match server.get_i().await {
                                Ok(val) => {
                                    println!("fetched user's username");
                                    let name = val["name"]
                                        .as_str()
                                        .unwrap_or("Error: Username was empty")
                                        .to_string();
                                    Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                                        Response::new(name.into()),
                                    )
                                }
                                Err(error) => {
                                    println!("could not get user's username");
                                    Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                                        Response::new(
                                            format!("Error: Could not get username: {}", error)
                                                .into(),
                                        ),
                                    )
                                }
                            };
                        }
                    };
                }
                Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                    "nothing".into(),
                ))
                // Ok::<String, warp::Rejection>("nothing".to_string())
            });
    warp::serve(warp::any().and(front.or(post)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

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

struct MisskeyApi {
    server_domain: String,
    token: String,
}

impl MisskeyApi {
    fn new(server_domain: &str, token: &str) -> MisskeyApi {
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

    async fn create_note(&self, text: &str) -> Result<serde_json::Value, String> {
        let payload = json!({
            "visivility": "public",
            "text": text,
            "i": self.token,
        });
        self.post_misskey_api("notes/create", Some(payload)).await
    }

    async fn get_i(&self) -> Result<serde_json::Value, String> {
        let payload = json!({
            "i": self.token
        });
        self.post_misskey_api("i", Some(payload)).await
    }
}
