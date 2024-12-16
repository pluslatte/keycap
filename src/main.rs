use warp::Filter;

#[tokio::main]
async fn main() {
    // HEY! keep in mind that warp::path("hoge").and(warp::fs::dir("somewhere/something")) WON'T WORK!
    let front = warp::fs::dir("front/build");

    let note = warp::post().map(|| {
        println!("POST");
        "ok"
    });
    warp::serve(warp::any().and(front.or(note)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
