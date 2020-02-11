use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl fmt::Debug for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_symbol())
    }
}

impl PieceKind {
    pub fn to_symbol(&self) -> char {
        match self {
            PieceKind::Rook => 'R',
            PieceKind::Knight => 'N',
            PieceKind::Queen => 'Q',
            PieceKind::Bishop => 'B',
            PieceKind::King => 'K',
            PieceKind::Pawn => 'P',
        }
    }

    pub fn from_symbol(piece_symbol: char) -> Option<PieceKind> {
        match piece_symbol {
            'R' => Some(PieceKind::Rook),
            'N' => Some(PieceKind::Knight),
            'B' => Some(PieceKind::Bishop),
            'Q' => Some(PieceKind::Queen),
            'K' => Some(PieceKind::King),
            'P' => Some(PieceKind::Pawn),
            _ => None,
        }
    }
}