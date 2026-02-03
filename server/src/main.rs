// Dimensionless Developments Rust Link Web Crawler

use axum::{
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use dimensionless_crawler_core::crawl_and_check;

#[derive(Deserialize)]
struct CrawlRequest {
    url: String,
    depth: Option<usize>,
}

#[derive(Serialize)]
struct CrawlResponse {
    links: Vec<dimensionless_crawler_core::LinkResult>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("server/static"))
        .route(
            "/api/crawl",
            post(|Json(payload): Json<CrawlRequest>| async move {
                let depth = payload.depth.unwrap_or(1);
                println!("Crawling {} with depth {}", payload.url, depth);
                let links = match crawl_and_check(&payload.url, depth).await {
                    Ok(l) => {
                        println!("Successfully crawled, found {} links", l.len());
                        l
                    }
                    Err(e) => {
                        eprintln!("Crawl failed: {}", e);
                        println!("Crawl failed: {}", e);
                        vec![]
                    }
                };
                Json(CrawlResponse { links })
            }),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
