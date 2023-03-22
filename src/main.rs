#[macro_use]
extern crate log;
use clap::Parser;
use core::db::Categories;
use core::db::ItemsFilters;
use core::db::Sizes;
use core::errors::ProxyError;
use core::types::Item;
use core::vinted;
use core::VERSION;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use std::{thread, time};

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Path of the proxies file. Each line should contain a proxy in the format `[http|s]:ip:port`.
    /// > proxybroker serve --host 127.0.0.1 --port 8888 --types HTTPS
    /// See more: https://github.com/bluet/proxybroker2
    #[arg(short, long)]
    proxies_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let args = Args::parse();

    info!("Version: {}", VERSION);

    run(&args).await
}

fn populate_cache(saved_items: &mut HashMap<i64, Item>, items: &Vec<Item>) {
    let is_initial = saved_items.is_empty();

    for item in items {
        if !saved_items.contains_key(&item.id) {
            if !is_initial {
                info!(
                    "New item: {} {}{} {}",
                    item.title, item.price, item.currency, item.brand_title
                );
            }

            saved_items.insert(item.id, item.clone());
        }
    }

    info!("New cache size: {}", saved_items.len());
}

fn clear_cache(saved_items: &mut HashMap<i64, Item>, items: &Vec<Item>) {
    let ids = items.iter().map(|item| item.id).collect::<Vec<i64>>();

    let previous_len = saved_items.len();
    saved_items.retain(|id, _| ids.contains(id));
    let new_len = saved_items.len();

    if previous_len != new_len {
        info!("Clear cache size: {} -> {}", previous_len, new_len);
    }
}

async fn fetch_items_and_notify(
    client: &mut vinted::Client,
    saved_items: &mut HashMap<i64, Item>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filters = ItemsFilters {
        sizes: Some(vec![Sizes::XS as i32, Sizes::S as i32]),
        categories: Some(vec![Categories::Clothes as i32]),
        search_text: Some("sandro".to_string()),
        ..Default::default()
    };

    let resp = match client.fetch_items(filters).await {
        Ok(resp) => resp,
        Err(error) => {
            error!("Error while fetching items: {}", error);

            if error.is::<reqwest::Error>() {
                let err = error.downcast::<reqwest::Error>()?;

                if err.is_connect() || err.is_timeout() {
                    error!("Proxy error or timeout: {}", err);
                    return Err(ProxyError.into());
                }

                match err.status() {
                    Some(status) => match status {
                        reqwest::StatusCode::UNAUTHORIZED => {
                            error!("Unauthorized. Fetching new CSRF token...");
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

            return Ok(());
        }
    };

    info!("Fetched {} items", resp.items.len());

    populate_cache(saved_items, &resp.items);
    clear_cache(saved_items, &resp.items);

    Ok(())
}

static TWO_SECONDS: time::Duration = time::Duration::from_secs(5);

fn remove_proxy(proxies: &mut Vec<String>, proxy: &String) {
    info!("Removing proxy: {}", proxy);
    proxies.retain(|p| p != proxy);
}

fn read_proxies_from_file<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut proxies: Vec<String> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().into_iter() {
        proxies.push(line.unwrap());
    }

    info!("Loaded {} proxies", proxies.len());

    proxies
}

async fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut saved_items: HashMap<i64, Item> = HashMap::new();
    let mut client = vinted::Client::new()?;

    //info!("Reading proxies from file: {}", args.proxies_file);
    //let proxies = read_proxies_from_file(&args.proxies_file);

    loop {
        // let proxy_addr = proxies.choose(&mut rand::thread_rng());

        // if let None = proxy_addr {
        //     error!("No proxies left. Exiting...");
        //     break;
        // }

        // info!("Using proxy: {}", proxy_addr);

        // let proxy = Proxy::all(proxy_addr.clone()).unwrap();

        match fetch_items_and_notify(&mut client, &mut saved_items).await {
            Ok(_) => {}
            Err(error) => {
                if error.is::<ProxyError>() {
                    // remove_proxy(&mut proxies.clone(), proxy_addr);
                    error!("Proxy error. Removing proxy and continuing...");
                    continue;
                } else {
                    error!("Error: {}", error);
                }
            }
        }

        thread::sleep(TWO_SECONDS);
    }
}
