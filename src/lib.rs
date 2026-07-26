use reqwest::{Client, Result, Url};
use scraper::{Html, Selector};

pub async fn page_links(client: &Client, url: &Url) -> Result<Vec<Url>> {
    let body = client
        .get(url.clone())
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let doc = Html::parse_document(&body);
    let link_selector = Selector::parse("a[href]").unwrap();
    Ok(
        doc
            .select(&link_selector)
            .flat_map(|a_elem| {
                let link = a_elem.attr("href").unwrap();
                url.join(link)
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, Url};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };
    use super::page_links;

    #[tokio::test]
    async fn page_links_returns_absolute_and_relative_links() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/page"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(
                    r#"
                    <html>
                    <body>
                       <a href="/about">About</a>
                       <a href="contact">Contact</a>
                       <a href="https://example.com/docs">Docs</a>
                    </body>
                    </html>
                    "#
                ),
            )
            .mount(&server)
            .await;
        let client = Client::new();
        let page_url = Url::parse(&format!("{}/page", server.uri())).unwrap();
        let links = page_links(&client, &page_url).await.unwrap();

        assert_eq!(
            links,
            vec![
                Url::parse(&format!("{}/about", server.uri())).unwrap(),
                Url::parse(&format!("{}/contact", server.uri())).unwrap(),
                Url::parse("https://example.com/docs").unwrap(),
            ],
        );
    }
    #[tokio::test]
    async fn page_links_returns_error_for_http_404_status() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = Client::new();
        let page_url = Url::parse(&format!("{}/missing", server.uri())).unwrap();

        assert!(page_links(&client, &page_url).await.is_err());
    }
}