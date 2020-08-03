extern crate serde;
extern crate reqwest;

use serde_json;
use std::io::prelude::*;
use std::convert::AsRef;
use std::fmt::Write;
use std::sync::mpsc::Sender;

use super::data;
use crate::engine::game::Game;
use data::game_full::GameFull;
use super::API;

impl API {
    pub fn run_game_stream(&self, game_id: String, tx: Sender<Game>) {
        let url = format!("bot/game/stream/{}", game_id);
        let response: reqwest::Response = self.auth_get(url).unwrap();
        let mut game_stream = GameStream {
            api: self.clone(),
            game_full: None,
            tx: tx,
        };
        let mut buffer = String::default();
        for b in response.bytes() {
            let c = match b {
                Ok(b) => b as char,
                Err(_) => break,
            };
            if c == '\n' {
                game_stream.parse_game(buffer.clone());
                buffer = String::default();
            } else {
                buffer.write_char(c).unwrap();
            }
        }
    }
}

struct GameStream {
    api: API,
    game_full: Option<GameFull>,
    tx: Sender<Game>,
}

impl GameStream {

    fn parse_game(&mut self, json: String) {
        if &json.len() <= &2 {
            return;
        }
        let game: data::game::Game = serde_json::from_str(&json).unwrap();
        match game.r#type.as_ref() {
            "gameFull" => { self.parse_game_full(json) },
            "gameState" => { self.parse_game_state(json) },
            "chatLine" => { self.parse_chat_line(json) },
            _ => {
                println!("Error game type = {}", game.r#type)
            }
        };
    }

    fn parse_game_full(&mut self, json: String) {
        let game_full: GameFull = serde_json::from_str(&json).unwrap();
        self.tx.send(game_full.to_engine_game(&self.api)).unwrap();
        self.game_full = Some(game_full);
    }

    fn parse_game_state(&self, json: String) {
        let game_state: data::game_state::GameState = serde_json::from_str(&json).unwrap();
        self.tx.send(game_state.to_engine_game(&self.api, self.game_full.as_ref().unwrap())).expect("Parse game_state");
    }

    fn parse_chat_line(&self, json: String) {
        let chat_line: data::chat_line::ChatLine = serde_json::from_str(&json).unwrap();
        println!("Chat Line: {}", chat_line.text);
    }
}