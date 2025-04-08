use std::path::Path;
use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{Document, Index, ReloadPolicy, TantivyDocument};

fn main() -> Result<()> {
    let index_path = Path::new("./data/index");

    let index = Index::open_in_dir(index_path)?;
    let schema = index.schema();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    let searcher = reader.searcher();

    let title = schema.get_field("title")?;
    let text = schema.get_field("text")?;

    let query_parser = QueryParser::for_index(&index, vec![title, text]);

    let query = query_parser.parse_query("global economy")?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}, score = {}", retrieved_doc.to_json(&schema), score);
    }

    Ok(())
}
