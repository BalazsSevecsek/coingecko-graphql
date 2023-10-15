use async_graphql::{
    http::GraphiQLSource, ComplexObject, EmptyMutation, EmptySubscription, Object, Schema,
    SimpleObject,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use log::info;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

struct Query;

#[Object]
impl Query {
    async fn get_my_object(&self) -> MyObj {
        MyObj { a: 1, b: 2 }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)] // NOTE: If you want the `ComplexObject` macro to take effect, this `complex` attribute is required.
struct MyObj {
    a: i32,
    b: i32,
}

#[ComplexObject]
impl MyObj {
    async fn c(&self) -> i32 {
        self.a + self.b
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenvy::dotenv().expect("Env file could not be loaded");

    info!("hello");

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        // .data(StarWars::new())
        .finish();

    let app: Router = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    println!("GraphiQL IDE: http://localhost:8000");

    // coingecko_graphql::get_list_of_accepted_tickers_and_ids().await?;
    coingecko_graphql::get_current_price("zombie-inu-2", "usd").await?;

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Server could not be started");

    return Ok(());
}
