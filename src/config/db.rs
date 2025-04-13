use mongodb::{options::ClientOptions, Client, Database};
use once_cell::sync::OnceCell;
use std::sync::Arc;

static DB_INSTANCE: OnceCell<Arc<Database>> = OnceCell::new();

pub async fn init_db() {
    let client_uri = "mongodb://localhost:27017"; // cámbialo si hace falta
    let client_options = ClientOptions::parse(client_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database("rust");
    DB_INSTANCE.set(Arc::new(db)).unwrap();
}

pub fn get_db() -> Arc<Database> {
    DB_INSTANCE.get().expect("Database not initialized").clone()
}
