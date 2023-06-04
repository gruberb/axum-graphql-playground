use crate::models::{MutationRoot, QueryRoot, ServiceSchema};
use crate::routes::{graphql_handler, graphql_playground, health};
use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use rocksdb::DB;
use std::sync::Arc;

mod models;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = DB::open_default("./db").unwrap();

    let db = Arc::new(db);

    let schema: ServiceSchema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db.clone())
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route("/health", get(health))
        .layer(Extension(schema));

    tracing::info!("Server listening at 0.0.0.0:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
