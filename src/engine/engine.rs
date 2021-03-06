extern crate rand;

use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Instant;
use std::collections::BinaryHeap;
use super::board_frame::BoardFrame;
use super::evaluator::MaterialEvaluator;
use super::engine_config::EngineConfig;
use super::game::Game;
use super::evaluated::Evaluated;
use crate::chess::{board::Board, r#move::Move, fen_parser, colour::Colour};

pub struct Engine {
    frame_queue: BinaryHeap<BoardFrame>,
    max_depth_reached: usize,
    moves_analysed: u128,
    config: EngineConfig,
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

        let (tx, rx): (Sender<Evaluated>, Receiver<Evaluated>) = channel();
        let now = Instant::now();
        let frame = BoardFrame::new(board, game.my_side, self.config, tx, MaterialEvaluator::evaluate(&board, board.turn));
        self.frame_queue.push(frame);
        self.process_frames();
        let x = rx.recv().unwrap();
        println!("Moves analysed: {}", self.moves_analysed);
        println!("Max depth: {}", self.max_depth_reached);
        println!("Result evaluation: {:?}", x.current_eval);
        println!("Best line: {:?}", x.moves);
        println!("Elapsed: {}", (Instant::now() - now).as_secs_f32());
        Some(x.moves[0])
    }
}

impl Engine {
    fn process_frames(&mut self) {
        println!("Processing Frames");
        while let Some(mut frame) = self.frame_queue.pop() {
            
            let now = Instant::now();
            // println!("{:?} {}", frame.moves, frame.child_count);
            if frame.child_count == 0 {
                if frame.time < now || (frame.deep_time < now && frame.depth >= self.config.deep_depth) {
                    frame.evaluation.send(Evaluated{moves: frame.moves.clone(), current_eval: MaterialEvaluator::evaluate(&frame.board, Colour::White)}).unwrap_or(());
                } else {
                    if frame.board.is_checkmate() {
                        frame.evaluation.send(Evaluated{moves: frame.moves.clone(), current_eval: frame.side.to_num() as f64 * -10000f64}).unwrap();
                        continue;
                    }
                    let pos_moves = frame.board.possible_moves();
                    self.moves_analysed += 1;
                    if self.max_depth_reached < frame.moves.len() {
                        self.max_depth_reached = frame.moves.len();
                    }
                    let (tx, rx): (Sender<Evaluated>, Receiver<Evaluated>) = channel();
                    frame.child_count = pos_moves.len();
                    frame.children = Some(rx);
                    for i in pos_moves {
                        let mut c: BoardFrame = frame.branch(i, tx.clone());
                        c.evaluation = tx.clone();
                        self.frame_queue.push(c);
                    }
                    self.frame_queue.push(frame);
                }
            } else {
                match frame.children {
                    Some(ref rx) => {
                        loop {
                            if frame.child_count > 0 {
                                match rx.try_recv() {
                                    Ok(v) => {
                                        frame.child_count -= 1;
                                        if frame.current_eval.is_none() {
                                            frame.current_eval = Some(v);
                                            continue;
                                        }
                                        let eval = frame.current_eval.clone().unwrap();
                                        if (frame.side == Colour::White && eval.current_eval < v.current_eval) || (frame.side == Colour::Black && eval.current_eval > v.current_eval) {
                                            frame.current_eval = Some(v);
                                        }
                                    },
                                    Err(_) => {
                                        frame.delay += 1;
                                        self.frame_queue.push(frame);
                                        break;
                                    }
                                }
                            } else {
                                frame.evaluation.send(frame.current_eval.unwrap()).unwrap();
                                break;
                            }
                        }
                    },
                    None => (),
                }
            }
        }
    }
}