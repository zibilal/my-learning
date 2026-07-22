use anyhow::Result;

async fn make_request() -> Result<String> {
    let url = "https://rust-lang.org/";
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(response)
}
fn main() -> Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    let response = runtime.block_on(make_request())?;
    println!("{}", response);
    Ok(())
}