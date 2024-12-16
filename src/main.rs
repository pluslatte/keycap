use anyhow::Ok;
use misskey::prelude::*;
use misskey::HttpClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = HttpClient::builder("https://virtualkemomimi.net/api/")
        .token("WzVTz8JZKRwD1H5iErKr1W10HeWfdsPV")
        .build()?;

    if let Err(e) = client.create_note("test").await {
        println!("{:?}", e);
    }

    Ok(())
}
