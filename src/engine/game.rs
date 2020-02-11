use crate::chess::board::Board;
use crate::chess::colour::Colour;

pub struct Game {
    pub game_id: String,
    pub initial_board: Board,
    pub moves: String,
    pub my_side: Colour,
}