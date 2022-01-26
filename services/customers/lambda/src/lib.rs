use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use tracing_subscriber;

#[derive(SimpleObject)]
pub struct Customer {
    id: ID,
    username: String,
}

pub struct Query;

#[Object(extends)]
impl Query {
    async fn me(&self) -> Customer {
        Customer {
            id: "1234".into(),
            username: "Me".to_string(),
        }
    }

    #[graphql(entity)]
    async fn get_customer_by_id<'a>(&self, _ctx: &'a Context<'_>, id: ID) -> Customer {
        let username = if id == "1234" {
            "Me".to_string()
        } else {
            format!("Customer {:?}", id)
        };
        Customer { id, username }
    }
}

pub type GraphQLServiceSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}
