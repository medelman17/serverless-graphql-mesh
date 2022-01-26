use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use tracing_subscriber;

#[derive(SimpleObject)]
pub struct Product {
    pub upc: String,
    pub name: String,
    pub price: i32,
}

pub struct Query;

#[Object(extends)]
impl Query {
    async fn list_products<'a>(&self, ctx: &'a Context<'_>) -> &'a Vec<Product> {
        ctx.data_unchecked::<Vec<Product>>()
    }

    #[graphql(entity)]
    async fn get_product_by_upc<'a>(
        &self,
        ctx: &'a Context<'_>,
        upc: String,
    ) -> Option<&'a Product> {
        let products = ctx.data_unchecked::<Vec<Product>>();
        products.iter().find(|product| product.upc == upc)
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
