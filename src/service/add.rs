use crate::repo::add_item;

use actix_web::{
  post,
  web,
  Responder,
  Result
};

#[post("/add")]
pub async fn add() -> Result<impl Responder> {
    let resp = add_item::add().await;

    Ok(web::Json( "status" ))
}
