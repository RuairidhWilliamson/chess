pub mod board;

pub mod colour;
pub mod piece_kind;
pub mod piece;
pub mod position;
pub mod r#move;
pub mod fen_parser;
pub mod position_iter;

pub const SIZE: i8 = 8;
pub const SQUARE_SIZE: usize = (SIZE * SIZE) as usize;