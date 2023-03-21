#[macro_use]
extern crate log;
use core::types::Item;
use core::vinted;
use core::VERSION;
use rand::seq::SliceRandom;
use reqwest::Proxy;
use std::collections::HashMap;
use std::time::Duration;
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    info!("Version: {}", VERSION);

    run().await
}

static TWO_SECONDS: time::Duration = time::Duration::from_secs(2);

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = vinted::Client::new();

    let mut saved_items: HashMap<i64, Item> = HashMap::new();
    let mut proxy_list = vec![""];

    loop {
        // choose a random proxy from list
        let proxy_addr = match proxy_list.choose(&mut rand::thread_rng()) {
            Some(proxy) => proxy,
            None => {
                return Err("No proxies available".into());
            }
        };

        let proxy = Proxy::custom(move |_url| Some(reqwest::Url::parse(proxy_addr).unwrap()));

        let resp = match client.fetch_items(proxy).await {
            Ok(resp) => resp,
            Err(error) => {
                if error.is::<reqwest::Error>() {
                    let err = error.downcast::<reqwest::Error>()?;

                    if err.is_connect() || err.is_timeout() {
                        error!("Proxy error: {}", err);
                        continue;
                    }

                    match err.status() {
                        Some(status) => match status {
                            reqwest::StatusCode::UNAUTHORIZED => {
                                client.fetch_csrf_token().await?;
                            }
                            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                                error!("Too many requests. Sleeping for 10 seconds...");
                                thread::sleep(Duration::from_secs(10));
                            }
                            _ => {}
                        },
                        None => {}
                    }
                }

                continue;
            }
        };

        info!("Fetched {} items", resp.items.len());

        println!("{}", serde_json::to_string_pretty(&resp.items).unwrap());

        for item in resp.items {
            if !saved_items.contains_key(&item.id) {
                info!(
                    "New item: {} {}{} {}",
                    item.title, item.price, item.currency, item.brand_title
                );
                saved_items.insert(item.id.clone(), item);
            }
        }

        thread::sleep(TWO_SECONDS);
    }

    Ok(())
}
