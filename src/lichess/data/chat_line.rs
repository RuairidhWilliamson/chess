use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatLine {
    pub r#type: String,
    pub username: String,
    pub text: String,
    pub room: String,
}