use super::board::Board;
use super::colour::Colour;
use super::position::Position;
use super::piece::Piece;
use super::SIZE;

struct FenParser {
    board: Board,
}

impl FenParser {
    fn is_digit(c: char) -> bool {
        48 <= c as usize && c as usize <= 58
    }

    fn parse_piece_positions(&mut self, field: &str) {
        let mut y = SIZE;
        for row in field.split("/") {
            let mut x = 0i8;
            y -= 1;
            for c in row.chars() {
                if FenParser::is_digit(c) {
                    x += c as i8 - 48;
                } else {
                    self.board.place_piece(
                        x,
                        y,
                        Piece::from_symbol(c).unwrap()
                    );
                    x += 1;
                }
            }
        }
    }

    fn parse_active_move(&mut self, field: &str) {
        match field.chars().next() {
            Some('w') => self.board.turn = Colour::White,
            Some('b') => self.board.turn = Colour::Black,
            _ => (),
        };
    }

    fn parse_castling(&mut self, field: &str) {
        for c in field.chars() {
            match c {
                'K' => self.board.castle_white_king_side = true,
                'Q' => self.board.castle_white_queen_side = true,
                'k' => self.board.castle_black_king_side = true,
                'q' => self.board.castle_black_queen_side = true,
                _ => (),
            };
        }
    }

    fn parse_en_passant(&mut self, field: &str) {
        self.board.en_passant = Position::from_symbol(field);
    }

    fn parse_halfmove_clock(&mut self, field: &str) {
        let move_count: Option<usize> = field.parse::<usize>().ok();
        move_count.and_then(|x| {
            self.board.half_move_number = x;
            Some(x)
        });
    }

    fn parse_turn_count(&mut self, field: &str) {
        let move_count: Option<usize> = field.parse::<usize>().ok();
        move_count.and_then(|x| {
            self.board.move_number = x;
            Some(x)
        });
    }
}

pub fn parse(fen: &str) -> Result<Board, ()> {
    if fen == "startpos" {
        return parse("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
    let mut fen_parser = FenParser{
        board: Board::new()
    };
    let fields: Vec<&str> = fen.split(" ").collect();
    fen_parser.parse_piece_positions(fields.get(0).ok_or_else(|| ())?);
    fen_parser.parse_active_move(fields.get(1).ok_or_else(|| ())?);
    fen_parser.parse_castling(fields.get(2).ok_or_else(|| ())?);
    fen_parser.parse_en_passant(fields.get(3).ok_or_else(|| ())?);
    fen_parser.parse_halfmove_clock(fields.get(4).ok_or_else(|| ())?);
    fen_parser.parse_turn_count(fields.get(5).ok_or_else(|| ())?);

    Ok(fen_parser.board)
}