use std::sync::Arc;

use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::auth::VerifiedToken;

/// The application persistence layer interface.
pub struct State {}

/// The GraphQL context used when queries are being evaluated.
pub struct Context {
    pub state: Arc<Mutex<State>>,
    pub token: VerifiedToken,
}

#[derive(juniper::GraphQLObject)]
struct User {
    pub id: Uuid,
    pub username: String,
}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn users() -> Vec<String> {
        vec![]
    }

    async fn me(ctx: &Context) -> User {
        User {
            id: Uuid::new_v4(),
            username: "kaylen".to_string(),
        }
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
