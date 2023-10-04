use dotenv::dotenv;
use std::env;

use aws_sdk_dynamodb::{
    Client, SdkError,
    model::{
        AttributeAction,
        ReturnValue,
        AttributeValue,
        AttributeValueUpdate,
    }
};

// pub async fn update_count() -> Result<aws_sdk_dynamodb::output::ScanOutput, SdkError<aws_sdk_dynamodb::error::ScanError>> {
pub async fn update_count() -> bool {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    dotenv().ok();

    let table_name = env::var("TABLE_NAME_COUNT").unwrap();

    let date = String::from("2023-10-02");

    let attr_val_up = AttributeValueUpdate::builder()
        .action(AttributeAction::Put)
        .value(AttributeValue::N("999".to_string()))
        .build();

    /*
    let resp = client
        .update_item()
        .table_name(table_name)
        .key("id".to_string(), "1".to_string())
        .attribute_updates("count", attr_val_up)
        .return_values(ReturnValue::AllNew)
        .send()
        .await
        .unwrap();
    */

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
            AttributeValue::S("1111".to_string()),
        )
        .send()
        .await
        .unwrap();

    println!("resp = {:?}", resp);

    true
}
