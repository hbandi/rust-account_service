use actix_web::{web, HttpResponse, Responder};
use account_service::models::user::User;
use account_service::db::user_dao;
use log::{info, error, warn};

pub async fn upsert_user(user: web::Json<User>) -> impl Responder {
    info!("Received create user request for: {:?}", user);
    
    match user_dao::upsert_user(user.0).await {
        Ok(created_user) => {
            info!("Successfully processed user creation");
            HttpResponse::Ok().json(created_user)
        },
        Err(e) => {
            error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub async fn get_user(name: web::Path<String>) -> impl Responder {
    info!("Received get user request for name: {}", name);
    
    match user_dao::get_user_by_name(&name).await {
        Ok(user) => {
            info!("Successfully retrieved user: {:?}", user);
            HttpResponse::Ok().json(user)
        },
        Err(e) => {
            warn!("Failed to retrieve user {}: {}", name, e);
            HttpResponse::NotFound().body(e.to_string())
        }
    }
}