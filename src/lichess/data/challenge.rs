use serde::{Serialize, Deserialize};
use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub status: String,
    pub challenger: user::User,
    pub dest_user: user::User,
    pub variant: variant::Variant,
    pub rated: bool,
    pub color: String,
    pub time_control: time_control::TimeControl,
}