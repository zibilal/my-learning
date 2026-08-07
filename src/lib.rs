mod my_reactor;

mod rec_list {
    #[derive(Debug)]
    pub enum List {
        Cons(i32, Box<List>),
        Nil,
    }
}

mod shoes {
    #[derive(PartialEq, Debug)]
    pub struct Shoe {
        pub size: u32,
        pub style: String,
    }

    pub fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[cfg(test)]
    mod tests_shoes {
        use super::*;
        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe { size: 10, style: String::from("sneaker")},
                Shoe { size: 13, style: String::from("sandal") },
                Shoe { size: 10, style: String::from("boot") },
            ];
            let in_my_size = shoe_in_size(shoes, 10);
            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker"),
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot"),
                    }
                ]
            );
        }
    }
}

pub mod scrap_links {
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

    use std::collections::HashSet;
    use std::sync::Mutex;
    use netiquette::Limiter;

    pub struct Traversal {
        client: Client,
        limiter: Limiter,
        pub seen: Mutex<HashSet<Url>>,
    }

    impl Traversal {
        pub fn new(client: Client, user_agent: String) -> Self {
            Self {
                seen: Mutex::new(HashSet::new()),
                client: client.clone(),
                limiter: Limiter::new(client, user_agent),
            }
        }
    }

    use std::sync::Arc;
    use tokio::task::JoinSet;

    pub async fn visit(traversal: Arc<Traversal>, mut url: Url, depth: usize) {
        url.set_fragment(None);
        if !traversal.seen.lock().unwrap().insert(url.clone()) {
            return;
        }
        if depth == 0 {
            return;
        }

        let permit = match traversal.limiter.acquire(&url).await {
            Ok(permit) => permit,
            Err(err) => {
                eprintln!("Skipping <{url}>: {err}");
                return;
            }
        };

        let links = match page_links(&traversal.client, &url).await {
            Ok(links) => links,
            Err(err) => {
                eprintln!("Error accessing {url}: {err}");
                permit.note_error(&url, err);
                return;
            }
        };
        drop(permit);

        let mut join_set = JoinSet::new();
        for link in links {
            spawn_visit(Arc::clone(&traversal), link, depth-1, &mut join_set)
        }
        join_set.join_all().await;
    }

    fn spawn_visit(traversal: Arc<Traversal>, url: Url, depth: usize, join_set: &mut JoinSet<()>) {
        join_set.spawn(visit(traversal, url, depth));
    }


    #[cfg(test)]
    mod tests {
        use crate::scrap_links::*;
        use wiremock::{
            matchers::{method, path},
            MockServer, Mock, ResponseTemplate
        };

        use reqwest::{Client, Url};
        use std::sync::Arc;
        use std::collections::HashSet;

        const USER_AGENT: &str = "crawler-tests/1.0";
        fn traversal() -> Arc<Traversal> {
            Arc::new(Traversal::new(Client::new(), USER_AGENT.to_string()))
        }

        fn url(s: &str) -> Url {
            Url::parse(s).expect("A valid URL")
        }

        #[tokio::test]
        async fn success_fetch_recurses_into_links_with_decremented_depth() {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/root"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_string(r#"<html><body>
                   <a href="/child-a">A</a>
                   <a href="/child-b">B</a>
                </body></html>"#)
                )
                .mount(&server)
                .await;
            for link in ["/child-a", "/child-b"] {
                Mock::given(method("GET"))
                    .and(path(link))
                    .respond_with(ResponseTemplate::new(200).set_body_string("<html></html>"))
                    .mount(&server)
                    .await;
            }

            let t = traversal();
            let root_url = url(&format!("{}/root", server.uri()));
            visit(Arc::clone(&t), root_url, 2).await;

            let requests = server.received_requests().await.unwrap();
            let page_links: HashSet<_> =
                requests.iter()
                    .map(|r|r.url.path().to_string())
                    .filter(|p|p != "/robots.txt")
                    .collect();

            assert_eq!(page_links, HashSet::from(["/root".to_string(), "/child-a".to_string(), "/child-b".to_string()]) );
        }

        #[tokio::test]
        async fn fetch_error_stop_recursion() {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/broken"))
                .respond_with(ResponseTemplate::new(500))
                .mount(&server)
                .await;

            let t = traversal();
            let target = url(&format!("{}/broken", server.uri()));
            visit(t, target, 1).await;

            let requests = server.received_requests().await.unwrap();
            assert!(
                requests.iter().all(|r|r.url.path() != "/broken/child")
            );
        }

        #[tokio::test]
        async fn robots_disallow_prevents_fetch() {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/robots.txt"))
                .respond_with(ResponseTemplate::new(200).set_body_string("User-agent: *\nDisallow: /secret\n"))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/secret"))
                .respond_with(ResponseTemplate::new(200).set_body_string("<htm></html>"))
                .mount(&server)
                .await;

            let t = traversal();
            let target = url(&format!("{}/secret", server.uri()));
            visit(t, target, 1).await;

            let requests = server.received_requests().await.unwrap();
            let page_hits = requests.iter().filter(|r|r.url.path() == "/secret").count();
            assert_eq!(page_hits, 0);
        }

        #[tokio::test]
        async fn revisiting_a_seen_url_is_a_no_op() {
            let t = traversal();
            let target = url("http://example.invalid/only-once");

            visit(Arc::clone(&t), target.clone(), 1).await;
            assert_eq!(t.seen.lock().unwrap().len(), 1);

            visit(Arc::clone(&t), target.clone(), 1).await;
            assert_eq!(t.seen.lock().unwrap().len(), 1);
        }

        #[tokio::test]
        async fn fragment_are_stripped_before_dedup() {
            let t = traversal();
            let with_fragment_a = url("https://example.invalid/page#fragment_a");
            let with_fragment_b = url("https://example.invalid/page#fragment_b");
            let without_fragment = url("https://example.invalid/page");

            visit(Arc::clone(&t), with_fragment_a, 0).await;
            visit(Arc::clone(&t), with_fragment_b, 0).await;

            let seen = t.seen.lock().unwrap();
            assert_eq!(seen.len(), 1);
            assert!(seen.contains(&without_fragment))
        }

        #[tokio::test]
        async fn depth_zero_marks_seen_but_does_not_fetch() {
            let t = traversal();
            let target = url("http://depth-zero.invalid/page");
            visit(Arc::clone(&t), target.clone(), 1).await;
            assert!(t.seen.lock().unwrap().contains(&target))
        }
    }
}