use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(id: &String, name: &String) -> User {
        return User {
            id: id.to_string(),
            name: name.to_string(),
        };
    }
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserRequest {
    pub name: String
}
