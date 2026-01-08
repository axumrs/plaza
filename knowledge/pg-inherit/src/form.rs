use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserForm {
    pub username: String,
    pub nickname: String,
    pub balance: i32,
}

#[derive(Deserialize)]
pub struct UpdateUserForm {
    pub nickname: String,
    pub balance: i32,
}
