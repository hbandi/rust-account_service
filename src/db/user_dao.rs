use crate::models::user::User;
use crate::db::db_configs::connect;
use mongodb::error::Error;
use mongodb::bson::doc;
use log::{info, error};

pub async fn upsert_user(user: User) -> Result<String, Error> {
    info!("Attempting to insert user: {}", user.name);
    let db = connect().await.map_err(|e| {
        error!("Database connection failed: {}", e);
        e
    })?;
    let user_collection = db.collection::<User>("users");

    match user_collection.insert_one(user.clone(), None).await {
        Ok(_) => {
            info!("Successfully created user: {}", user.name);
            Ok(format!("user created with name {}", user.name))
        },
        Err(e) => {
            error!("Failed to create user {}: {}", user.name, e);
            Err(e)
        }
    }
}

pub async fn get_user_by_name(name: &str) -> Result<User, Error> {
    info!("Attempting to find user with name: {}", name);
    let db = connect().await.map_err(|e| {
        error!("Database connection failed: {}", e);
        e
    })?;
    let user_collection = db.collection::<User>("users");
    
    let filter = doc! { "name": name };
    match user_collection.find_one(filter, None).await? {
        Some(user) => {
            info!("Found user: {}", user.name);
            Ok(user)
        },
        None => {
            error!("User not found: {}", name);
            Err(Error::custom("User not found"))
        }
    }
}