use super::position::Position;
use super::piece_kind::PieceKind;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promote: Option<PieceKind>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_symbol())
    }
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Self{
            from,
            to,
            promote: None,
        }
    }

    pub fn from_symbol(symbol: &str) -> Option<Move> {
        if symbol == "" || symbol.len() < 4 {
            return None;
        }
        Some(Move{
            from: Position::from_symbol(&symbol[0..2])?,
            to: Position::from_symbol(&symbol[2..4])?,
            promote: if symbol.len() >= 5 { PieceKind::from_symbol((symbol.chars().nth(4)? as u8 - 32) as char) } else { None },
        })
    }

    pub fn to_symbol(&self) -> String {
        format!("{}{}", self.from.to_symbol(), self.to.to_symbol())
    }
}