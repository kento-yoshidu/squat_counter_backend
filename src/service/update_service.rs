use crate::repository::update_repository;

use actix_web::{
    get,
    Responder,Result,
    web
};

#[get("/update/count")]
pub async fn update_count() -> Result<impl Responder> {
    let res = update_repository::update_count().await;

    println!("update_count = {:?}", res);

    Ok(web::Json("hoge"))
}
