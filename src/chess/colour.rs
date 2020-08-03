use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

impl fmt::Debug for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_symbol())
    }
}

impl Colour {
    pub fn to_symbol(&self) -> char {
        match self {
            Colour::White => 'W',
            Colour::Black => 'B',
        }
    }

    pub fn opposite(&self) -> Colour {
        if self == &Colour::White { Colour::Black } else { Colour::White }
    }

    pub fn to_num(&self) -> isize {
        match self {
            Colour::White => 1,
            Colour::Black => -1,
        }
    }
}