use super::colour::Colour;
use super::piece_kind::PieceKind;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub colour: Colour,
    pub kind: PieceKind,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.colour, self.kind)
    }
}

impl Piece {
    pub fn from_symbol(piece_symbol: char) -> Option<Piece> {
        let (colour, piece_symbol) = 
        if 65 <= piece_symbol as u8 && piece_symbol as u8 <= 90 {
            (Colour::White, piece_symbol)
        } else {
            (Colour::Black, (piece_symbol as u8 - 32) as char)
        };
        Some(Piece{
            kind: PieceKind::from_symbol(piece_symbol)?
            , colour: colour
        })
    }

    pub fn new(kind: PieceKind, colour: Colour) -> Self {
        Self{kind, colour}
    }
}
