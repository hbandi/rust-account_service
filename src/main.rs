mod http;
use http::server;
use env_logger::Env;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("Starting HTTP server at http://localhost:8080");
    
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI environment variable not set");
    println!("MONGO_URI value: {}", mongo_uri);
    
    server::start_server().await
}