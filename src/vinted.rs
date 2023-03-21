use std::time::Duration;

use log::{error, info};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::Proxy;

use crate::types::{Item, PaginatedResponse};

const VINTED_BASE_URL: &str = "https://www.vinted.fr";

const CRSF_META_START_TAG: &str = "<meta name=\"csrf-token\" content=\"";
const CRSF_META_END_TAG: &str = "\" />";

pub struct Client {
    client: reqwest::Client,
    csrf_token: String,
}

fn create_client(proxy: Proxy) -> reqwest::Result<reqwest::Client> {
    let default_headers = HeaderMap::from_iter([
        (
            header::ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        ),
        (
            header::ACCEPT_LANGUAGE,
            HeaderValue::from_static("en,*;q=0.1"),
        ),
        (header::CACHE_CONTROL, HeaderValue::from_static("no-cache")),
        (header::PRAGMA, HeaderValue::from_static("no-cache")),
    ]);

    reqwest::ClientBuilder::new()
        .gzip(true)
        .proxy(proxy)
        .cookie_store(true)
        .timeout(Duration::from_secs(3))
        .user_agent("Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3615.0 Safari/537.36")
        .default_headers(default_headers)
        .build()
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            csrf_token: String::new(),
        }
    }

    pub async fn fetch_csrf_token(&mut self) -> Result<&str, Box<dyn std::error::Error>> {
        info!("Fetching CSRF token...");

        let url = format!("{}/cookie-policy", VINTED_BASE_URL);
        let res = self
            .client
            .get(&url)
            .header(
                header::ACCEPT,
                "text/html,application/xhtml+xml,application/xmlq=0.9,image/webp,*/*;q=0.8",
            )
            .send()
            .await?;

        let res = match res.error_for_status() {
            Ok(res) => res,
            Err(e) => {
                error!("Error fetching CSRF token: {}", e);
                return Err(e.into());
            }
        };

        let body = res.text().await?;

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

        // Extract the csrf token
        self.csrf_token = body[start_index..end_index].to_string();

        info!("Fetched CSRF token");

        Ok(&self.csrf_token)
    }

    pub async fn fetch_items(
        &mut self,
        proxy: Proxy,
    ) -> Result<PaginatedResponse<Item>, Box<dyn std::error::Error>> {
        self.client = create_client(proxy)?;

        if self.csrf_token.is_empty() {
            self.fetch_csrf_token().await?;
        }

        info!("Fetching items...");

        let url = format!("{}/api/v2/catalog/items", VINTED_BASE_URL);
        let res = self
            .client
            .get(&url)
            .header("X-CSRF-Token", &self.csrf_token)
            .header(header::ACCEPT, "application/json")
            .query(&[
                ("page", "1"),
                ("per_page", "10"),
                ("order", "newest_first"),
                ("search_text", "sandro"),
            ])
            .send()
            .await?;

        let res = match res.error_for_status() {
            Ok(res) => res,
            Err(err) => {
                error!("Error fetching items: {}", err);
                return Err(err.into());
            }
        };

        let response = res.json::<PaginatedResponse<Item>>().await?;

        info!("Fetched {} items", response.items.len());
        Ok(response)
    }
}
