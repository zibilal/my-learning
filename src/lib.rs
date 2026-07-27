use reqwest::{Client, Url, Result};
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
    let selector = Selector::parse("a[href]").unwrap();
    Ok(
        doc
            .select(&selector)
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
    async fn page_links_returns_absolute_and_relative_urls() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/page"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"
                <a href="/about">About</a>
                <a href="contacts">Contacts</a>
                <a href="https://example.com/docs">Documents</a>
                "#
            ))
            .mount(&server)
            .await;
        let client = Client::new();
        let url = Url::parse(&format!("{}/page", server.uri())).unwrap();
        let links = page_links(&client, &url).await.unwrap();
        assert_eq!(links,
            vec![
                Url::parse(&format!("{}/about", server.uri())).unwrap(),
                Url::parse(&format!("{}/contacts", server.uri())).unwrap(),
                Url::parse("https://example.com/docs").unwrap(),
            ]
        );
    }

    #[tokio::test]
    async fn page_links_returns_error_when_server_gives_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/path"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = Client::new();
        let url = Url::parse(&format!("{}/path", server.uri())).unwrap();
        assert!(page_links(&client, &url).await.is_err());
    }
}