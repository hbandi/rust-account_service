use mongodb::{Client, Database, options::ClientOptions};
use std::env;
use log::{info, error};

pub async fn connect() -> mongodb::error::Result<Database> {
    info!("Attempting to connect to MongoDB");
    info!("MONGO_URI value: {}", env::var("MONGO_URI").unwrap_or_default());
    
    let db_uri = env::var("MONGO_URI")
        .map_err(|_| {
            error!("MONGO_URI environment variable not set");
            mongodb::error::Error::custom("MONGO_URI not set")
        })?;
    let db_name = env::var("MONGO_DB")
        .map_err(|_| {
            error!("MONGO_DB environment variable not set");
            mongodb::error::Error::custom("MONGO_DB not set")
        })?;

    let options = ClientOptions::parse(&db_uri).await?;
    let client = Client::with_options(options)?;
    info!("Successfully connected to MongoDB");

    Ok(client.database(&db_name))
}
