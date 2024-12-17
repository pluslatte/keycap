use std::collections::HashMap;

use warp::{reply::Response, Filter};

use keycap::MisskeyApi;

#[tokio::main]
async fn main() {
    // HEY! keep in mind that warp::path("hoge").and(warp::fs::dir("somewhere/something")) WON'T WORK!
    let front = warp::fs::dir("front/build");

    // Filter POST request from front-end
    let post =
        warp::post()
            .and(warp::body::json())
            .and_then(|body: HashMap<String, String>| async move {
                if let (Some(server_domain), Some(token)) =
                    (body.get("server_domain"), body.get("token"))
                {
                    // Target server and access token
                    let misskey_api = MisskeyApi::new(server_domain, token);

                    // If request from front-end had "text" in its body
                    if let Some(text) = body.get("text") {
                        match misskey_api.create_note(text).await {
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
                            return match misskey_api.get_i().await {
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
            });
    warp::serve(warp::any().and(front.or(post)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
