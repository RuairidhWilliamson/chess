use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeControl {
    pub r#type: String,
}