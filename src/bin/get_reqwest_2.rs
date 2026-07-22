use anyhow::Result;

async fn make_request() -> Result<String> {
    let url = "https://rust-lang.org/";
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    let response = make_request().await?;
    println!("{}", response);
    Ok(())
}