use super::colour::Colour;
use super::piece::Piece;
use super::r#move::Move;
use super::position::Position;
use super::piece_kind::PieceKind;
use super::SQUARE_SIZE;
use super::SIZE;
use super::position_iter::PositionIter;
use std::fmt;

pub struct Board {
    pieces: [Option<Piece>; SQUARE_SIZE],
    pub turn: Colour,
    pub castle_white_king_side: bool,
    pub castle_white_queen_side: bool,
    pub castle_black_king_side: bool,
    pub castle_black_queen_side: bool,
    pub en_passant: Option<Position>,
    pub half_move_number: usize,
    pub move_number: usize,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        self.position_iter().for_each(|pos| {
            match self.get(pos) {
                Some(piece) => write!(f, "{:?} ", piece),
                None => write!(f, "   "),
            }.unwrap();
            i += 1;
            if i == SIZE {
                i = 0;
                write!(f, "\n").unwrap();
            }
        });
        fmt::Result::Ok(())
    }
}

impl Board {

    pub fn get(&self, position: Position) -> Option<Piece> {
        self.pieces[position.get_index()?]
    }

    fn set(&mut self, piece: Option<Piece>, position: Position) -> Option<Option<Piece>> {
        let temp = self.pieces[position.get_index()?];
        self.pieces[position.get_index()?] = piece;
        Some(temp)
    }

    pub fn place_piece(&mut self, x: i8, y: i8, piece: Piece) {
        self.set(Some(piece), Position::new(x, y));
    }

    // Copies the board and plays a move
    pub fn branch(&self, m: Move) -> Board {
        let mut board = Board{
            pieces: self.pieces.clone(),
            turn: self.turn.clone(),
            castle_black_king_side: self.castle_black_king_side,
            castle_white_king_side: self.castle_white_king_side,
            castle_black_queen_side: self.castle_black_queen_side,
            castle_white_queen_side: self.castle_white_queen_side,
            en_passant: self.en_passant,
            half_move_number: self.half_move_number,
            move_number: self.move_number,
        };
        board.play_move(m);
        board
    }

    pub fn new() -> Board {
        let pieces: [Option<Piece>; SQUARE_SIZE] = [None; SQUARE_SIZE];
        Board{
            pieces,
            turn: Colour::White,
            castle_black_king_side: false,
            castle_black_queen_side: false,
            castle_white_king_side: false,
            castle_white_queen_side: false,
            en_passant: None,
            half_move_number: 0,
            move_number: 1,
        }
    }

    // Modifies the board by playing a move returns if it is valid or not
    pub fn play_move(&mut self, m: Move) -> bool {
        let from_piece = self.get(m.from);
        let to_piece = self.get(m.to);
        let result = 
            from_piece.is_some() && from_piece.unwrap().colour == self.turn
            && (to_piece.is_none() || to_piece.unwrap().colour != self.turn);
        if !result { return false; }
        let from = from_piece.unwrap();
        match from.kind {
            PieceKind::Rook => self.update_can_castle(m, from.colour),
            PieceKind::King => self.play_king_move(m, from.colour),
            _ => (),
        }
        if from.kind == PieceKind::Pawn && m.to.y * 2 == (SIZE - 1) * (1 + Self::get_pawn_direction(self.turn)) {
            self.set(Some(Piece::new(m.promote.unwrap_or(PieceKind::Queen), self.turn)), m.to);
        } else {
            self.set(from_piece, m.to);
        }
        self.set(None, m.from);
        self.turn = self.turn.opposite();
        result
    }

    fn update_can_castle(&mut self, m: Move, colour: Colour) {
        if m.from.x < SIZE / 2 {
            match colour {
                Colour::White => self.castle_white_queen_side = false,
                Colour::Black => self.castle_black_queen_side = false,
            }
        } else {
            match colour {
                Colour::White => self.castle_white_king_side = false,
                Colour::Black => self.castle_black_king_side = false,
            }
        }
    }

    fn play_king_move(&mut self, m: Move, colour: Colour) {
        let king_delta = m.to.x - m.from.x;
        if king_delta.abs() == 2 {
            let x = 7 * (1 + king_delta / 2) / 2;
            let pos = Position::new(x, m.from.y);
            let rook = self.set(None, pos).unwrap();
            self.set(
                rook,
                Position::new(m.from.x + king_delta / 2, m.from.y)
            );
        }
        match colour {
            Colour::White => {self.castle_white_king_side = false; self.castle_white_queen_side = false;},
            Colour::Black => {self.castle_black_king_side = false; self.castle_black_queen_side = false;},
        }
    }

    pub fn parse_moves(&mut self, moves: &str) -> bool {
        moves == "" || moves.split(' ').map(|move_symbol| {
            let m = Move::from_symbol(move_symbol);
            m.is_some() && self.play_move(m.unwrap())
        }).all(|x| x)
    }

    pub fn position_iter(&self) -> PositionIter {
        PositionIter::new()
    }

    // Finds the king and checks if any pieces are attacking it by calling is_king_in_check
    pub fn is_check(&self, colour: Colour) -> bool {
        self.position_iter().any(
            |pos| {
                let piece = self.get(pos);
                piece.and_then(|p| { Some(p.kind == PieceKind::King && p.colour == colour) } )
                    .unwrap_or(false)
                    && self.is_king_in_check(pos)
            }
        )
    }

    fn is_king_in_check(&self, pos: Position) -> bool {
        let colour = self.get(pos).unwrap().colour;
        let direction = Self::get_pawn_direction(colour);
        self.get(Position::new(pos.x + 1, pos.y + direction)) == Some(Piece::new(PieceKind::Pawn, colour.opposite()))
        || self.get(Position::new(pos.x - 1, pos.y + direction)) == Some(Piece::new(PieceKind::Pawn, colour.opposite()))
        || vec![PieceKind::Bishop, PieceKind::Knight, PieceKind::Queen, PieceKind::Rook, PieceKind::King].iter().any(|kind| {
            self.move_map(pos, *kind).iter().any(|m| {
                self.get(m.to) == Some(Piece::new(*kind, colour.opposite()))
            })
        })
    }

    fn get_pawn_direction(colour: Colour) -> i8 {
        match colour {
            Colour::White => 1i8,
            Colour::Black => -1i8,
        }
    }

    // Returns a vector of all possible moves of the current player
    pub fn possible_moves(&self) -> Vec<Move> {
        self.position_iter().flat_map(|pos| {
            let piece = self.get(pos);
            if piece.is_none() || piece.unwrap().colour != self.turn  {
                return Vec::default().into_iter();
            }
            let Piece{kind, colour: _} = piece.unwrap();
            self.move_map(pos, kind).into_iter()
        }).filter(
            |m| {
                !self.branch(*m).is_check(self.turn)
            }
        ).collect()
    }

    // fn castle_moves(&self, pos: Position, kind: PieceKind) -> Vec<Move> {
        
    // }

    fn move_map(&self, pos: Position, kind: PieceKind) -> Vec<Move> {
        match kind {
            PieceKind::Pawn => self.pawn_moves(pos),
            PieceKind::Knight => self.knight_moves(pos),
            PieceKind::Queen => self.queen_moves(pos),
            PieceKind::King => self.king_moves(pos),
            PieceKind::Bishop => self.bishop_moves(pos),
            PieceKind::Rook => self.rook_moves(pos),
        }
    }

    fn pawn_moves(&self, pos: Position) -> Vec<Move> {
        let mut out = Vec::default();
        let Position{x, y} = pos;
        let direction = Self::get_pawn_direction(self.turn);
        let mut f = |new_pos: Position| -> bool {
            if self.get(new_pos).is_none() {
                out.push(Move::new(pos, new_pos));
                return true;
            }
            false
        };
        if f(Position{x, y: y + direction}) {
            if y == SIZE - 2 || y == 1 {
                f(Position{x, y: y + direction * 2});
            }
        }
        let mut g = |new_pos: Position| -> bool {
            if self.get(new_pos).is_some() && self.get(new_pos).unwrap().colour != self.get(pos).unwrap().colour {
                out.push(Move::new(pos, new_pos));
                return true;
            }
            false
        };
        g(Position{x: x + 1, y: y + direction});
        g(Position{x: x - 1, y: y + direction});
        out
    }

    fn knight_moves(&self, pos: Position) -> Vec<Move> {
        self.general_moves(pos, 1, vec![1, 2, -1, -2, -1, 2, 1, -2])
    }

    fn rook_moves(&self, pos: Position) -> Vec<Move> {
        self.general_moves(pos, SIZE - 1, vec![1, 0, -1, 0])
    }

    fn queen_moves(&self, pos: Position) -> Vec<Move> {
        self.general_moves(pos, SIZE - 1, vec![1, 1, 0, -1, -1, 0, 1, -1])
    }

    fn king_moves(&self, pos: Position) -> Vec<Move> {
        self.general_moves(pos, 1, vec![1, 1, 0, -1, -1, 0, 1, -1])
    }

    fn bishop_moves(&self, pos: Position) -> Vec<Move> {
        self.general_moves(pos, SIZE - 1, vec![1, 1, -1, -1])
    }

    fn general_moves(&self, pos: Position, distance: i8, matrix: Vec<i8>) -> Vec<Move> {
        let mut out = Vec::default();
        let colour = self.get(pos).unwrap().colour;
        let Position{x, y} = pos;
        for i in 0..matrix.len() + 1 {
            let del_x = matrix.get(i % matrix.len()).unwrap();
            let del_y = matrix.get((i + 1) % matrix.len()).unwrap();
            for d in 1..(distance + 1) {
                let position = Position{x: x + *del_x * d, y: y + *del_y * d};
                if position.is_off_board() {
                    break;
                }
                if self.get(position).is_none() {
                    let m = Move::new(pos, position);
                    out.push(m);
                    continue;
                }
                if self.get(position).unwrap().colour != colour {
                    let m = Move::new(pos, position);
                    out.push(m);
                }
                break;
            }
        };
        out
    }
}