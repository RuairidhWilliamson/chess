extern crate rand;

use std::io::{self, Write};
use std::f64;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Instant;
use std::collections::BinaryHeap;
use super::board_frame::BoardFrame;
use super::evaluator::MaterialEvaluator;
use super::engine_config::EngineConfig;
use super::game::Game;
use crate::chess::{board::Board, r#move::Move, fen_parser};
use rand::{seq::SliceRandom, thread_rng};

pub struct Engine {
    frame_queue: BinaryHeap<BoardFrame>,
    max_depth_reached: isize,
    moves_analysed: u128,
    config: EngineConfig
}

impl Engine {
    pub fn channels() -> (Sender<Game>, Receiver<Game>) {
        channel::<Game>()
    }

    pub fn run_fen_engine(fen: String, config: EngineConfig) -> Option<Move> {
        let mut engine = Engine{
            frame_queue: BinaryHeap::new(),
            max_depth_reached: 0,
            moves_analysed: 0,
            config,
        };

        let board: Board = fen_parser::parse(&fen).unwrap();
        let side = board.turn;
        let game: Game = Game{
            game_id: String::default(),
            initial_board: board,
            moves: String::default(),
            my_side: side,
        };
        engine.receive_game(game)
    }

    pub fn start_receiver_engine(rx: Receiver<Game>, perform_move: fn(&str, &str) -> bool, config: EngineConfig) {
        let mut engine = Engine{
            frame_queue: BinaryHeap::new(),
            max_depth_reached: 0,
            moves_analysed: 0,
            config,
        };
        for game in rx.iter() {
            let game_id = &game.game_id.clone();
            let best_move = engine.receive_game(game);
            match best_move {
                Some(m) => perform_move(game_id, &m.to_symbol()),
                None => false,
            };
        }
    }

    fn receive_game(&mut self, game: Game) -> Option<Move> {
        let mut board = game.initial_board;
        if !&board.parse_moves(&game.moves) {
            return None;
        }
        if board.turn != game.my_side {
            return None;
        }
        let frame = BoardFrame{
            board,
            priority: 1,
            depth: self.config.depth,
            deep_depth: self.config.deep_depth,
            game_id: game.game_id,
            side: game.my_side,
            evaluation: None,
            moves: Vec::default(),
        };
        self.frame_queue.push(frame);
        self.process_frames();
        None
    }

    fn process_frames(&mut self) {
        loop {
            let mut frame: BoardFrame = match self.frame_queue.pop() {
                Some(f) => f,
                None => break,
            };
            if frame.depth <= 0 || frame.deep_depth <= 0 {
                frame.evaluation = Some(MaterialEvaluator::evaluate(&frame.board, frame.side));
            } else {
                for i in frame.board.possible_moves() {
                    self.frame_queue.push(frame.branch(i));
                }
            }
        }
    }

    fn process_move(&mut self, frame: &BoardFrame) -> Option<Move> {
        let board: &Board = &frame.board;
        let possible_moves = board.possible_moves();
        if possible_moves.is_empty() {
            return None;
        }
        self.max_depth_reached = 0;
        self.moves_analysed = 0;
        if self.config.debug {
        println!("\n--Processing move--");
        println!("There are {} moves", possible_moves.len());
        }
        
        let now = Instant::now();
        let (best_move, highest_value) = self.recursive_first_move(&frame);
        if self.config.debug {
            println!("Evaluated: {} => {}", best_move, highest_value);
            println!("{} total moves analysed. {} max depth.", self.moves_analysed, self.max_depth_reached);
            println!("{}s", now.elapsed().as_secs_f32());
        
            let opponents_frame = frame.branch(best_move);
            if self.config.debug && !opponents_frame.board.possible_moves().is_empty() {
                let (opponents_best_move, opponents_value) = self.recursive_first_move(&opponents_frame);
                println!("Opponent: {} => {}", opponents_best_move, -opponents_value);
            }
            println!("--Finished processing move--");
        }
        return Some(best_move);
    }

    fn recursive_first_move(&mut self, frame: &BoardFrame) -> (Move, f64) {
        let board: &Board = &frame.board;
        let possible_moves = board.possible_moves();
        let mut highest_value = f64::NEG_INFINITY;
        let mut best_moves: Vec<Move> = Vec::default();
        let mut c = 0;
        let total = possible_moves.len();
        for i in possible_moves {
            c += 1;
            print!("    {} {}%\r", i, 100 * c / total);
            // print!("{}", i);
            io::stdout().flush().unwrap();
            let frame = frame.branch(i);
            let value = self.recursive_move(&frame);
            // println!(" => {}       ", value);
            if value == f64::INFINITY {
                best_moves.clear();
                best_moves.push(i);
                break;
            }
            if value > highest_value {
                highest_value = value;
                best_moves.clear();
            }
            if value == highest_value {
                best_moves.push(i);
            }
        }
        print!("                     \r");
        io::stdout().flush().unwrap();
        if self.config.debug {
            println!("There are {} moves that are evaluated at {}", best_moves.len(), highest_value);
        }
        let mut rng = thread_rng();
        let best_move = best_moves.choose(&mut rng).unwrap();
        (*best_move, highest_value)
    }

    fn recursive_move(&mut self, frame: &BoardFrame) -> f64 {
        let board = &frame.board;
        self.moves_analysed += 1;
        let possible_moves = board.possible_moves();
        if possible_moves.len() == 0 {
            return if board.is_check(frame.side) { f64::INFINITY } else { 0f64 };
        }
        if frame.depth <= 0 || frame.deep_depth <= 0 {
            if self.max_depth_reached < self.config.deep_depth - frame.deep_depth {
                self.max_depth_reached = self.config.deep_depth - frame.deep_depth;
            }
            return -MaterialEvaluator::evaluate(&board, frame.side);
        }
        let mut highest_value = f64::NEG_INFINITY;
        for i in possible_moves {
            let new_frame = &frame.branch(i);
            let value = self.recursive_move(new_frame);
            if value == f64::INFINITY {
                highest_value = value;
            }
            if value > highest_value {
                highest_value = value;
            }
        }
        return -highest_value;
    }
}