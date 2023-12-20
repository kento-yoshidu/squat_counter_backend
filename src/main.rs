mod repository;
mod model;

use actix_web::{App, http::header, HttpServer};
use actix_cors::Cors;

mod service;
use service::{fetch_service, add_service, /*update_service*/};

use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(fetch_service::fetch_user)
            .service(fetch_service::fetch_count)
            // .service(fetch_service::fetch_today)
            // .service(add_service::add_user)
            .service(add_service::add_count)
            //.service(update_service::update_count)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
