// Dimensionless Developments Rust Link Web Crawler

use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::{HashSet, VecDeque};
use url::Url;

#[derive(Debug, Serialize)]
pub struct LinkResult {
    pub url: String,
    pub status: Option<u16>,
    pub ok: bool,
}

pub async fn crawl_and_check(start: &str, max_depth: usize) -> Result<Vec<LinkResult>, Box<dyn std::error::Error + Send + Sync>> {
    let start_url = Url::parse(start)?;
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let mut seen: HashSet<String> = HashSet::new();
    let mut results: Vec<LinkResult> = Vec::new();

    let mut queue: VecDeque<(Url, usize)> = VecDeque::new();
    queue.push_back((start_url.clone(), 0));
    seen.insert(start_url.to_string());

    // First, check the starting URL itself
    eprintln!("Checking initial URL: {}", start_url);
    let _start_status = match client.head(start_url.clone()).send().await {
        Ok(r) => {
            let s = r.status().as_u16();
            eprintln!("Initial URL {} returned status {}", start_url, s);
            Some(s)
        }
        Err(e) => {
            eprintln!("Failed to check initial URL {}: {}", start_url, e);
            // Try GET instead of HEAD
            match client.get(start_url.clone()).send().await {
                Ok(r) => {
                    let s = r.status().as_u16();
                    eprintln!("Initial URL {} (GET) returned status {}", start_url, s);
                    Some(s)
                }
                Err(e2) => {
                    eprintln!("Failed to GET initial URL {}: {}", start_url, e2);
                    None
                }
            }
        }
    };

    while let Some((url, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }

        eprintln!("Fetching page at depth {}: {}", depth, url);

        // fetch page
        let resp = client.get(url.clone()).send().await;
        let body = match resp {
            Ok(r) => {
                let status_code = r.status().as_u16();
                eprintln!("Successfully fetched {}, status: {}", url, status_code);
                r.text().await.unwrap_or_default()
            }
            Err(e) => {
                eprintln!("Failed to fetch {}: {}", url, e);
                String::new()
            }
        };

        if body.is_empty() {
            eprintln!("Body is empty for {}", url);
            continue;
        }

        // parse links into owned Urls inside its own scope so `Html` is dropped
        // before we perform any async requests.
        let resolved: Vec<Url> = {
            let document = Html::parse_document(&body);
            let selector = Selector::parse("a").unwrap();
            let mut res: Vec<Url> = Vec::new();
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    if href.is_empty() || href.starts_with("#") {
                        continue;
                    }
                    let joined = match Url::parse(href) {
                        Ok(u) => u,
                        Err(_) => match url.join(href) {
                            Ok(u2) => u2,
                            Err(_) => continue,
                        },
                    };
                    res.push(joined);
                }
            }
            res
        };

        eprintln!("Found {} links on {}", resolved.len(), url);

        // now process resolved links (owned Urls) with async requests
        for joined in resolved {
            // Only crawl same-domain links
            if joined.domain() != url.domain() {
                continue;
            }

            let link_str = joined.to_string();
            if seen.contains(&link_str) {
                continue;
            }
            seen.insert(link_str.clone());

            eprintln!("Checking link: {}", link_str);

            // check status - try HEAD first, fall back to GET
            let status = match client.head(joined.clone()).send().await {
                Ok(r) => {
                    let s = r.status().as_u16();
                    eprintln!("Link {} returned status {}", link_str, s);
                    Some(s)
                }
                Err(_) => {
                    // Try GET instead of HEAD
                    match client.get(joined.clone()).send().await {
                        Ok(r) => {
                            let s = r.status().as_u16();
                            eprintln!("Link {} (GET) returned status {}", link_str, s);
                            Some(s)
                        }
                        Err(e) => {
                            eprintln!("Failed to check {}: {}", link_str, e);
                            None
                        }
                    }
                }
            };

            let ok = status.map(|s| s < 400).unwrap_or(false);
            results.push(LinkResult { url: link_str.clone(), status, ok });

            if depth + 1 <= max_depth {
                queue.push_back((joined, depth + 1));
            }
        }
    }

    eprintln!("Crawl complete. Total results: {}", results.len());
    Ok(results)
}
