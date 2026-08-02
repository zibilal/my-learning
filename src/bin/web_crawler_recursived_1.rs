use reqwest::{Client, Url};
use std::sync::Arc;

use my_learning::scrap_links::{visit, Traversal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    const USER_AGENT: &str = "crawler-me/1.0";

    let Some(url) = std::env::args().nth(1) else {
        anyhow::bail!("You didn't provide the URL");
    };

    let start_url = Url::parse(&url)?;
    let client = Client::builder().user_agent(USER_AGENT).build()?;
    let traversal = Arc::new(Traversal::new(client, USER_AGENT.to_string()));
    visit(Arc::clone(&traversal), start_url, 2).await;

    let traversal = Arc::into_inner(traversal).unwrap();
    let seen = traversal.seen.into_inner()?;
    let mut sorted = Vec::from_iter(seen.into_iter());
    sorted.sort();

    for page in sorted {
        println!("{}", page);
    }

    Ok(())
}