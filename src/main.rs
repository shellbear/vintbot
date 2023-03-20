#[macro_use]
extern crate log;
use core::types::Item;
use core::vinted;
use core::VERSION;
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

static ONE_SECOND: time::Duration = time::Duration::from_secs(1);

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = vinted::Client::new()?;

    let mut saved_items: HashMap<i64, Item> = HashMap::new();

    loop {
        match client.fetch_items().await {
            Ok(items) => {
                info!("Fetched {} items", items.items.len());

                for item in items.items {
                    if !saved_items.contains_key(&item.id) {
                        info!("New item: {}", item.title);
                        saved_items.insert(item.id.clone(), item);
                    }
                }
            }
            Err(e) => {
                if e.is::<reqwest::Error>() {
                    let err = e.downcast::<reqwest::Error>()?;
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
            }
        }

        thread::sleep(ONE_SECOND);
    }

    Ok(())
}
