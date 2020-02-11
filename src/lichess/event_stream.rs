extern crate serde;
extern crate reqwest;

use serde_json;
use std::io::prelude::*;
use std::convert::AsRef;
use std::fmt::Write;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Sender;

use super::data;
use super::API;

use crate::engine::game::Game;

impl API {
    pub fn run_event_stream(&self, tx: Sender<Game>) -> Result<(), ()> {
        let response = self.auth_get(String::from("stream/event")).expect("Get stream event");
        if !response.status().is_success() {
            println!("{} => {}", response.url(), response.status());
            return Err(());
        }
        let mut buffer = String::default();
        let mut threads: Vec<JoinHandle<()>> = Vec::default();
        for b in response.bytes() {
            let c = match b {
                Ok(c) => c as char,
                Err(_) => continue,
            };
            if c == '\n' {
                self.parse_event(buffer.clone(), tx.clone())
                    .and_then(|thread| {Some(threads.push(thread))});
                buffer = String::default();
            } else {
                buffer.write_char(c).expect("Write character to buffer");
            }
        }
        // Cleanup threads
        for thread in threads.into_iter() {
            thread.join().expect("Join threads");
        }
        Ok(())
    }

    fn parse_event(&self, json: String, tx: Sender<Game>) -> Option<JoinHandle<()>> {
        if json == "" {
            return None;
        }
        let event: data::event::Event = serde_json::from_str(&json).expect("Cannot deserialize event json");
        let api = self.clone();
        Some(match event.r#type.as_ref() {
            "gameStart" => {
                let game_id = event.game.unwrap().id;
                println!("Game Start Event id = {}", game_id);
                thread::spawn(move || {
                    api.run_game_stream(game_id, tx);
                })
            }
            "challenge" => {
                let challenge_details = event.challenge.unwrap();
                println!("Challenge Event id = {}", challenge_details.id);
                thread::spawn(move || {
                    api.decide_challenge(challenge_details);
                })
            }
            _ => {
                println!("Error event type = {}", event.r#type);
                thread::spawn(|| {})
            }
        })
    }
}