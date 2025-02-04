use std::{collections::HashMap, net::SocketAddrV4, path::PathBuf, str::FromStr};

use clap::{builder::RangedU64ValueParser, Arg, Command};
use warp::{reply::Response, Filter};

use keycap::MisskeyApi;

#[tokio::main]
async fn main() {
    const GIT_COMMIT_HASH: &str = env!("GIT_HASH");
    const CLIENT_PATH: &str = env!("CLIENT_PATH");
    let matches = Command::new("keycap")
        .version(GIT_COMMIT_HASH)
        .about("Server program which provides an alternative, light-weight web client for Misskey.")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("http port to listen on")
                .value_parser(RangedU64ValueParser::<u64>::new().range(1..=65535)),
        )
        .get_matches();

    let client_path = PathBuf::from_str(CLIENT_PATH).unwrap();
    println!("client path: {}", client_path.display());
    assert!(
        client_path.exists(),
        "Invalid client path (which should be embedded on build)"
    );
    println!("confirmed");

    println!("starting keycap server...");
    // HEY! keep in mind that warp::path("hoge").and(warp::fs::dir("somewhere/something")) WON'T WORK!
    let front = warp::fs::dir(client_path);

    let version = warp::path("version").and_then(|| async move {
        Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
            GIT_COMMIT_HASH.into(),
        ))
    });

    let create_note = warp::path("create_note").and(warp::body::json()).and_then(
        |body: HashMap<String, String>| async move {
            if let (Some(server_domain), Some(token), Some(text)) = (
                body.get("server_domain"),
                body.get("token"),
                body.get("text"),
            ) {
                let misskey_api = MisskeyApi::new(server_domain, token);
                match misskey_api.create_note(text).await {
                    Ok(_) => {
                        println!("note created: {}", text);
                    }
                    Err(error) => {
                        println!("note creation failure: {}", error)
                    }
                };
                return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                    Response::new("ok".into()),
                );
            }
            Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                "invalid".into(),
            ))
        },
    );

    let username = warp::path("username").and(warp::body::json()).and_then(
        |body: HashMap<String, String>| async move {
            if let (Some(server_domain), Some(token)) =
                (body.get("server_domain"), body.get("token"))
            {
                let misskey_api = MisskeyApi::new(server_domain, token);
                match misskey_api.get_i().await {
                    Ok(val) => {
                        let name = val["name"]
                            .as_str()
                            .unwrap_or("Error: Username was empty")
                            .to_string();
                        println!("fetched user's username: {}", name);
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(name.into()),
                        );
                    }
                    Err(error) => {
                        println!("could not get user's username");
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(
                                format!("Error: Could not get username: {}", error).into(),
                            ),
                        );
                    }
                };
            }
            Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                "invalid".into(),
            ))
        },
    );

    let timeline_home = warp::path("timeline_home")
        .and(warp::body::json())
        .and_then(|body: HashMap<String, String>| async move {
            if let (Some(server_domain), Some(token)) =
                (body.get("server_domain"), body.get("token"))
            {
                let misskey_api = MisskeyApi::new(server_domain, token);
                match misskey_api.get_timeline_home().await {
                    Ok(val) => {
                        println!("got: {}", val);
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(val.to_string().into()),
                        );
                    }
                    Err(error) => {
                        println!("could not get home timeline");
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(
                                format!("Error: Could not get home timeline: {}", error).into(),
                            ),
                        );
                    }
                };
            }
            Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                "invalid".into(),
            ))
        });

    let timeline_local = warp::path("timeline_local")
        .and(warp::body::json())
        .and_then(|body: HashMap<String, String>| async move {
            if let (Some(server_domain), Some(token)) =
                (body.get("server_domain"), body.get("token"))
            {
                let misskey_api = MisskeyApi::new(server_domain, token);
                match misskey_api.get_timeline_local().await {
                    Ok(val) => {
                        println!("got: {}", val);
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(val.to_string().into()),
                        );
                    }
                    Err(error) => {
                        println!("could not get local timeline");
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(
                                format!("Error: Could not get local timeline: {}", error).into(),
                            ),
                        );
                    }
                };
            }
            Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                "invalid".into(),
            ))
        });

    let timeline_social = warp::path("timeline_social")
        .and(warp::body::json())
        .and_then(|body: HashMap<String, String>| async move {
            if let (Some(server_domain), Some(token)) =
                (body.get("server_domain"), body.get("token"))
            {
                let misskey_api = MisskeyApi::new(server_domain, token);
                match misskey_api.get_timeline_social().await {
                    Ok(val) => {
                        println!("got: {}", val);
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(val.to_string().into()),
                        );
                    }
                    Err(error) => {
                        println!("could not get social timeline");
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(
                                format!("Error: Could not get social timeline: {}", error).into(),
                            ),
                        );
                    }
                };
            }
            Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                "invalid".into(),
            ))
        });

    let timeline_global = warp::path("timeline_global")
        .and(warp::body::json())
        .and_then(|body: HashMap<String, String>| async move {
            if let (Some(server_domain), Some(token)) =
                (body.get("server_domain"), body.get("token"))
            {
                let misskey_api = MisskeyApi::new(server_domain, token);
                match misskey_api.get_timeline_global().await {
                    Ok(val) => {
                        println!("got: {}", val);
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(val.to_string().into()),
                        );
                    }
                    Err(error) => {
                        println!("could not get global timeline");
                        return Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(
                            Response::new(
                                format!("Error: Could not get global timeline: {}", error).into(),
                            ),
                        );
                    }
                };
            }
            Ok::<warp::http::Response<warp::hyper::Body>, warp::Rejection>(Response::new(
                "invalid".into(),
            ))
        });

    let api = warp::post().and(warp::path("api")).and(
        version
            .or(create_note)
            .or(username)
            .or(timeline_home)
            .or(timeline_local)
            .or(timeline_social)
            .or(timeline_global),
    );

    let port_to_listen_opt: Option<&u64> = matches.get_one("port");
    let port_to_listen = match port_to_listen_opt {
        Some(port) => *port,
        None => 3030,
    };
    let socket_address = SocketAddrV4::new([0, 0, 0, 0].into(), port_to_listen.try_into().unwrap());
    println!("started. listening on: {}", socket_address);
    warp::serve(front.or(api)).run(socket_address).await;
}
