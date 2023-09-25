mod repo;

#[allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod service;
use service::{fetch, add};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fetch)
            .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/*

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME").unwrap();

    let resp = client.scan().table_name(table_name).send().await;

    match resp {
        Ok(scan_output) => {
            if let Some(items) = scan_output.items {
                for item in items {
                    if let Some(attr_val) = item.get("year") {
                        if let Ok(year_val) = attr_val.as_n() {
                            if let Ok(year) = year_val.parse::<u32>() {
                                println!("{}", year);
                            }
                        }
                    }

                    if let Some(attr_val) = item.get("roger") {
                        if let Ok(roger_val) = attr_val.as_n() {
                            if let Ok(roger) = roger_val.parse::<u32>() {
                                println!("{}", roger);
                            }
                        }
                    }

                    if let Some(attr_val) = item.get("other") {
                        if let Ok(other_val) = attr_val.as_n() {
                            if let Ok(other) = other_val.parse::<u32>() {
                                println!("{}", other);
                            }
                        }
                    }
                    /*
                        if let Some(attr_val) = item.get("id") {
                            if let Ok(id_val) = attr_val.as_n() {
                                if let Ok(id) = id_val.parse::<u32>() {
                                    println!("{}", id);
                                }
                            }
                        }
                        if let Some(attr_val) = item.get("year") {
                            if let Ok(name) = attr_val.as_s() {
                                println!("{}", name.to_string());
                            }
                        }
                    */
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
*/

