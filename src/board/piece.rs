use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Piece {
    Cross,
    Circle
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Cross => write!(f, "[X]"),
            Self::Circle => write!(f, "[O]")
        }
    }
}