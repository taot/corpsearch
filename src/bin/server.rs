use std::path::Path;
use anyhow::Result;

use rouille::{router, Request};
use rouille::Response;
use serde::Serialize;
use tantivy::{Index, ReloadPolicy};

#[derive(Serialize)]
struct SearchResults {
    query: String,
    data: Vec<String>
}

fn index() -> Response {
    Response::text("Hello, world!")
}

fn search(request: &Request) -> Response {
    let query = request.get_param("query");
    println!("query: {:?}", query);
    let results = SearchResults { query: "Hello, World!".to_string(), data: vec!["a".to_string(), "b".to_string()] };
    Response::json(&results)
}

fn not_found() -> Response {
    Response::empty_404()
}

fn main() -> Result<()> {

    let index_path = Path::new("./data/index");

    let index = Index::open_in_dir(index_path)?;
    let schema = index.schema();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
            (GET) (/) => {
                index()
            },
            (GET) (/search) => {
                search(&request)
            },
            _ => {
                not_found()
            }
        )
    });
}
