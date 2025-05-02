use crate::piece::{Piece, Position, XAxis, YAxis};
use crate::{BottomLeft, BottomRight, UpperLeft, UpperRight};

pub struct Board {
    pieces: Vec<Box<dyn Piece>>,
    dimensions: (XAxis, YAxis),
}

impl Board {
    pub fn get_pieces(&self) -> impl Iterator<Item = &Box<dyn Piece>> {
        self.pieces.iter()
    }

    pub fn get_limits(&self) -> (BottomLeft, BottomRight, UpperLeft, UpperRight) {
        let bl = BottomLeft(Position {
            x: XAxis::new(0u8),
            y: YAxis::new(0u8),
        });
        let br = BottomRight(Position {
            x: self.dimensions.0.clone(),
            y: YAxis::new(0u8),
        });
        let ul = UpperLeft(Position {
            x: XAxis::new(0u8),
            y: self.dimensions.1.clone(),
        });
        let ur = UpperRight(Position {
            x: self.dimensions.0.clone(),
            y: self.dimensions.1.clone(),
        });

        debug_assert!(ur.0.y == ul.0.y);
        debug_assert!(ur.0.x == br.0.x);
        debug_assert!(ul.0.x == bl.0.x);
        debug_assert!(bl.0.y == br.0.y);

        (bl, br, ul, ur)
    }
}
