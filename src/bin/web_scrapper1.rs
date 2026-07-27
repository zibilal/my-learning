use anyhow::Result;
use reqwest::{Client, Url};
use my_learning::page_links;

#[tokio::main]
async fn main() -> Result<()> {
    let Some(url) = std::env::args().nth(1) else {
      anyhow::bail!("Usage: web_scrapper1 url");
    };
    let url = Url::parse(&url)?;
    let links = page_links(&Client::new(), &url).await?;
    for link in links {
        println!("{}", link);
    }
    Ok(())
}