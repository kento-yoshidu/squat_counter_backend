/*
use crate::{
    repository::add_repository,
    model::count::Count,
    model::{user::{User, UserRequest}, count::CountRequest}
};

use actix_web::{
    post,
    web,
    Responder,
    Result, HttpResponse
};

use chrono::{Local, Datelike};
use uuid::Uuid;

#[post("/add/user")]
pub async fn add_user(req: web::Json<UserRequest>) -> Result<impl Responder> {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    let name = &req.name;

    let user = User {
        id: uuid,
        name: name.to_string()
    };

    let _resp = add_repository::add_user(&user).await;

    Ok(HttpResponse::Ok().json("{\"message\":\"Hello world again!\"}"))
}

#[post("/add/count")]
pub async fn add_count(req: web::Json<CountRequest>) -> Result<impl Responder> {
    println!("req = {:?}", req);

    let current_date = Local::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let date = format!("{}-{:02}-{:02}", year, month, day);

    let count = Count {
        date: date.to_string(),
        count: req.count.to_string(),
        user_name: req.user_name.to_string()
    };

    let _resp = add_repository::add_count(&count).await;

    Ok(web::Json("foo"))
}
*/
