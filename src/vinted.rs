use log::debug;
use once_cell::sync::Lazy;
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::Deserialize;

use crate::types::{Item, Pagination};

const VINTED_BASE_URL: &str = "https://www.vinted.fr";

const CRSF_META_START_TAG: &str = "<meta name=\"csrf-token\" content=\"";
const CRSF_META_END_TAG: &str = "\" />";

static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    // Generate the default headers
    let mut default_headers = HeaderMap::new();

    default_headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3615.0 Safari/537.36"),
    );
    default_headers.insert(
        header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    default_headers.insert(
        header::ACCEPT_LANGUAGE,
        HeaderValue::from_static("en,*;q=0.1"),
    );

    default_headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    default_headers.insert(header::PRAGMA, HeaderValue::from_static("no-cache"));

    reqwest::ClientBuilder::new()
        .gzip(true)
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3615.0 Safari/537.36")
        .default_headers(default_headers)
        .build()
        .unwrap()
});

async fn fetch_vinted_csrf_token() -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}/cookie-policy", VINTED_BASE_URL);
    let resp = CLIENT
        .get(&url)
        .header(
            header::ACCEPT,
            "text/html,application/xhtml+xml,application/xmlq=0.9,image/webp,*/*;q=0.8",
        )
        .send()
        .await?;
    let body = resp.text().await?;

    // Find the starting index of the start pattern
    let start_index = match body.find(CRSF_META_START_TAG) {
        Some(index) => index + CRSF_META_START_TAG.len(),
        None => return Err("Could not find CSRF token".into()),
    };

    // Find the ending index of the end pattern
    let end_index = match body[start_index..].find(CRSF_META_END_TAG) {
        Some(index) => start_index + index,
        None => return Err("Could not find CSRF token".into()),
    };

    // Extract the substring between the patterns
    Ok(body[start_index..end_index].to_string())
}

#[derive(Deserialize)]
struct ItemsResponse {
    items: Vec<Item>,
    pagination: Pagination,
}

pub async fn fetch_items() -> Result<(), Box<dyn std::error::Error>> {
    let csrf_token = fetch_vinted_csrf_token().await?;

    debug!("CSRF token: {}", csrf_token);

    let url = format!("{}/api/v2/catalog/items", VINTED_BASE_URL);
    let resp = CLIENT
        .get(&url)
        .header(header::ACCEPT, "application/json")
        .query(&[("page", "1"), ("per_page", "1")])
        .send()
        .await?;

    let response = resp.json::<ItemsResponse>().await?;

    println!("{:?}", response.pagination);

    Ok(())
}
