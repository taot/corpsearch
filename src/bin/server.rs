use std::path::Path;
use std::sync::{Arc, Mutex};
use anyhow::Result;

use rouille::{router, Request};
use rouille::Response;
use serde::Serialize;
use tantivy::{Document, Index, IndexReader, ReloadPolicy, TantivyDocument};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Value};

struct Tantivy {
    reader: IndexReader,
    schema: Schema,
    query_parser: QueryParser
}

fn create_tantivy_index_reader() -> Result<Tantivy> {
    let index_path = Path::new("./data/index");

    let index = Index::open_in_dir(index_path)?;
    let schema = index.schema();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let title = schema.get_field("title")?;
    let text = schema.get_field("text")?;

    let query_parser = QueryParser::for_index(&index, vec![title, text]);

    Ok(Tantivy { reader, schema, query_parser })
}

#[derive(Serialize)]
struct Doc {
    url: String,
    title: String,
}

#[derive(Serialize)]
struct SearchResults {
    q: String,
    data: Vec<String>,
    docs: Vec<Doc>
}

fn index() -> Result<Response> {
    Ok(Response::text("Hello, world!"))
}

fn search(request: &Request, tantivy: &Tantivy) -> Result<Response> {
    let q = request.get_param("q");
    println!("q: {:?}", q);

    let query = tantivy.query_parser.parse_query("global economy")?;

    let searcher = tantivy.reader.searcher();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    let mut docs = vec![];

    for (score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}, score = {}", retrieved_doc.to_json(&tantivy.schema), score);

        let url_field = tantivy.schema.get_field("url")?;
        let title_field = tantivy.schema.get_field("title")?;

        let url = retrieved_doc.get_first(url_field).unwrap().as_str().unwrap().to_string();
        let title = retrieved_doc.get_first(title_field).unwrap().as_str().unwrap().to_string();

        let doc = Doc { url, title };

        docs.push(doc);
    }

    let results = SearchResults {
        q: "Hello, World!".to_string(),
        data: vec!["a".to_string(), "b".to_string()],
        docs: docs
    };

    Ok(Response::json(&results))
}

fn not_found() -> Result<Response> {
    Ok(Response::empty_404())
}

fn add_cors_headers(response: Response) -> Response {
    response.with_additional_header("Access-Control-Allow-Origin", "*")
        .with_additional_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        .with_additional_header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .with_additional_header("Access-Control-Max-Age", "3600")
}

fn main() -> Result<()> {
    let tantivy_shared = Arc::new(create_tantivy_index_reader()?);

    rouille::start_server("0.0.0.0:8080", move |request| {
        let tantivy_clone = Arc::clone(&tantivy_shared);

        let resp = if request.method() == "OPTIONS" {
            Ok(Response::empty_204())
        } else {
            router!(request,
                (GET) (/) => {
                    index()
                },
                (GET) (/search) => {
                    search(&request, &tantivy_clone)
                },
                _ => {
                    not_found()
                }
            )
        };

        let resp = resp.map(|r| {
            r.with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
                .with_additional_header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        });

        resp.unwrap_or_else(|err| {
            Response::text(err.to_string()).with_status_code(500)
        })
    });
}
