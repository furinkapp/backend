use std::{error::Error, str::FromStr, sync::Arc};

use furink_proto::users::{
    profile_service_client::ProfileServiceClient, user_service_client::UserServiceClient,
};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};
use uuid::Uuid;

use crate::auth::VerifiedToken;

/// The application persistence layer interface.
pub struct State {
    users: UserServiceClient<Channel>,
    profiles: ProfileServiceClient<Channel>,
}

impl State {
    pub async fn new<S: AsRef<str>>(url: S) -> Result<Self, Box<dyn Error>> {
        let channel = Endpoint::from_str(url.as_ref())?.connect().await?;
        let users = UserServiceClient::new(channel.clone());
        let profiles = ProfileServiceClient::new(channel);
        Ok(State { users, profiles })
    }
}

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
        todo!()
    }

    async fn user(ctx: &Context, id: Uuid) -> User {
        todo!("support for fetching a user by their id")
    }

    async fn post(ctx: &Context, id: Uuid) -> String {
        todo!("support for fetching a post by its id")
    }

    async fn posts(ctx: &Context) -> Vec<String> {
        todo!("support for fetching a paginated list of posts")
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
