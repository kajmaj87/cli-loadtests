use hyper::{Client, Uri};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "lt - a minimalistic cli load testing app")]
#[clap(version = "1.0")]
#[clap(about = "App is intended for some quick checks done interactively or in scripted mode", long_about = None)]
struct Cli {
    #[clap(value_parser)]
    url: String,
    #[clap(short, long, default_value_t = 10, value_parser)]
    requests_per_second: u64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config_handle = Arc::new(Cli::parse());
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let client_handle = Arc::new(Client::new());

    loop {
        let client = client_handle.clone();
        let config = config_handle.clone();
        thread::sleep(Duration::from_micros(1_000_000/config.requests_per_second));
        tokio::spawn(async move {
            let uri = config.url.parse::<Uri>().unwrap();
            // Await the response...
            let resp = client.get(uri).await.unwrap();
            println!("Response: {:?}", resp);
        });
    }

    Ok(())
}
