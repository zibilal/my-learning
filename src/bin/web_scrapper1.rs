use anyhow::Result;
use reqwest::{Client, Url};
use my_learning::page_links;

#[tokio::main]
async fn main() -> Result<()> {
    let Some(url) = std::env::args().nth(1) else {
        anyhow::bail!("You didn't provide the server url");
    };

    let link = Url::parse(&url)?;
    let client = Client::new();
    let links = page_links(&client, &link).await?;

    for link in links {
        println!("{}", link);
    }

    Ok(())
}