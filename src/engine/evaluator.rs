use crate::chess::board::Board;
use crate::chess::colour::Colour;
use crate::chess::piece::Piece;
use crate::chess::piece_kind::PieceKind;


pub struct MaterialEvaluator {}

impl MaterialEvaluator {
    pub fn evaluate(board: &Board, side: Colour) -> f64 {
        let mut sum = 0;
        for pos in board.position_iter() {
            sum += match board.get(pos) {
                Some(p) => (if p.colour == side {1} else {-1}) * Self::get_piece_value(p),
                None => 0,
            };
        }
        sum as f64
    }

    fn get_piece_value(piece: Piece) -> i8 {
        match piece.kind {
            PieceKind::King => 0,
            PieceKind::Queen => 9,
            PieceKind::Rook => 5,
            PieceKind::Bishop => 3,
            PieceKind::Knight => 3,
            PieceKind::Pawn => 1,
        }
    }
} 