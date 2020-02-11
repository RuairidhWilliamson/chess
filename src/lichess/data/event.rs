use serde::{Serialize, Deserialize};
use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub r#type: String,
    pub challenge: Option<challenge::Challenge>,
    pub game: Option<game_start::GameStart>,
}