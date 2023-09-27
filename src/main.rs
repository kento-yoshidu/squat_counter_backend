mod repo;

use actix_web::{App, HttpServer};

mod service;
use service::{fetch, add};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fetch)
            .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
