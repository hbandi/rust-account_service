use actix_web::{App, HttpServer};
use crate::http::routes::configure_routes;

pub async fn start_server() -> std::io::Result<()> {
    println!("Server is starting at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
