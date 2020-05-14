use std::cmp::Ordering;
use crate::chess::board::Board;
use crate::chess::colour::Colour;
use crate::chess::r#move::Move;
use super::evaluator::MaterialEvaluator;
use std::time::{Instant, Duration};
use super::engine_config::EngineConfig;

pub struct BoardFrame<'a> {
    pub board: Board,
    pub side: Colour,
    pub depth: isize,
    pub deep_depth: isize,
    pub evaluation: Option<f64>,
    pub moves: Vec<Move>,
    pub time: Instant,
    pub deep_time: Instant,
    pub children: Vec<&'a Self>,
}

impl<'a> BoardFrame<'a> {
    pub fn new(board: Board, side: Colour, config: EngineConfig) -> Self {
        Self {
            board,
            side,
            depth: 0,
            deep_depth: 0,
            time: Instant::now() + Duration::from_secs_f32(config.time),
            deep_time: Instant::now() + Duration::from_secs_f32(config.time * config.deep_frac),
            evaluation: None,
            moves: Vec::default(),
            children: Vec::default(),
        }
    }

    pub fn branch(self, m: Move) -> Self {
        let new_board = self.board.branch(m);
        
        let deep = MaterialEvaluator::evaluate(&new_board, Colour::White) != MaterialEvaluator::evaluate(&self.board, Colour::White)
            || new_board.is_check(new_board.turn) || self.board.is_check(self.board.turn);
        let mut moves = self.moves.clone();
        moves.push(m);
        Self{
            board: new_board,
            side: self.side.opposite(),
            depth: self.depth + if deep {0} else {1},
            deep_depth: self.deep_depth + 1,
            evaluation: None,
            time: self.time,
            deep_time: self.deep_time,
            moves,
            children: Vec::default(),
        }
    }
}

impl Ord for BoardFrame<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth.cmp(&other.depth)
    }
}

impl PartialOrd for BoardFrame<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardFrame<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
    }
}

impl Eq for BoardFrame<'_> {}