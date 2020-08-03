use crate::chess::r#move::Move;
#[derive(Clone)]
pub struct Evaluated {
    pub moves: Vec<Move>,
    pub current_eval: f64,
}