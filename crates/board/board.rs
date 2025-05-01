use crate::pieces::{Piece, XAxis, YAxis};

pub struct Board {
    pieces: Vec<Piece>,
    dimensions: (XAxis, YAxis),
}

impl Board {
    pub fn get_pieces(&self) -> impl Iterator<Item = &Piece> {
        self.pieces.iter()
    }
}
