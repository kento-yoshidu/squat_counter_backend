use crate::repository::add_item;

use actix_web::{
    post,
    web,
    Responder,
    Result
};

#[post("/add")]
pub async fn add() -> Result<impl Responder> {
    let _resp = add_item::add().await;

    Ok(web::Json( "status" ))
}
