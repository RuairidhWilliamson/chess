extern crate rand;

use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Instant;
use std::collections::BinaryHeap;
use super::board_frame::BoardFrame;
use super::evaluator::MaterialEvaluator;
use super::engine_config::EngineConfig;
use super::game::Game;
use crate::chess::{board::Board, r#move::Move, fen_parser};

pub struct Engine<'a> {
    frame_queue: BinaryHeap<BoardFrame<'a>>,
    max_depth_reached: isize,
    moves_analysed: u128,
    config: EngineConfig
}

impl Engine<'_> {
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
        let frame = BoardFrame::new(board, game.my_side, self.config);
        self.frame_queue.push(frame);
        self.process_frames();
        None
    }

    fn process_frames(&mut self) {
        while let Some(mut frame) = self.frame_queue.pop() {
            let now = Instant::now();
            if frame.children.is_empty() {
                if frame.time < now || (frame.deep_time < now && frame.depth >= self.config.deep_depth) {
                    frame.evaluation = Some(MaterialEvaluator::evaluate(&frame.board, frame.side));
                } else {
                    let pos_moves = frame.board.possible_moves();
                    for i in pos_moves {
                        self.frame_queue.push(frame.branch(i));
                    }
                }
            } else {

            }
            self.frame_queue.push(frame);
        }
    }
}