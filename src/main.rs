mod lichess;
mod chess;
mod engine;
mod config;

use engine::puzzles::{get_puzzle, next_move};
use engine::engine::Engine;
use engine::engine_config::EngineConfig;
use std::env;
use std::thread;
use chess::fen_parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "puzzle" {
        run_puzzle(args);
        return;
    }

    if args.len() > 2 {
        run_fen(args);
        return;
    }

    run_lichess_bot(args);
}

fn run_puzzle(args: Vec<String>) {
    let config = EngineConfig{
        depth: 2,
        deep_depth: 5,
        debug: true,
    };
    let puzzle_line_number = args[2].parse::<usize>().unwrap();
    match get_puzzle(puzzle_line_number) {
        Some((fen, moves)) => {
            println!("{:?}", parse(&fen).unwrap());
            next_move(&fen, &moves, config);
        },
        None => (),
    };
}

fn run_fen(args: Vec<String>) {
    let config = EngineConfig{
        depth: 2,
        deep_depth: 5,
        debug: false,
    };
    let fen = args[1..].join(" ");
    Engine::run_fen_engine(fen, config);
}

fn run_lichess_bot(args: Vec<String>) {
    let config = EngineConfig{
        depth: 2,
        deep_depth: 5,
        debug: args.get(1).unwrap_or(&String::default()) == "debug",
    };
    let (tx, rx) = Engine::channels();
    let handle = thread::spawn(move || {
        let make_move_func = |game_id: &str, r#move: &str| {
            crate::lichess::API::new().make_move(game_id, r#move)
        };
        Engine::start_receiver_engine(rx, make_move_func, config);
    });
    lichess::start_bot(tx).unwrap();
    handle.join().unwrap();
}
