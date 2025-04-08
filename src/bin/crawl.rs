//! `cargo run --example download`
extern crate env_logger;
extern crate spider;

use spider::utils::log;
use spider::website::Website;

use env_logger::Env;
use spider::tokio;
use std::fs::OpenOptions;
use std::io::Write;

use corpsearch::encode_url;

#[tokio::main]
async fn main() {
    let download_path = "./data/download";
    std::fs::create_dir_all(download_path).unwrap_or_default();

    let env = Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let website_name = "https://spider.cloud";

    let mut website: Website = Website::new(website_name);

    website.scrape().await;

    for page in website.get_pages().unwrap().iter() {
        if let Some(bytes) = page.get_bytes() {
            let download_file = encode_url(page.get_url());

            let download_file = if download_file.is_empty() {
                "index"
            } else {
                &download_file
            };

            let download_file = format!("{}/{}", download_path, download_file);

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&download_file)
                .expect("Unable to open file");

            file.write_all(bytes).unwrap_or_default();

            log("downloaded", download_file)

        } else {
            log("empty", page.get_url())
        }
    }
}
