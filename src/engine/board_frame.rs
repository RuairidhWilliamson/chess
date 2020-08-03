use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use crate::chess::board::Board;
use crate::chess::colour::Colour;
use crate::chess::r#move::Move;
use super::evaluated::Evaluated;
use super::evaluator::MaterialEvaluator;
use std::time::{Instant, Duration};
use super::engine_config::EngineConfig;
use std::sync::mpsc::*;

pub struct BoardFrame {
    pub board: Board,
    pub side: Colour,
    pub depth: isize,
    pub deep_depth: isize,
    pub moves: Vec<Move>,
    pub time: Instant,
    pub deep_time: Instant,
    pub child_count: usize,
    pub children: Option<Receiver<Evaluated>>,
    pub evaluation: Sender<Evaluated>,
    pub current_eval: Option<Evaluated>,
    pub delay: usize,
    pub heuristic: f64,
}

impl BoardFrame {
    pub fn new(board: Board, side: Colour, config: EngineConfig, evaluation: Sender<Evaluated>, heuristic: f64) -> Self {
        Self {
            board,
            side,
            depth: 0,
            deep_depth: 0,
            time: Instant::now() + Duration::from_secs_f32(config.time),
            deep_time: Instant::now() + Duration::from_secs_f32(config.time * config.deep_frac),
            moves: Vec::default(),
            child_count: 0,
            children: None,
            evaluation: evaluation,
            current_eval: None,
            delay: 0,
            heuristic: heuristic,
        }
    }

    pub fn branch(&self, m: Move, evaluation: Sender<Evaluated>) -> Self {
        let new_board = self.board.branch(m);
        
        let eval = MaterialEvaluator::evaluate(&new_board, new_board.turn);
        let deep = eval != -self.heuristic || new_board.is_check(new_board.turn) || self.board.is_check(self.board.turn);
        let mut moves = self.moves.clone();
        moves.push(m);
        Self {
            board: new_board,
            side: self.side.opposite(),
            depth: self.depth + if deep {0} else {1},
            deep_depth: self.deep_depth + 1,
            time: self.time,
            deep_time: self.deep_time,
            moves,
            child_count: 0,
            children: None,
            evaluation: evaluation,
            current_eval: None,
            delay: 0,
            heuristic: eval,
        }
    }
}

impl Ord for BoardFrame {
    fn cmp(&self, other: &Self) -> Ordering {
        let h = self.heuristic.partial_cmp(&other.heuristic).unwrap_or(Equal);
        let x = (self.delay, &self.deep_depth, &self.depth).cmp(&(other.delay, &other.deep_depth, &other.depth)).reverse();
        if x == Equal {
            return h;
        } else {
            return x;
        }
    }
}

impl PartialOrd for BoardFrame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardFrame {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
    }
}

impl Eq for BoardFrame {}