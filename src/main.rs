use std::{collections::HashMap, str::FromStr};

use serde_json::json;
use warp::Filter;

#[tokio::main]
async fn main() {
    // HEY! keep in mind that warp::path("hoge").and(warp::fs::dir("somewhere/something")) WON'T WORK!
    let front = warp::fs::dir("front/build");

    let note =
        warp::post()
            .and(warp::body::json())
            .and_then(|body: HashMap<String, String>| async move {
                match create_note(body.get("text").unwrap().to_string()).await {
                    Ok(_) => {}
                    Err(error) => {
                        println!("note creation failure: {}", error)
                    }
                };
                println!("note created");
                Ok::<&str, warp::Rejection>("ok")
            });
    warp::serve(warp::any().and(front.or(note)))
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

struct MisskeyServer {
    url: String,
}

impl MisskeyServer {
    fn new(domain: &str) -> MisskeyServer {
        MisskeyServer {
            url: format!("https://{}", domain),
        }
    }

    fn api_endpoint(&self) -> ApiEndpoint {
        ApiEndpoint::new(&self.url)
    }
}

async fn post_misskey_api(
    name_endpoint: &str,
    payload: Option<serde_json::Value>,
) -> Result<serde_json::Value, String> {
    let server_domain = "virtualkemomimi.net";
    let server = MisskeyServer::new(server_domain);
    let endpoint = server.api_endpoint();

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

async fn create_note(text: String) -> Result<serde_json::Value, String> {
    let token = "";
    let payload = json!({
        "visivility": "public",
        "text": &text,
        "i": &token
    });
    post_misskey_api("notes/create", Some(payload)).await
}
