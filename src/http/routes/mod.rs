pub mod health;
pub mod user;

use actix_web::web;
use log::info;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");

    cfg
        .route("/health", web::get().to(health::health_check))
        .service(
            web::scope("/user")
                .route("", web::post().to(user::upsert_user))
                .route("/{id}", web::get().to(user::get_user)),
        );

    info!("Routes configured successfully");
} 