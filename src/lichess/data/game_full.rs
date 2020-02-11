use serde::{Serialize, Deserialize};
use super::*;
use crate::engine::game::Game;
use crate::chess::fen_parser;
use crate::chess::colour::Colour;
use super::super::API;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameFull {
    pub r#type: String,
    pub id: String,
    pub rated: bool,
    pub variant: variant::Variant,
    pub clock: Option<clock::Clock>,
    pub created_at: u128,
    pub white: user::User,
    pub black: user::User,
    pub initial_fen: String,
    pub state: game_state::GameState,
    pub speed: String,
}

impl GameFull {
    pub fn to_engine_game(&self, api: &API) -> Game {
        Game {
            game_id: self.id.clone(),
            initial_board: fen_parser::parse(&self.initial_fen).unwrap(),
            moves: self.state.moves.clone(),
            my_side: self.get_my_side(api),
        }
    }

    fn get_my_side(&self, api: &API) -> Colour {
        if self.white.id == api.config.lichess.my_id {
            Colour::White
        } else {
            if self.black.id != api.config.lichess.my_id {
                println!("UH OH: {} {}", self.white.id, self.black.id);
            }
            Colour::Black
        }
    }
}