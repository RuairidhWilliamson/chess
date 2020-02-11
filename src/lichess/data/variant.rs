use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Variant {
    pub key: String,
    pub name: String,
    pub short: String,
}