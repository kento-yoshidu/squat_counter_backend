/*
use crate::model::user::User;
use crate::model::count::Count;

use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, types::AttributeValue};

pub async fn add_user(user: &User) -> Result<aws_sdk_dynamodb::output::PutItemOutput, aws_sdk_dynamodb::SdkError<aws_sdk_dynamodb::error::PutItemError>> {
    dotenv().ok();

    let table_name = env::var("TABLE_NAME_USER").unwrap();

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let id = AttributeValue::S(user.id.to_string());
    let name = AttributeValue::S(user.name.to_string());

    let request = client
        .put_item()
        .table_name(table_name)
        .item("id", id)
        .item("name", name);

    let resp = request.send().await;

    resp
}

pub async fn add_count(count: &Count) -> Result<aws_sdk_dynamodb::output::PutItemOutput, aws_sdk_dynamodb::SdkError<aws_sdk_dynamodb::error::PutItemError>> {
    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let username = &count.user_name;

    let date = AttributeValue::S(count.date.to_string());
    let count = AttributeValue::S(count.count.to_string());
    let username = AttributeValue::S(username.to_string());

    let request = client
        .put_item()
        .table_name(table_name)
        .item("date", date)
        .item("count", count)
        .item("user_name", username);

    let resp = request.send().await;

    resp
}
*/
