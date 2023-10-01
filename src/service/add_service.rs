use crate::{
    repository::add_repository,
    model::count::Count,
    model::user::User
};

use actix_web::{
    post,
    web,
    Responder,
    Result, HttpResponse
};

#[post("/add/user")]
pub async fn add_user(req: web::Json<User>) -> Result<impl Responder> {
    let name = &req.name;

    let _resp = add_repository::add_user(name).await;

    Ok(HttpResponse::Ok().json("{\"message\":\"Hello world again!\"}"))
}

#[post("/add/count")]
pub async fn add_count(req: web::Json<Count>) -> Result<impl Responder> {
    let date = &req.date;
    let count = &req.count;
    let user_name = &req.user_name;

    let _resp = add_repository::add_count(date, count, user_name).await;

    Ok(web::Json("foo"))
}
