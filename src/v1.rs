use std::{collections::HashMap, sync::Arc};

use juniper::{EmptyMutation, EmptySubscription, RootNode};
use serde::Serialize;
use tokio::sync::RwLock;
use warp::{hyper::StatusCode, Filter};

struct Context(Arc<RwLock<HashMap<String, String>>>);

impl juniper::Context for Context {}

struct Query;

#[juniper::graphql_object(
	Context = Context
)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    async fn get_message(context: &Context, name: String) -> Option<String> {
        let Context(context) = context;
        match context.read().await.get(&name) {
            Some(message) => Some(message.clone()),
            None => None,
        }
    }

    async fn set_message(context: &Context, name: String, message: String) -> String {
        let Context(context) = context;
        context.write().await.insert(name, message.clone());
        message
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
}

#[derive(Serialize)]
struct ErrorMessage<'a> {
    msg: &'a str,
    code: u8,
}

/// Builds and returns the first version of the API.
pub async fn run_api_v1() {
    let database_ctx = Arc::new(RwLock::new(HashMap::new()));

    let state = warp::any().map(move || Context(database_ctx.clone()));
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    let log = warp::log("furink_backend");

    let root = warp::get()
        .and(warp::path("graphiql"))
        .and(juniper_warp::graphiql_filter("/graphql", None))
        .or(warp::path("graphql").and(graphql_filter))
        .or(warp::path::end().map(|| {
            warp::reply::with_status(
                warp::reply::json(&ErrorMessage {
                    code: 0,
                    msg: "Error 404: Not Found",
                }),
                StatusCode::NOT_FOUND,
            )
        }))
        .with(log);

    warp::serve(root).run(([127, 0, 0, 1], 3000)).await;
}
