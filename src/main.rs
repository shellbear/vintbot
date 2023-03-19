use core::vinted;
use core::VERSION;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Version: {}", VERSION);

    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    vinted::fetch_items().await?;

    Ok(())
}
