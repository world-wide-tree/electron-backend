use std::sync::Arc;

use surrealdb::{engine::remote::ws::{Client, Ws}, opt::auth::Root, Surreal};
use tokio::sync::OnceCell;

static SURREAL_CLIENT: OnceCell<Arc<Surreal<Client>>> = OnceCell::const_new();

pub fn db_pool() -> &'static Arc<Surreal<Client>> {
    SURREAL_CLIENT.get().expect("DbManager Not initialised!")   
}

pub async fn init_db(){
    let client = Surreal::new::<Ws>("127.0.0.1:8080").await.unwrap();
    client.signin(Root{
        username: "root",
        password: "root"
    })
    .await
    .unwrap()
    ;
    
    client.use_ns("Ziyrak").use_db("Ziyrak").await.unwrap();
    
    SURREAL_CLIENT.set(Arc::new(client)).unwrap()
}