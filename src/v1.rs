use std::collections::HashMap;

use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, GraphQLEnum, GraphQLObject, RootNode,
};
use serde::Serialize;
use warp::{hyper::StatusCode, Filter};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

struct Context {
    // Use your real database pool here.
    pool: HashMap<String, String>,
}

struct Query;

#[juniper::graphql_object(
	Context = Context
)]
impl Query {
    fn apiVersion() -> &'static str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // The executor is a special (optional) argument that allows accessing the context.
    fn human(&self, name: String) -> String {
        name
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

impl juniper::Context for Context {}

#[derive(Serialize)]
struct ErrorMessage<'a> {
    msg: &'a str,
    code: u8,
}

/// Builds and returns the first version of the API.
pub async fn run_api_v1() {
    let state = warp::any().map(move || Context {
        pool: HashMap::new(),
    });
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
