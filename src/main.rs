
mod data_store;
mod models;
mod server;
mod service;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginDto{
    user_name: String,
    password: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSidninDto{
    first_name: String,
    last_name: String,
    phone_number: String,

    user_name: String,
    password: String,
    password_confirm: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JWToken{
    access_token: String
}
impl JWToken{
    pub fn new(token: String) -> Self{
        Self { access_token: token }
    }
}
//Json(body): Json<UserLoginDto>
async fn user_login_handler(Json(body): Json<UserLoginDto>) -> impl IntoResponse{
    (StatusCode::OK, Json(JWToken::new(format!("{}:{}", body.user_name, body.password))))
}
async fn user_signin_handler(Json(body): Json<UserSidninDto>) -> impl IntoResponse{
    (StatusCode::OK, Json(JWToken::new(format!("{}:{}", body.user_name, body.password))))
}
async fn run_server(router: Router){
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve::serve(listener, router).await.unwrap();    
}
fn server_route() -> Router{
    let router = Router::new()
        .route("/", get(|| async { "Root Router" }))
        .route("/user/login", post(user_login_handler))
        .route("/user/signin", post(user_login_handler))
        .route("/user/me", get("Get User self data router"))
    ;
    router
}
#[tokio::main]
async fn main() {
    let router = server_route(); 
    run_server(router).await;
    println!("Hello, world!");
}
