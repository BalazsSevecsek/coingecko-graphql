extern crate log;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use coingecko_graphql::{Query, SymbolCache};
use env_logger::{Builder, Target};
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::{env, process::exit};

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphiql").finish())
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

    let cache = SymbolCache::populate().await?;
    info!("Populate symbol cache");

    let schema: Schema<Query, EmptyMutation, EmptySubscription> =
        Schema::build(Query, EmptyMutation, EmptySubscription)
            .data(db_connection)
            .data(cache)
            .finish();
    info!("Graphql schema compiled");

    let app: Router = Router::new().route(
        "/graphiql",
        get(graphiql).post_service(GraphQL::new(schema)),
    );
    let server_future = Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            rx_shutdown.await.ok();
        });

    info!("Server started on http://localhost:{}", 8000);
    info!("GraphiQL path: http://localhost:8000/graphiql");

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
