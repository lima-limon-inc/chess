use crate::piece::{
    ChoiceOfPromotablePiece, Color, DiagonalRange, HorizontalRange, Move, Piece, PieceType,
    Position, VerticalRange, XAxis, YAxis,
};
use crate::pieces::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::Effect;
use crate::{BottomLeft, BottomRight, UpperLeft, UpperRight};

use std::collections::HashSet;
pub struct Board {
    pieces: Vec<Box<dyn Piece>>,
    dimensions: (XAxis, YAxis),
    turn: Color,
}

impl Board {
    #[allow(dead_code)]
    /// Only intended for testing
    /// TODO: Add a "fromstr" new where you draw the board with a string
    fn new(pieces: Vec<Box<dyn Piece>>) -> Self {
        let dimensions = (XAxis::new(7), YAxis::new(7));

        Board {
            pieces,
            dimensions,
            turn: Color::White,
        }
    }

    pub fn get_turn(&self) -> Color {
        self.turn
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
            x: self.dimensions.0,
            y: YAxis::new(0i8),
        });
        let ul = UpperLeft(Position {
            x: XAxis::new(0i8),
            y: self.dimensions.1,
        });
        let ur = UpperRight(Position {
            x: self.dimensions.0,
            y: self.dimensions.1,
        });

        debug_assert!(ur.0.y == ul.0.y);
        debug_assert!(ur.0.x == br.0.x);
        debug_assert!(ul.0.x == bl.0.x);
        debug_assert!(bl.0.y == br.0.y);

        (bl, br, ul, ur)
    }

    pub fn is_inside(&self, pos: &Position) -> bool {
        let (bl, br, ul, _) = self.get_limits();

        let within_x = bl.0.x <= pos.x && pos.x <= br.0.x;
        let within_y = bl.0.y <= pos.y && pos.y <= ul.0.y;

        within_x && within_y
    }

    pub fn horizontal_range(
        &self,
        origin: Position,
        limit: Option<u8>,
        friendly: Color,
    ) -> HorizontalRange {
        let (teammates, opponents): (Vec<_>, Vec<_>) = self
            .get_pieces()
            .partition(|piece| piece.get_color() == friendly);
        let teammates: HashSet<_> = teammates
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();
        let opponents: HashSet<_> = opponents
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();

        let (_, _, _, ur) = self.get_limits();

        let up: Vec<_> = {
            let mut left = Vec::new();
            let mut current = origin;
            loop {
                current.y += 1.into();

                if let Some(limit) = limit {
                    if left.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    left.push(current);
                    break;
                }

                left.push(current);
            }
            left
        };

        let down: Vec<_> = {
            let mut left = Vec::new();
            let mut current = origin;
            loop {
                current.y -= 1.into();

                if let Some(limit) = limit {
                    if left.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    left.push(current);
                    break;
                }

                left.push(current);
            }
            left
        };

        HorizontalRange(up.into_iter().chain(down).collect())
    }

    pub fn vertical_range(
        &self,
        origin: Position,
        limit: Option<u8>,
        friendly: Color,
    ) -> VerticalRange {
        let (teammates, opponents): (Vec<_>, Vec<_>) = self
            .get_pieces()
            .partition(|piece| piece.get_color() == friendly);
        let teammates: HashSet<_> = teammates
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();
        let opponents: HashSet<_> = opponents
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();

        let (bl, br, _, _) = self.get_limits();

        let left: Vec<_> = {
            let mut left = Vec::new();
            let mut current = origin;
            loop {
                current.x -= 1.into();

                if let Some(limit) = limit {
                    if left.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    left.push(current);
                    break;
                }

                left.push(current);
            }
            left
        };

        let right: Vec<_> = {
            let mut left = Vec::new();
            let mut current = origin;
            loop {
                current.x += 1.into();

                if let Some(limit) = limit {
                    if left.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    left.push(current);
                    break;
                }

                left.push(current);
            }
            left
        };

        VerticalRange(left.into_iter().chain(right).collect())
    }

    pub fn diagonal_range(
        &self,
        origin: Position,
        limit: Option<u8>,
        friendly: Color,
    ) -> DiagonalRange {
        // We do each diagonal line
        //         1\   /2
        //           \ /
        //            o
        //           / \
        //         3/   \4
        let (teammates, opponents): (Vec<_>, Vec<_>) = self
            .get_pieces()
            .partition(|piece| piece.get_color() == friendly);
        let teammates: HashSet<_> = teammates
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();
        let opponents: HashSet<_> = opponents
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();

        let diagonals = Vec::new();

        let center_to_ul = {
            let mut upper_left = Vec::new();
            let mut current = origin;

            loop {
                current.x -= 1.into();
                current.y += 1.into();

                if let Some(limit) = limit {
                    if upper_left.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    upper_left.push(current);
                    break;
                }

                upper_left.push(current);
            }
            upper_left
        };

        let center_to_ur = {
            let mut upper_right = Vec::new();
            let mut current = origin;

            loop {
                current.x += 1.into();
                current.y += 1.into();

                if let Some(limit) = limit {
                    if upper_right.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    upper_right.push(current);
                    break;
                }

                upper_right.push(current);
            }
            upper_right
        };
        let center_to_bl = {
            let mut bottom_left = Vec::new();
            let mut current = origin;

            loop {
                current.x -= 1.into();
                current.y -= 1.into();

                if let Some(limit) = limit {
                    if bottom_left.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    bottom_left.push(current);
                    break;
                }

                bottom_left.push(current);
            }
            bottom_left
        };
        let center_to_br = {
            let mut bottom_right = Vec::new();
            let mut current = origin;

            while self.is_inside(&current) {
                current.x += 1.into();
                current.y -= 1.into();

                if let Some(limit) = limit {
                    if bottom_right.len() >= limit.into() {
                        break;
                    }
                };

                if self.is_inside(&current) == false {
                    break;
                };

                if teammates.contains(&current) {
                    break;
                } else if opponents.contains(&current) {
                    bottom_right.push(current);
                    break;
                }

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
            .filter(|piece| piece.get_color() == self.get_turn())
            .map(|piece| piece.available_positions(self))
    }

    pub fn execute_move(&mut self, mov: Move) {
        // TODO: Remove unwrap(s)

        // TODO: Do this in one step; i'm doing it in two because of compilaton error
        let index = self
            .pieces
            .iter()
            .position(|piece| piece.get_position() == mov.origin)
            .unwrap();

        let enemy_piece = self
            .pieces
            .iter()
            .position(|piece| piece.get_position() == mov.destination);

        let piece: &mut Box<dyn Piece> = self.pieces.get_mut(index).unwrap();

        // TODO: Add error
        let piece_color = piece.get_color();
        if piece_color != self.turn {
            panic!()
        }

        piece.move_to(mov.destination);

        match mov.effect {
            Some(Effect::Capture) => {
                let enemy_piece = enemy_piece.expect(
                    "Tried to capture an enemy at position, but there is no enemy at that position",
                );
                self.pieces.swap_remove(enemy_piece);
            }
            Some(Effect::Castling {
                origin,
                destination,
            }) => {
                let other_piece_index = self
                    .pieces
                    .iter()
                    .position(|piece| piece.get_position() == origin)
                    .unwrap();

                let other_piece: &mut Box<dyn Piece> =
                    self.pieces.get_mut(other_piece_index).unwrap();

                other_piece.move_to(destination);
            }
            Some(Effect::Promotion(choice)) => {
                if let Some(choice) = choice {
                    let promoted_piece = Board::promote_piece(choice, piece.as_ref());
                    *piece = promoted_piece;
                } else {
                    panic!("Tried to promote piece but no piece was specified");
                }
            }
            None => (),
        }

        self.turn = !self.turn;
    }
    // TODO: Pub crate instead of pub
    pub fn capture_piece(&mut self, pos: Position) {
        // Retain pieces that have a different position
        self.pieces.retain(|piece| piece.get_position() != pos);
    }

    fn promote_piece(choice: ChoiceOfPromotablePiece, original: &dyn Piece) -> Box<dyn Piece> {
        let position = original.get_position();
        let color = original.get_color();
        match choice {
            ChoiceOfPromotablePiece::Bishop => Box::new(Bishop::new(color, position)),
            ChoiceOfPromotablePiece::Knight => Box::new(Knight::new(color, position)),
            ChoiceOfPromotablePiece::Queen => Box::new(Queen::new(color, position)),
            ChoiceOfPromotablePiece::Rook => Box::new(Rook::new(color, position)),
        }
    }

    // TODO: Use this function for the other functions for the differnet pieces.
    pub fn find_pieces(
        &self,
        piece_type_filter: Option<PieceType>,
        color_filter: Option<Color>,
    ) -> impl Iterator<Item = &Box<dyn Piece>> {
        self.get_pieces()
            .filter(move |piece| {
                if let Some(piece_type_filter) = &piece_type_filter {
                    piece.get_type() == *piece_type_filter
                } else {
                    true
                }
            })
            .filter(move |piece| {
                if let Some(color_filter) = color_filter {
                    piece.get_color() == color_filter
                } else {
                    true
                }
            })
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
        Board {
            pieces,
            dimensions,
            turn: Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PieceType;

    #[test]
    fn promotion_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![Box::new(Pawn::new(
            Color::White,
            Position::new(XAxis::new(0), YAxis::new(6)),
        ))];
        let mut board = Board::new(pieces);

        let mut mov = board
            .get_moves_from(Position::new(0i8.into(), 6i8.into()))
            .unwrap()
            .into_iter()
            .nth(0)
            .unwrap();

        mov.effect = Some(Effect::Promotion(Some(ChoiceOfPromotablePiece::Queen)));

        board.execute_move(mov);

        let result = board.get_pieces().nth(0).unwrap();
        assert_eq!(result.get_color(), Color::White);
        assert_eq!(result.get_position(), Position::new(0.into(), 7.into()));
        assert_eq!(result.get_type(), PieceType::Queen);
    }

    #[test]
    fn capture_test() {
        #[rustfmt::skip]
        let pieces: Vec<Box<dyn Piece>> = vec![
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(0), YAxis::new(1)))),
            Box::new(Pawn::new(Color::Black, Position::new(XAxis::new(1), YAxis::new(2)))),
        ];
        let mut board = Board::new(pieces);

        let pieces: Vec<_> = board.get_pieces().collect();
        assert_eq!(pieces.len(), 2);

        let mov = board
            .get_moves_from(Position::new(0i8.into(), 1i8.into()))
            .unwrap()
            .into_iter()
            .filter(|mov| mov.effect.is_some())
            .nth(0)
            .unwrap();

        board.execute_move(mov);

        let result: Vec<_> = board.get_pieces().collect();
        assert_eq!(result.len(), 1);

        let result = board.get_pieces().nth(0).unwrap();
        assert_eq!(result.get_color(), Color::White);
        assert_eq!(result.get_position(), Position::new(1.into(), 2.into()));
        assert_eq!(result.get_type(), PieceType::Pawn);
    }

    #[test]
    fn castling_king_side_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![
            Box::new(Rook::new(
                Color::White,
                Position::new(XAxis::new(7), YAxis::new(0)),
            )),
            Box::new(King::new(
                Color::White,
                Position::new(XAxis::new(4), YAxis::new(0)),
            )),
        ];
        let mut board = Board::new(pieces);
        let mov = board
            .get_moves_from(Position::new(7i8.into(), 0i8.into()))
            .unwrap()
            .into_iter()
            .filter(|mov| mov.effect.is_some())
            .nth(0)
            .unwrap();

        board.execute_move(mov);

        let result: Vec<_> = board.get_pieces().collect();
        assert_eq!(result.len(), 2);

        let king = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::King)
            .nth(0)
            .unwrap();

        assert_eq!(king.get_color(), Color::White);
        assert_eq!(king.get_position(), Position::new(6.into(), 0.into()));

        let rook = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::Rook)
            .nth(0)
            .unwrap();

        assert_eq!(rook.get_color(), Color::White);
        assert_eq!(rook.get_position(), Position::new(5.into(), 0.into()));
    }

    #[test]
    fn castling_queen_side_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![
            Box::new(Rook::new(
                Color::White,
                Position::new(XAxis::new(0), YAxis::new(0)),
            )),
            Box::new(King::new(
                Color::White,
                Position::new(XAxis::new(4), YAxis::new(0)),
            )),
        ];
        let mut board = Board::new(pieces);
        let mov = board
            .get_moves_from(Position::new(0i8.into(), 0i8.into()))
            .unwrap()
            .into_iter()
            .filter(|mov| mov.effect.is_some())
            .nth(0)
            .unwrap();

        board.execute_move(mov);

        let result: Vec<_> = board.get_pieces().collect();
        assert_eq!(result.len(), 2);

        let king = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::King)
            .nth(0)
            .unwrap();

        assert_eq!(king.get_color(), Color::White);
        assert_eq!(king.get_position(), Position::new(2.into(), 0.into()));

        let rook = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::Rook)
            .nth(0)
            .unwrap();

        assert_eq!(rook.get_color(), Color::White);
        assert_eq!(rook.get_position(), Position::new(3.into(), 0.into()));
    }

    #[test]
    fn castling_queen_side_from_king_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![
            Box::new(Rook::new(
                Color::White,
                Position::new(XAxis::new(0), YAxis::new(0)),
            )),
            Box::new(Rook::new(
                Color::White,
                Position::new(XAxis::new(7), YAxis::new(0)),
            )),
            Box::new(King::new(
                Color::White,
                Position::new(XAxis::new(4), YAxis::new(0)),
            )),
        ];
        let mut board = Board::new(pieces);
        let moves: Vec<_> = board
            .get_moves_from(Position::new(4i8.into(), 0i8.into()))
            .unwrap()
            .into_iter()
            .filter(|mov| mov.effect.is_some())
            .collect();

        assert_eq!(moves.len(), 2);

        let queen_rook_move = moves
            .into_iter()
            .filter(|mov| match mov.effect {
                Some(Effect::Castling {
                    origin,
                    destination,
                }) => {
                    let queen = Position::new(0.into(), 0.into());
                    matches!(origin, queen)
                }
                _ => false,
            })
            .nth(0)
            .unwrap();

        board.execute_move(queen_rook_move);

        let result: Vec<_> = board.get_pieces().collect();
        assert_eq!(result.len(), 3);

        let king = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::King)
            .nth(0)
            .unwrap();

        assert_eq!(king.get_color(), Color::White);
        assert_eq!(king.get_position(), Position::new(2.into(), 0.into()));

        let rook = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::Rook)
            .nth(0)
            .unwrap();

        assert_eq!(rook.get_color(), Color::White);
        assert_eq!(rook.get_position(), Position::new(3.into(), 0.into()));
    }

    #[test]
    fn castling_king_side_from_king_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![
            Box::new(Rook::new(
                Color::White,
                Position::new(XAxis::new(0), YAxis::new(0)),
            )),
            Box::new(Rook::new(
                Color::White,
                Position::new(XAxis::new(7), YAxis::new(0)),
            )),
            Box::new(King::new(
                Color::White,
                Position::new(XAxis::new(4), YAxis::new(0)),
            )),
        ];
        let mut board = Board::new(pieces);
        let moves: Vec<_> = board
            .get_moves_from(Position::new(4i8.into(), 0i8.into()))
            .unwrap()
            .into_iter()
            .filter(|mov| mov.effect.is_some())
            .collect();

        assert_eq!(moves.len(), 2);
        let king_rook_move = moves
            .into_iter()
            .filter(|mov| match mov.effect {
                Some(Effect::Castling {
                    origin,
                    destination,
                }) => {
                    let king_pos = Position::new(7.into(), 0.into());
                    origin == king_pos
                }
                _ => false,
            })
            .nth(0)
            .unwrap();

        board.execute_move(king_rook_move);

        let result: Vec<_> = board.get_pieces().collect();
        assert_eq!(result.len(), 3);

        let king = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::King)
            .nth(0)
            .unwrap();

        assert_eq!(king.get_color(), Color::White);
        assert_eq!(king.get_position(), Position::new(6.into(), 0.into()));

        let rook = board
            .get_pieces()
            .filter(|piece| piece.get_type() == PieceType::Rook)
            // TODO: Remove .nth
            .nth(1)
            .unwrap();

        assert_eq!(rook.get_color(), Color::White);
        assert_eq!(rook.get_position(), Position::new(5.into(), 0.into()));
    }

    #[test]
    fn movements_within_bounds_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![Box::new(Queen::new(
            Color::White,
            Position::new(XAxis::new(0), YAxis::new(0)),
        ))];
        let mut board = Board::new(pieces);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(0i8.into(), 0i8.into()))
            .unwrap();

        for mov in moves {
            let destination = mov.destination;
            assert!(destination.x.0 >= 0);
            assert!(destination.y.0 >= 0);
        }
    }

    #[test]
    fn check_movements_covered_by_friendly_piece_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![
            Box::new(Queen::new(
                Color::White,
                Position::new(XAxis::new(0), YAxis::new(0)),
            )),
            Box::new(Knight::new(
                Color::White,
                Position::new(XAxis::new(1), YAxis::new(0)),
            )),
            Box::new(Pawn::new(
                Color::White,
                Position::new(XAxis::new(0), YAxis::new(1)),
            )),
            Box::new(Pawn::new(
                Color::White,
                Position::new(XAxis::new(1), YAxis::new(1)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(4), YAxis::new(1)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(4), YAxis::new(3)),
            )),
            Box::new(Rook::new(
                Color::White,
                Position::new(XAxis::new(4), YAxis::new(2)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(3), YAxis::new(2)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(5), YAxis::new(2)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(7), YAxis::new(6)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(5), YAxis::new(6)),
            )),
            Box::new(Bishop::new(
                Color::White,
                Position::new(XAxis::new(6), YAxis::new(5)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(7), YAxis::new(4)),
            )),
            Box::new(Pawn::new(
                Color::Black,
                Position::new(XAxis::new(5), YAxis::new(4)),
            )),
        ];
        let mut board = Board::new(pieces);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(0i8.into(), 0i8.into()))
            .unwrap();

        // The queen is blocked by its own pieces and can't move
        assert_eq!(moves.len(), 0);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(4i8.into(), 2i8.into()))
            .unwrap();

        // The rook can only move in places where pawns are available
        assert_eq!(moves.len(), 4);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(6i8.into(), 5i8.into()))
            .unwrap();

        // The rook can only move in places where pawns are available
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn check_castling_covered_by_pieces_test() {
        #[rustfmt::skip]
        let pieces: Vec<Box<dyn Piece>> = vec![
            // White pieces
            // Pawns
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(0), YAxis::new(6)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(1), YAxis::new(6)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(2), YAxis::new(6)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(3), YAxis::new(6)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(4), YAxis::new(6)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(5), YAxis::new(6)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(6), YAxis::new(6)))),
            Box::new(Pawn::new(Color::White, Position::new(XAxis::new(7), YAxis::new(6)))),
            // Pawns
            Box::new(Rook::new(Color::White, Position::new(XAxis::new(0), YAxis::new(7)))),
            Box::new(Knight::new(Color::White, Position::new(XAxis::new(1), YAxis::new(7)))),
            Box::new(Bishop::new(Color::White, Position::new(XAxis::new(2), YAxis::new(7)))),
            Box::new(Queen::new(Color::White, Position::new(XAxis::new(3), YAxis::new(7)))),
            Box::new(King::new(Color::White, Position::new(XAxis::new(4), YAxis::new(7)))),
            Box::new(Bishop::new(Color::White, Position::new(XAxis::new(5), YAxis::new(7)))),
            Box::new(Knight::new(Color::White, Position::new(XAxis::new(6), YAxis::new(7)))),
            Box::new(Rook::new(Color::White, Position::new(XAxis::new(7), YAxis::new(7)))),
        ];

        let mut board = Board::new(pieces);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(4i8.into(), 7i8.into()))
            .unwrap();

        // The king can't castle when the positions are covered by other pieces
        assert_eq!(moves.len(), 0);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(0i8.into(), 7i8.into()))
            .unwrap();

        // The left rook can't either
        assert_eq!(moves.len(), 0);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(7i8.into(), 7i8.into()))
            .unwrap();

        // The right rook can't either
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn pawn_initial_2_positions_test() {
        let pieces: Vec<Box<dyn Piece>> = vec![Box::new(Pawn::new(
            Color::White,
            Position::new(XAxis::new(0), YAxis::new(1)),
        ))];
        let mut board = Board::new(pieces);

        let moves: Vec<_> = board
            .get_moves_from(Position::new(0i8.into(), 1i8.into()))
            .unwrap();
        assert_eq!(moves.len(), 2);
    }
}
