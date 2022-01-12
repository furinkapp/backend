use std::{env, error::Error, sync::Arc};

use dotenv::dotenv;
use log::info;
use tokio::sync::Mutex;
use warp::Filter;

use crate::{
    errors::handle_rejection,
    filters::access_token,
    graphql::{create_schema, Context, State},
};

mod auth;
mod errors;
mod filters;
mod graphql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("\nfurink-database v{}", env!("CARGO_PKG_VERSION"));
    println!("Authors: {}\n", env!("CARGO_PKG_AUTHORS"));
    // Initialize the logger.
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    // Load the environment variables from .env file.
    dotenv().ok();
    // Create the GraphQL schema.
    info!("Creating GraphQL context...");
    // Create the state and context filter.
    let state = State {};
    let state = Arc::new(Mutex::new(state));
    let context = warp::any().and(access_token()).map(move |token| Context {
        state: state.clone(),
        token,
    });
    // Create the GraphQL server.
    info!("Creating GraphQL server...");
    let graphql_filter = juniper_warp::make_graphql_filter(create_schema(), context.boxed());
    // Start the server.
    let log = warp::log("warp_server");
    info!("Starting server...");
    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(warp::path("graphql").and(graphql_filter))
            .recover(handle_rejection)
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await;

    Ok(())
}
