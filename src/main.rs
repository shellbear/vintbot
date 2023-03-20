#[macro_use]
extern crate log;

use core::vinted;
use core::VERSION;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Version: {}", VERSION);

    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = vinted::Client::new()?;

    let items = client.fetch_items().await?;
    info!("Items: {:?}", items);

    Ok(())
}
