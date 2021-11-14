#[macro_use]
extern crate juniper;

use log::{info, LevelFilter};

use crate::v1::run_api_v1;

mod v1;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    println!(
        "\nfurink-session v{}\nAuthors: {}\n",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );

    info!("mounting api v1...");
    run_api_v1().await;
}
