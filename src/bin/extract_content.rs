use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};
use dom_smoothie::{Article, Config, Readability};

use std::fs::File;
use corpsearch::decode_url;

fn read_file_content(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn extract_text(html: &str) -> Result<Article> {
    let cfg = Config {
        ..Default::default()
    };
    let mut readability = Readability::new(html, None, Some(cfg))?;

    let article: Article = readability.parse()?;

    println!("{:<15} {}","Title:", article.title);
    println!("{:<15} {:?}","Byline:", article.byline);
    println!("{:<15} {}","Length:", article.length);
    println!("{:<15} {:?}","Excerpt:", article.excerpt);
    println!("{:<15} {:?}","Site Name:", article.site_name);
    println!("{:<15} {:?}", "Dir:", article.dir);
    println!("{:<15} {:?}","Published Time:", article.published_time);
    println!("{:<15} {:?}","Modified Time:", article.modified_time);
    println!("{:<15} {:?}","Image:", article.image);
    println!("Text Content: {}", article.text_content);

    Ok(article)
}

fn write_content_file(file_path: &Path, url: &str, article: &Article) -> Result<()> {
    let mut file = File::create(file_path)?;

    writeln!(file, "{}", url)?;
    writeln!(file, "{}", article.title)?;
    writeln!(file)?;

    writeln!(file, "{}", article.text_content)?;

    Ok(())
}

fn get_url_from_path(path: &PathBuf) -> Result<String> {
    let encoded_url = path.file_name().ok_or(anyhow!("Bad file path: {}", path.display()))?.to_str().ok_or(anyhow!("Bad file path: {}", path.display()))?;
    decode_url(encoded_url)
}

fn main() -> Result<()> {
    let download_dir = Path::new("./data/download");
    
    // Check if directory exists
    if !download_dir.exists() {
        return Err(anyhow::anyhow!("Directory {} does not exist", download_dir.display()));
    }

    let json_dir = Path::new("./data/content");
    std::fs::create_dir_all(json_dir).unwrap_or_default();

    let entries = fs::read_dir(download_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            println!("\nReading file: {}", path.display());
            let url = get_url_from_path(&path)?;
            println!("URL: {}", url);

            let content = read_file_content(&path)?;
            let article = extract_text(&content)?;

            let json_path = json_dir.join(path.file_name().ok_or(anyhow!("Bad file path: {}", path.display()))?);
            write_content_file(json_path.as_path(), &url, &article)?;

            println!();
        }
    }

    Ok(())
}
