/*
use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{
    Client,
    model::{
        ReturnValue,
        AttributeValue,
    },
    output,
    SdkError
};

pub async fn update_count() -> Result<output::UpdateItemOutput, SdkError<aws_sdk_dynamodb::error::ScanError>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    let resp = client
        .update_item()
        .table_name(table_name)
        .key(
            "date",
            AttributeValue::S("2023-01-01".to_string()),
        )
        .update_expression("set #key_name = :value".to_string())
        .expression_attribute_names(
            "#key_name".to_string(),
            "count".to_string())
        .expression_attribute_values(
            ":value",
            AttributeValue::S("111222k1".to_string()),
        )
        .return_values(ReturnValue::AllNew)
        .send()
        .await
        .unwrap();

    Ok(resp)
}
*/
