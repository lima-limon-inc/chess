use crate::piece::{
    Color, DiagonalRange, HorizontalRange, Move, Piece, PlusRange, Position, StarRange,
    VerticalRange, XAxis, YAxis,
};
use crate::pieces::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::{BottomLeft, BottomRight, UpperLeft, UpperRight};

pub struct Board {
    pieces: Vec<Box<dyn Piece>>,
    dimensions: (XAxis, YAxis),
}

impl Board {
    /// Only intended for testing
    fn new(pieces: Vec<Box<dyn Piece>>) -> Self {
        let dimensions = (XAxis::new(7), YAxis::new(7));

        Board { pieces, dimensions }
    }

    pub fn get_pieces(&self) -> impl Iterator<Item = &Box<dyn Piece>> {
        self.pieces.iter()
    }

    pub fn get_limits(&self) -> (BottomLeft, BottomRight, UpperLeft, UpperRight) {
        let bl = BottomLeft(Position {
            x: XAxis::new(0i8),
            y: YAxis::new(0i8),
        });
        let br = BottomRight(Position {
            x: self.dimensions.0.clone(),
            y: YAxis::new(0i8),
        });
        let ul = UpperLeft(Position {
            x: XAxis::new(0i8),
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

    pub fn horizontal_range(&self, origin: Position, limit: Option<u8>) -> HorizontalRange {
        let (bl, br, ul, ur) = self.get_limits();

        let up: Vec<_> = { (origin.y.0..ur.0.y.0).collect() };
        let down: Vec<_> = { (ur.0.y.0..origin.y.0).rev().collect() };

        let (up, down): (Vec<_>, Vec<_>) = if let Some(limit) = limit {
            (
                up.into_iter().take(limit.into()).collect(),
                down.into_iter().take(limit.into()).collect(),
            )
        } else {
            (up.into_iter().collect(), down.into_iter().collect())
        };

        HorizontalRange(
            up.into_iter()
                .chain(down)
                .map(|y_axis| Position::new(origin.x, YAxis::new(y_axis)))
                .collect(),
        )
    }

    pub fn vertical_range(&self, origin: Position, limit: Option<u8>) -> VerticalRange {
        let (bl, br, _, _) = self.get_limits();

        let left: Vec<_> = { (bl.0.x.0..origin.x.0).rev().collect() };
        let right: Vec<_> = { (origin.x.0..br.0.x.0).collect() };

        let (left, right): (Vec<_>, Vec<_>) = if let Some(limit) = limit {
            (
                left.into_iter().take(limit.into()).collect(),
                right.into_iter().take(limit.into()).collect(),
            )
        } else {
            (left.into_iter().collect(), right.into_iter().collect())
        };

        VerticalRange(
            left.into_iter()
                .chain(right)
                .map(|x_axis| Position::new(XAxis::new(x_axis), origin.y))
                .collect(),
        )
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

    pub fn get_moves_from(&self, pos: Position) -> Option<Vec<Move>> {
        self.get_pieces()
            .find(|piece| piece.get_position() == pos)
            .map(|piece| piece.available_positions(self))
    }

    pub fn execute_move(&mut self, mov: Move) {
        // TODO: Remove unwrap
        let piece = self
            .get_pieces()
            .find(|piece| piece.get_position() == mov.origin)
            .unwrap();
    }
}

impl Default for Board {
    fn default() -> Self {
        let dimensions = (XAxis::new(7), YAxis::new(7));
        #[rustfmt::skip]
        let pieces: Vec<Box<dyn Piece>> = vec![
            // White pieces
            // Pawns
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(0), YAxis::new(1)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(1), YAxis::new(1)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(2), YAxis::new(1)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(3), YAxis::new(1)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(4), YAxis::new(1)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(5), YAxis::new(1)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(6), YAxis::new(1)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(7), YAxis::new(1)))),
            // Other pieces
            Box::new(Rook::new(Color::White, Position::new(XAxis::new(0), YAxis::new(0)))),
            Box::new(Knight::new(Color::White, Position::new(XAxis::new(1), YAxis::new(0)))),
            Box::new(Bishop::new(Color::White, Position::new(XAxis::new(2), YAxis::new(0)))),
            Box::new(Queen::new(Color::White, Position::new(XAxis::new(3), YAxis::new(0)))),
            Box::new(King::new(Color::White, Position::new(XAxis::new(4), YAxis::new(0)))),
            Box::new(Bishop::new(Color::White, Position::new(XAxis::new(5), YAxis::new(0)))),
            Box::new(Knight::new(Color::White, Position::new(XAxis::new(6), YAxis::new(0)))),
            Box::new(Rook::new(Color::White, Position::new(XAxis::new(7), YAxis::new(0)))),

            // Black pieces
            // Pawns
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(0), YAxis::new(6)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(1), YAxis::new(6)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(2), YAxis::new(6)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(3), YAxis::new(6)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(4), YAxis::new(6)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(5), YAxis::new(6)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(6), YAxis::new(6)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(7), YAxis::new(6)))),
            // Other pieces
            Box::new(Rook::new(Color::Black, Position::new(XAxis::new(0), YAxis::new(7)))),
            Box::new(Knight::new(Color::Black, Position::new(XAxis::new(1), YAxis::new(7)))),
            Box::new(Bishop::new(Color::Black, Position::new(XAxis::new(2), YAxis::new(7)))),
            Box::new(Queen::new(Color::Black, Position::new(XAxis::new(3), YAxis::new(7)))),
            Box::new(King::new(Color::Black, Position::new(XAxis::new(4), YAxis::new(7)))),
            Box::new(Bishop::new(Color::Black, Position::new(XAxis::new(5), YAxis::new(7)))),
            Box::new(Knight::new(Color::Black, Position::new(XAxis::new(6), YAxis::new(7)))),
            Box::new(Rook::new(Color::Black, Position::new(XAxis::new(7), YAxis::new(7)))),
        ];
        Board { pieces, dimensions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promotion_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![Box::new(Pawn::new(
            Color::White,
            Position::new(XAxis::new(0), YAxis::new(6)),
        ))];
        let board = Board::new(pieces);

        let moves = board
            .get_moves_from(Position::new(0i8.into(), 6i8.into()))
            .unwrap();
        std::dbg!(moves);
    }
}
