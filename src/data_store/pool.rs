use std::sync::Arc;

use surrealdb::{engine::remote::ws::{Client, Ws}, Surreal};
use tokio::sync::OnceCell;

static SURREAL_CLIENT: OnceCell<Arc<Surreal<Client>>> = OnceCell::const_new();

pub fn db_pool() -> &'static Arc<Surreal<Client>> {
    SURREAL_CLIENT.get().expect("DbManager Not initialised!")   
}