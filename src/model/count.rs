use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Count {
    pub id: String,
    pub date: String,
    pub count: String,
    pub user_name: String,
}

impl Count {
    pub fn new(id: &String, date: &String, count: &String, user_name: &String) -> Count {
        return Count {
            id: id.to_string(),
            date: date.to_string(),
            count: count.to_string(),
            user_name: user_name.to_string(),
        };
    }
}
