use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: i32,
}

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub id: i32,
}
