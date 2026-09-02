use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub user_id: String,
    pub age: u8,
    pub document_status: String,
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub status: String,
    pub message: String,
}
