use serde::{Serialize, Deserialize};
use crate::engine::game::Game;
use super::game_full::GameFull;
use super::super::API;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub r#type: String,
    pub moves: String,
    pub wtime: u128,
    pub btime: u128,
    pub winc: u128,
    pub binc: u128,
    pub wdraw: bool,
    pub bdraw: bool,
}

impl GameState {
    pub fn to_engine_game(&self, api: &API, full_game: &GameFull) -> Game {
        let mut game = full_game.to_engine_game(api);
        game.moves = self.moves.clone();
        game
    }
}