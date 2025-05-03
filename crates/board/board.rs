use crate::piece::{DiagonalRange, Piece, PlusRange, Position, StarRange, XAxis, YAxis};
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

    pub fn is_inside(&self, pos: &Position) -> bool {
        let (bl, br, ul, ur) = self.get_limits();

        let within_x = bl.0.x <= pos.x && pos.x <= br.0.x;
        let within_y = bl.0.y <= pos.y && pos.y <= ul.0.y;

        within_x && within_y
    }

    pub fn diagonal_range(&self, origin: Position, limit: Option<u8>) -> DiagonalRange {
        let (bl, br, ul, ur) = self.get_limits();

        // We do each diagonal line
        //         1\   /2
        //           \ /
        //            o
        //           / \
        //         3/   \4
        let diagonals = Vec::new();

        let center_to_ul = {
            let mut upper_left = Vec::new();
            let mut current = origin;

            while current.x >= ul.0.x && current.y <= ul.0.y {
                if let Some(limit) = limit {
                    if upper_left.len() >= limit.into() {
                        break;
                    }
                };
                current.x -= 1.into();
                current.y += 1.into();
                upper_left.push(current);
            }
            upper_left
        };
        let center_to_ur = {
            let mut upper_right = Vec::new();
            let mut current = origin;

            while current.x <= ur.0.x && current.y <= ur.0.y {
                if let Some(limit) = limit {
                    if upper_right.len() >= limit.into() {
                        break;
                    }
                };
                current.x += 1.into();
                current.y += 1.into();
                upper_right.push(current);
            }
            upper_right
        };
        let center_to_bl = {
            let mut bottom_left = Vec::new();
            let mut current = origin;

            while current.x >= bl.0.x && current.y >= bl.0.y {
                if let Some(limit) = limit {
                    if bottom_left.len() >= limit.into() {
                        break;
                    }
                };
                current.x -= 1.into();
                current.y -= 1.into();
                bottom_left.push(current);
            }
            bottom_left
        };
        let center_to_br = {
            let mut bottom_right = Vec::new();
            let mut current = origin;

            while current.x <= br.0.x && current.y >= br.0.y {
                if let Some(limit) = limit {
                    if bottom_right.len() >= limit.into() {
                        break;
                    }
                };
                current.x += 1.into();
                current.y -= 1.into();
                bottom_right.push(current);
            }
            bottom_right
        };

        DiagonalRange({
            diagonals
                .into_iter()
                .chain(center_to_ul)
                .chain(center_to_ur)
                .chain(center_to_bl)
                .chain(center_to_br)
                .collect()
        })
    }
}
