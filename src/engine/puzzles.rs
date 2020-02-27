use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::engine::engine::Engine;
use crate::engine::engine_config::EngineConfig;

const PUZZLES_PATH: &str = "puzzles.txt";

fn get_puzzles_buf() -> BufReader<File> {

    let file = File::open(PUZZLES_PATH).unwrap();
    let reader = BufReader::new(file);
    reader
}

pub fn get_puzzle(puzzle_line: usize) -> Option<(String, String)> {
    let reader = get_puzzles_buf();
    let line = reader.lines().nth(puzzle_line - 1)?;
    parse_puzzle(line.unwrap())
}

pub fn next_move(fen: &String, moves: &String, config: EngineConfig) -> bool {

    let best_move = Engine::run_fen_engine(String::from(fen), config);
    println!("Engine move is {}", best_move.unwrap());
    match best_move {
        Some(m) => moves.split(" ").any(|item| {m.to_symbol() == item}),
        None => false,
    }
}

fn parse_puzzle(line: String) -> Option<(String, String)> {
    if !line.contains("=") || line.starts_with("#") || line.starts_with("//") {
        return None;
    }
    let mut line_split = line.split("=>");
    let fen = line_split.next().unwrap();
    let moves = line_split.next();
    Some((String::from(fen), String::from(moves.unwrap())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_puzzles() {
        let config = EngineConfig{
            depth: 1,
            deep_depth: 3,
            debug: false,
        };
        let reader = get_puzzles_buf();
        for (i, line) in reader.lines().enumerate() {
            let line: String = line.unwrap();
            match parse_puzzle(line) {
                Some((fen, moves)) => {
                    println!("\nPuzzle Line {}: {} = {}", i + 1, fen, moves);
                    let result = next_move(&fen, &moves, config);
                    assert!(result);
                }
                None => (),
            };
        }
    }
}
