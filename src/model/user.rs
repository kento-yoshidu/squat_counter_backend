use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    id: String,
    name: String,
}

impl User {
    pub fn new(id: &String, name: &String) -> User {
        return User {
            id: id.to_string(),
            name: name.to_string(),
        };
    }
}
