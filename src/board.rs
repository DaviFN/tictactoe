pub mod piece;

use std::fmt;
use piece::Piece;
pub struct Board {
    pieces : [Option<Piece>; 9]
}

impl Board {
    pub fn new() -> Self {
        Self { pieces: [None; 9] }
    }

    pub fn restart(&mut self) {
        for piece in &mut self.pieces {
            *piece = None;
        }
    }

    pub fn piece_in(&self, location : &PieceLocation) -> Option<Piece> {
        if !location.is_valid() {
            return None;
        }
        let index_in_pieces_array = location.index_in_x * 3 + location.index_in_y;
        self.pieces[index_in_pieces_array]
    }

    pub fn place_piece(&mut self, location: &PieceLocation, piece: &Piece) -> bool {
        if !location.is_valid() {
            return false;
        }
        let index_in_pieces_array = location.index_in_x * 3 + location.index_in_y;
        self.pieces[index_in_pieces_array] = Some(piece.clone());
        true
    }

    fn string_representing_piece(&self, piece : &Option<Piece>) -> String {
        match piece {
            Some(piece) => piece.to_string(),
            None => String::from("[?]")
        }
    }

    fn array_has_specific_piece_in(&self, specific_piece: &Piece, position: usize) -> bool {
        if(position >= 9) {
            return false;
        }
        match self.pieces[position] {
            None => false,
            Some(piece) => piece == *specific_piece
        }
    }

    pub fn is_in_win_condition_for_piece_type(&self, piece_type: &Piece) -> bool {
        let piece = self.pieces[0];
        // horizontally
        for i in 0..3 {
            let mut row_indicates_victory = true;
            for j in 0..3 {
                if !self.array_has_specific_piece_in(piece_type, 3 * i + j) {
                    row_indicates_victory = false;
                    break;
                }
            }
            if(row_indicates_victory) {
                return true;
            }
        }
        // vertically
        for j in 0..3 {
            let mut column_indicates_victory = true;
            for i in 0..3 {
                if !self.array_has_specific_piece_in(piece_type, 3 * i + j) {
                    column_indicates_victory = false;
                    break;
                }
            }
            if(column_indicates_victory) {
                return true;
            }
        }
        // main diagonal
        if self.array_has_specific_piece_in(piece_type, 0) &&
           self.array_has_specific_piece_in(piece_type, 4) &&
           self.array_has_specific_piece_in(piece_type, 8) {
            return true;
        }
        // secondary diagonal
        if self.array_has_specific_piece_in(piece_type, 2) &&
           self.array_has_specific_piece_in(piece_type, 4) &&
           self.array_has_specific_piece_in(piece_type, 6) {
            return true;
        }
        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    [1] [2] [3]\n[a] {} {} {}\n[b] {} {} {}\n[c] {} {} {}",
        self.string_representing_piece(&self.pieces[0]),
        self.string_representing_piece(&self.pieces[1]),
        self.string_representing_piece(&self.pieces[2]),
        self.string_representing_piece(&self.pieces[3]),
        self.string_representing_piece(&self.pieces[4]),
        self.string_representing_piece(&self.pieces[5]),
        self.string_representing_piece(&self.pieces[6]),
        self.string_representing_piece(&self.pieces[7]),
        self.string_representing_piece(&self.pieces[8]))
    }
}

pub struct PieceLocation {
    index_in_x : usize,
    index_in_y : usize
}

impl PieceLocation {
    pub fn new(location_string : &str) -> Self {
        let mut new_piece_location = PieceLocation {index_in_x : 3, index_in_y: 3};
        
        if location_string.len() == 2 || (location_string.len() == 3 && location_string.chars().nth(1) == Some('-')) {
            let first_char = location_string.chars().nth(0);
            let second_char = match location_string.len() {
                2 => location_string.chars().nth(1),
                3 => location_string.chars().nth(2),
                _ => None
            };

            new_piece_location.index_in_x = match first_char {
                Some('a') => 0,
                Some('b') => 1,
                Some('c') => 2,
                _ => 3
            };
            new_piece_location.index_in_y = match second_char {
                Some('1') => 0,
                Some('2') => 1,
                Some('3') => 2,
                _ => 3
            };
        }
        new_piece_location
    }

    pub fn is_valid(&self) -> bool {
        self.index_in_x < 3 && self.index_in_y < 3
    }
}