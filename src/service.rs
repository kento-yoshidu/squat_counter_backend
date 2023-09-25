use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, model::{AttributeAction, AttributeValue}};

#[allow(unused)]
use actix_web::{
    get, post, web,
    web::{Data, Json, Path},
    Responder, HttpResponse,
    Result
};

#[allow(unused)]
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};

#[derive(Serialize, FromRow, Debug)]
struct User {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize)]
pub enum ApiErrorResponse {
    AuthenticationFailure,
    NeedsAuthentication,
}

#[derive(Debug, Serialize)]
pub struct ApiResponseBody {
    pub success: bool,
    pub errcode: u16,
    pub message: String,
}

#[get("/test")]
pub async fn test() -> impl Responder {
    let user = User { id: 1, name: String::from("test_user") };

    web::Json(user)
}

#[get("/fetch")]
pub async fn fetch() -> Result<impl Responder> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME").unwrap();

    let resp = client.scan().table_name(table_name).send().await;

    println!("response = {:?}", resp);

    let user = User { id: 1, name: String::from("hoge") };

    Ok(web::Json(user))
}

#[post("/add")]
pub async fn add() -> Result<impl Responder> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME").unwrap();

    let user_id = AttributeValue::N("3".to_string());
    let user_name = AttributeValue::S("gogogob".to_string());

    let request = client
        .put_item()
        .table_name(table_name)
        .item("Id", user_id)
        .item("Name", user_name);

    let resp = request.send().await;

    println!("resop = {:?}", resp);

    Ok(web::Json( "status" ))
}
