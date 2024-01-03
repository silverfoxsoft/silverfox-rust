use axum::http::Method;
use tower_http::{cors::{Any, CorsLayer}, cors, services::ServeDir};
use std::env;
use axum::handler::Handler;
use sea_orm::{Database};
use silverfox_rust::{api};
use silverfox_rust::common;
use common::app_state::AppState;
#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let api_router = api::api::router();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let app = api_router
        .layer(cors).with_state(AppState { conn });
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    tracing::info!("service started at : {:?}", &server_url);
    axum::serve(listener, app).await?;
    Ok(())
}


pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}