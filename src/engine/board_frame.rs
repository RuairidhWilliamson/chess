use std::cmp::Ordering;
use crate::chess::board::Board;
use crate::chess::colour::Colour;
use crate::chess::r#move::Move;
use super::evaluator::MaterialEvaluator;

pub struct BoardFrame {
    pub board: Board,
    pub game_id: String,
    pub side: Colour,
    pub priority: usize,
    pub depth: isize,
    pub deep_depth: isize,
}

impl BoardFrame {
    pub fn branch(&self, m: Move) -> Self {
        let new_board = self.board.branch(m);
        
        let deep = MaterialEvaluator::evaluate(&new_board, Colour::White) != MaterialEvaluator::evaluate(&self.board, Colour::White)
            || new_board.is_check(new_board.turn);

        Self{
            board: new_board,
            game_id: self.game_id.clone(),
            side: self.side.opposite(),
            priority: self.priority,
            depth: self.depth - if deep {0} else {1},
            deep_depth: self.deep_depth - 1,
        }
    }
}

impl Ord for BoardFrame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth.cmp(&other.depth)
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