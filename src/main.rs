
mod data_store;
mod models;
mod server;
mod service;

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use server::routers::api_router;
use service::app_state::AppState;
use tokio::net::TcpListener;
use data_store::pool::init_db;




async fn run_server(router: Router){
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve::serve(listener, router).await.unwrap();    
}

#[tokio::main]
async fn main() {
    init_db().await;
    let app_state = Arc::new(AppState::init());
    let router = api_router(app_state);

    run_server(router).await;
    println!("Hello, world!");
}
