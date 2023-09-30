mod repository;
mod model;

use actix_web::{App, http::header, HttpServer};
use actix_cors::Cors;

mod service;
use service::{fetch, add};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .service(fetch::fetch_user)
            .service(fetch::fetch_count)
            .service(add::add_count)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
