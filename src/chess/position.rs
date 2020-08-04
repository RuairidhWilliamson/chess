use super::SIZE;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: i8,
    pub y: i8,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_symbol())
    }
}


impl Position {
    pub fn new(x: i8, y: i8) -> Self {
        Self{x, y}
    }

    pub fn get_index(&self) -> Option<usize> {
        if self.is_off_board() {
            return None;
        }
        Some((self.x + self.y * SIZE) as usize)
    }

    pub fn is_off_board(&self) -> bool {
        self.x >= SIZE || self.y >= SIZE || self.x < 0 || self.y < 0
    }

    pub fn from_symbol(symbol: &str) -> Option<Position> {
        let mut chars = symbol.chars();
        let col = chars.next()?;
        let row = chars.next()?;
        Some(Position{x: col as i8 - 97, y: row as i8 - 49})
    }

    pub fn to_symbol(&self) -> String {
        if self.is_off_board() {
            return String::default();
        }
        format!("{}{}", (self.x + 97) as u8 as char, (self.y + 49) as u8 as char)
    }

    pub fn average(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x) / 2,
            y: (self.y + other.y) / 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_off_board() {
        let pos = Position{x: -1, y: 0};
        assert!(pos.is_off_board());
    }

    #[test]
    fn y_off_board() {
        let pos = Position{x: 0, y: -1};
        assert!(pos.is_off_board());
    }
}