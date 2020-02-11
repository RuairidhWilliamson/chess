extern crate reqwest;

mod data;
mod event_stream;
mod game_stream;
use crate::config;

use std::sync::mpsc::Sender;
use crate::engine::game::Game;

use reqwest::{Client, Response, Error};

#[derive(Clone)]
pub struct API {
    config: config::Config,
}

impl API {
    pub fn new() -> Self {
        Self {
            config: config::load_env().expect("Could not load config"),
        }
    }

    fn build_url(&self, url: String) -> String {
        format!("{}{}", self.config.lichess.base_url, url)
    }

    fn check_response(request: Result<Response, Error>) -> bool {
        match request {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    pub fn auth_get(&self, url: String) -> Result<Response, Error> {
        Client::new().get(&self.build_url(url)).bearer_auth(&self.config.lichess.api_key).send()
    }

    pub fn auth_post(&self, url: String) -> Result<Response, Error> {
        Client::new().post(&self.build_url(url)).bearer_auth(&self.config.lichess.api_key).send()
    }

    #[allow(dead_code)]
    pub fn check_auth_get(&self, url: String) -> bool {
        Self::check_response(self.auth_get(url))
    }

    pub fn check_auth_post(&self, url: String) -> bool {
        Self::check_response(self.auth_post(url))
    }

    pub fn decide_challenge(&self, challenge: data::challenge::Challenge) -> bool {
        if self.config.lichess.challenge_filter.variant_whitelist.contains(&challenge.variant.key)
            && self.config.lichess.challenge_filter.time_control_whitelist.contains(&challenge.time_control.r#type) {
            self.accept_challenge(challenge.id)
        } else {
            self.decline_challenge(challenge.id)
        }
    }
    pub fn accept_challenge(&self, challenge_id: String) -> bool {
        self.check_auth_post(format!("challenge/{}/accept", challenge_id))
    }
    
    pub fn decline_challenge(&self, challenge_id: String) -> bool {
        self.check_auth_post(format!("challenge/{}/decline", challenge_id))
    }

    pub fn make_move(&self, game_id: &str, move_symbol: &str) -> bool {
        self.check_auth_post(format!("bot/game/{}/move/{}", game_id, move_symbol))
    }

    #[allow(dead_code)]
    pub fn abort_game(&self, game_id: &str) -> bool {
        self.check_auth_post(format!("bot/game/{}/abort", game_id))
    }

    #[allow(dead_code)]
    pub fn resign_game(&self, game_id: &str) -> bool {
        self.check_auth_post(format!("bot/game/{}/resign", game_id))
    }
}

pub fn start_bot(tx: Sender<Game>) -> Result<(), ()> {
    API::new().run_event_stream(tx).or(Err(()))
}