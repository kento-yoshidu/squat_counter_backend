#[allow(unused)]
use actix_web::{
    get, post, web,
    web::{Data, Json, Path},
    Responder, HttpResponse
};

#[allow(unused)]
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow, Debug)]
struct User {
    id: i32,
    name: String,
}

#[get("/test")]
pub async fn test() -> impl Responder {
    let user = User { id: 1, name: String::from("test_user") };

    web::Json(user)
}
