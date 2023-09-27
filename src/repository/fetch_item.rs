use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{Client, model::AttributeValue};

pub async fn fetch_item() -> Result<impl Responder> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME").unwrap();

    let resp = client.scan().table_name(table_name).send().await.unwrap();

    let mut users: Vec<User> = Vec::new();

    for item in resp.items.unwrap_or_default() {
        if let (Some(AttributeValue::N(id)), Some(AttributeValue::S(name))) =
            (item.get("Id"), item.get("Name"))
        {
            users.push(User::new(id, name))
        }
    }

}