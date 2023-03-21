#[macro_use]
extern crate log;
use clap::Parser;
use core::db::ItemsFilters;
use core::errors::ProxyError;
use core::types::Item;
use core::vinted;
use core::VERSION;
use rand::seq::SliceRandom;
use reqwest::Proxy;
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
    /// Path of the proxies file. Each line should contain a proxy in the format `ip:port`.
    #[arg(short, long)]
    proxies_file: String,
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
}

fn clear_cache(saved_items: &mut HashMap<i64, Item>, items: &Vec<Item>) {
    let ids = items.iter().map(|item| item.id).collect::<Vec<i64>>();

    saved_items.retain(|id, _| ids.contains(id));
}

async fn fetch_items_and_notify(
    saved_items: &mut HashMap<i64, Item>,
    proxy: Proxy,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = vinted::Client::new();
    let filters = ItemsFilters::default();

    let resp = match client.fetch_items(proxy, filters).await {
        Ok(resp) => resp,
        Err(error) => {
            if error.is::<reqwest::Error>() {
                let err = error.downcast::<reqwest::Error>()?;

                if err.is_connect() || err.is_timeout() {
                    return Err(ProxyError.into());
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

            return Ok(());
        }
    };

    info!("Fetched {} items", resp.items.len());

    clear_cache(saved_items, &resp.items);
    populate_cache(saved_items, &resp.items);

    Ok(())
}

static TWO_SECONDS: time::Duration = time::Duration::from_secs(5);

fn remove_proxy(proxies: &mut Vec<String>, proxy: &String) {
    proxies.retain(|p| p != proxy);
}

fn read_proxies_from_file(path: &Path) -> Vec<String> {
    let mut proxies: Vec<String> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().into_iter() {
        proxies.push(line.unwrap());
    }

    proxies
}

async fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut saved_items: HashMap<i64, Item> = HashMap::new();
    let proxies = read_proxies_from_file(Path::new(&args.proxies_file));

    loop {
        let proxy_addr = match proxies.choose(&mut rand::thread_rng()) {
            Some(proxy) => proxy.clone(),
            None => {
                panic!("No proxies available");
            }
        };

        let current_proxy = proxy_addr.clone();
        let proxy = Proxy::custom(move |_url| Some(reqwest::Url::parse(&proxy_addr).unwrap()));

        match fetch_items_and_notify(&mut saved_items, proxy).await {
            Ok(_) => {}
            Err(error) => {
                if error.is::<ProxyError>() {
                    remove_proxy(&mut proxies.clone(), &current_proxy);
                } else {
                    error!("Error: {}", error);
                }
            }
        }

        thread::sleep(TWO_SECONDS);
    }
}
