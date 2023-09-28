use dotenv::dotenv;
use std::env;

use crate::model::count::Count;

use aws_sdk_dynamodb::{Client, model::AttributeValue, SdkError};

pub async fn fetch_user() -> Result<aws_sdk_dynamodb::output::ScanOutput, SdkError<aws_sdk_dynamodb::error::ScanError>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME").unwrap();

    let resp = client
        .scan()
        .table_name(table_name)
        .send()
        .await
        .unwrap();

    Ok(resp)
}

pub async fn fetch_count() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    let resp = client.scan().table_name(table_name).send().await.unwrap();

    let mut counts: Vec<Count> = Vec::new();

    for item in resp.items.unwrap_or_default() {
        if let (Some(AttributeValue::S(id)), Some(AttributeValue::S(date)), Some(AttributeValue::S(count))) =
            (item.get("id"), item.get("date"), item.get("count"))
        {
            counts.push(Count::new(id, date, count))
        }
    }

    println!("counts = {:?}", counts);
}
