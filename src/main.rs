use std::collections::HashMap;

use warp::{reply::Response, Filter};

use keycap::MisskeyApi;

#[tokio::main]
async fn main() {
    // HEY! keep in mind that warp::path("hoge").and(warp::fs::dir("somewhere/something")) WON'T WORK!
    let front = warp::fs::dir("front/build");

    // Filter POST request from front-end
    let post = warp::post().and(warp::body::json()).and_then(
        |body: HashMap<String, String>| async move {
            println!("\ngot POST request with body");
            if let (Some(server_domain), Some(token)) =
                (body.get("server_domain"), body.get("token"))
            {
                println!("body of the request had `server_domain` and `token` in it");
                // Target server and access token
                let misskey_api = MisskeyApi::new(server_domain, token);
                println!("target server is {}, with token {}", server_domain, token);

                // If request from front-end had "text" in its body
                match body.get("text") {
                    Some(text) => {
                        println!("body of the request had `text` in it");
                        println!("trying to create an note with given text {}", text);
                        match misskey_api.create_note(text).await {
                            Ok(_) => {
                                println!("note created: {}", text);
                            }
                            Err(error) => {
                                println!("note creation failure: {}", error)
                            }
                        };
                        println!("send text response and return");
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new("ok".into()),
                        );
                    }
                    None => {
                        println!("body of the request does not have `text`");
                    }
                };

                // If request from front-end had "request_type" in its body
                match body.get("request_type") {
                    Some(request_type) => {
                        println!("body of the request had `request_type` in it");
                        // If "request_type" was "username"
                        if request_type == "username" {
                            println!("`request_type` was `{}`", request_type);
                            println!("trying to get user's username");
                            return match misskey_api.get_i().await {
                                Ok(val) => {
                                    let name = val["name"]
                                        .as_str()
                                        .unwrap_or("Error: Username was empty")
                                        .to_string();
                                    println!("fetched user's username: {}", name);
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
                        // Else if "request_type" was "timelineHome"
                        else if request_type == "timelineHome"
                            || request_type == "timelineLocal"
                            || request_type == "timelineSocial"
                            || request_type == "timelineGlobal"
                        {
                            println!("`request_type` was `{}`", request_type);
                            let timeline_opt = match request_type.as_str() {
                                "timelineHome" => {
                                    println!("trying to get HOME timeline");
                                    misskey_api.get_timeline_home().await
                                }
                                "timelineLocal" => {
                                    println!("trying to get LOCAL timeline");
                                    misskey_api.get_timeline_local().await
                                }
                                "timelineSocial" => {
                                    println!("trying to get SOCIAL timeline");
                                    misskey_api.get_timeline_social().await
                                }
                                "timelineGlobal" => {
                                    println!("trying to get GLOBAL timeline");
                                    misskey_api.get_timeline_global().await
                                }
                                _ => unreachable!(),
                            };
                            return match timeline_opt {
                                Ok(val) => {
                                    println!("got: {}", val);
                                    Ok(Response::new(val.to_string().into()))
                                }
                                Err(error) => {
                                    println!("could not get home timeline");
                                    Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                                        Response::new(
                                            format!(
                                                "Error: Could not get home timeline: {}",
                                                error
                                            )
                                            .into(),
                                        ),
                                    )
                                }
                            };
                        }
                        println!("invalid `request_type`: {}", request_type);
                    }
                    None => {
                        println!("body of the request does not have `request_type`");
                    }
                };

                println!("invalid POST request");
            }
            Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                "invalid".into(),
            ))
        },
    );
    warp::serve(warp::any().and(front.or(post)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
