extern crate log;

use async_graphql::{http::GraphiQLSource, EmptyMutation, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use coingecko_graphql::{DbConnection, Query, Subscription, SymbolCache};
use env_logger::{Builder, Target};
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::{env, process::exit};

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx_shutdown, rx_shutdown) = tokio::sync::oneshot::channel::<()>();
    dotenvy::dotenv().expect("Env file could not be loaded");
    let database_url: String =
        env::var("DATABASE_URL").expect("No database url is present in env file");
    info!("Env file loaded");

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
    info!("Logger initialized");

    let db_connection = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    info!("Database connected");

    sqlx::migrate!().run(&db_connection).await?;
    info!("Migrations ran down");

    let connection_wrapper = DbConnection::new(db_connection);

    let cache = SymbolCache::populate().await?;
    info!("Populate symbol cache");

    let schema: Schema<Query, EmptyMutation, Subscription> =
        Schema::build(Query, EmptyMutation, Subscription)
            .data(connection_wrapper)
            .data(cache)
            .finish();

    info!("Graphql schema compiled");

    let app: Router = Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema));

    let server_future = Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            rx_shutdown.await.ok();
        });

    info!("Server started on http://localhost:{}", 8000);

    tokio::select! {
        _=tokio::signal::ctrl_c()=>{
            tx_shutdown.send(()).ok();
        }
        res= server_future=>{
            match res {
                Ok(_)=>{},
                Err(e)=>{
                    eprintln!("Fatal:Server shut down due to: {}",e);
                    exit(1)
                }
            }
        }
    }

    return Ok(());
}
