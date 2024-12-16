use warp::Filter;

#[tokio::main]
async fn main() {
    let html_str = "<!DOCTYPE html>
    <html>
        <head>
            <title>skibidi</title>
        </head>
        <body>
            <h1>Skibidi</h1>
        </body>
    </html>";
    let test = warp::path("hoge").map(|| warp::reply::html(html_str.to_string()));

    warp::serve(test).run(([127, 0, 0, 1], 3030)).await;
}
