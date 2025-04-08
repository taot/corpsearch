use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter};
use anyhow::Result;

struct PageContent {
    url: String,
    title: String,
    text: String
}

fn read_content_file(file_path: &Path) -> Result<PageContent> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    reader.read_line(&mut line)?;
    let url = line.trim_end().to_string();
    line.clear();

    reader.read_line(&mut line)?;
    let title = line.trim_end().to_string();
    line.clear();

    reader.read_line(&mut line)?;
    line.clear();

    let mut text = String::new();
    reader.read_to_string(&mut text)?;

    let page_content = PageContent {
        url: url,
        title: title,
        text: text.to_string()
    };

    Ok(page_content)
}

fn main() -> Result<()> {
    let index_path = Path::new("./data/index");
    fs::create_dir_all(index_path).unwrap_or_default();

    let mut schema_builder = Schema::builder();

    let url = schema_builder.add_text_field("url", STRING | STORED);
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let text = schema_builder.add_text_field("text", TEXT);
    let schema = schema_builder.build();

    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let content_dir = Path::new("./data/content");

    for entry in fs::read_dir(content_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip if it's not a file
        if path.is_file() {
            println!("Reading file: {:?}", path);
            let page_content = read_content_file(path.as_path())?;

            index_writer.add_document(doc!(
                url => page_content.url,
                title => page_content.title,
                text => page_content.text
            ))?;
        }
    }

    index_writer.commit()?;

    Ok(())
}
