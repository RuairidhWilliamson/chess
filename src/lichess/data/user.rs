use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    name: String,
    title: Option<String>,
    rating: u16,
    provisional: Option<bool>,
    patron: Option<bool>,
    online: Option<bool>,
    lag: Option<u16>,
}