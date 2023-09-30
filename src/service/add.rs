use crate::{repository::add_item, model::count::Count};

use actix_web::{
    post,
    web,
    Responder,
    Result
};

#[post("/add")]
pub async fn add_count(req: web::Json<Count>) -> Result<impl Responder> {
    let date = &req.date;
    let count = &req.count;
    let user_name = &req.user_name;

    let _resp = add_item::add_count(date, count, user_name).await;

    Ok(web::Json( "status" ))
}
