mod repository;
mod model;

use actix_web::{App, HttpServer};

mod service;
use service::{fetch, add};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fetch::fetch_user)
            .service(fetch::fetch_count)
            .service(add::add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
