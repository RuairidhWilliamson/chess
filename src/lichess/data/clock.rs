use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clock {
    pub initial: u128,
    pub increment: u128,
}