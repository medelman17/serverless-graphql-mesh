use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID,
};

use tracing_subscriber;

#[derive(SimpleObject)]
pub struct Review {
    pub body: String,
    pub author: Customer,
    pub product: Product,
}

pub struct Customer {
    pub id: ID,
}

#[Object(extends)]
impl Customer {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn reviews<'a>(&self, ctx: &'a Context<'_>) -> Vec<&'a Review> {
        let reviews = ctx.data_unchecked::<Vec<Review>>();
        reviews
            .iter()
            .filter(|review| review.author.id == self.id)
            .collect()
    }
}

pub struct Product {
    pub upc: String,
}

#[Object(extends)]
impl Product {
    #[graphql(external)]
    async fn upc(&self) -> &String {
        &self.upc
    }

    async fn reviews<'a>(&self, ctx: &'a Context<'_>) -> Vec<&'a Review> {
        let reviews = ctx.data_unchecked::<Vec<Review>>();
        reviews
            .iter()
            .filter(|review| review.product.upc == self.upc)
            .collect()
    }
}

pub struct Query;

#[Object]
impl Query {
    #[graphql(entity)]
    async fn get_customer_by_id(&self, id: ID) -> Customer {
        Customer { id }
    }

    #[graphql(entity)]
    async fn get_product_by_upc(&self, upc: String) -> Product {
        Product { upc }
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
