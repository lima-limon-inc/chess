use crate::{
    piece::{HorizontalRange, PlusRange, StarRange, VerticalRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, Piece, PieceType, Position,
    Recognizable,
};

use std::collections::HashSet;
// ================================== King =====================================
pub struct King {
    color: Color,
    position: Position,
}

impl King {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

// TODO: Make this a macro
impl CurrentPosition for King {
    fn get_position(&self) -> Position {
        self.position
    }
}

// TODO: Make this a macro
impl Recognizable for King {
    fn get_type(&self) -> PieceType {
        PieceType::King
    }
}

// TODO: Make this a macro
impl Colored for King {
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

impl Moveset for King {
    fn move_to(&mut self, destination: Position) {
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        // This is the maximum possible range from the King's position

        let vertical_axis = { board.vertical_range(self.get_position(), Some(1)) };
        let horizontal_axis = { board.horizontal_range(self.get_position(), Some(1)) };
        // This is the maximum possible range from the Rook's position
        let plus_range = PlusRange::from(horizontal_axis, vertical_axis);

        let diagonal_range = board.diagonal_range(self.get_position(), Some(1));

        let max_range = StarRange::from(diagonal_range, plus_range);

        let (teammates, opponents): (Vec<_>, Vec<_>) = board
            .get_pieces()
            .partition(|piece| piece.get_color() == self.color);
        let teammates: HashSet<_> = teammates
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();
        let opponents: HashSet<_> = opponents
            .into_iter()
            .map(|piece| piece.get_position())
            .collect();

        // TODO Castling
        // TODO Moves that put you in check
        // TODO King is in adjacent square.
        let possible_positions = max_range
            .0
            .into_iter()
            // You can't move to a position where teammates are standing
            .filter(|possible_position| !teammates.contains(possible_position))
            .map(|possible_position| {
                // If you move to a Possition with an opponent, it has a side effect
                if opponents.contains(&possible_position) {
                    Move::new(
                        self.get_position(),
                        possible_position,
                        Some(Effect::Capture),
                    )
                } else {
                    Move::new(self.get_position(), possible_position, None)
                }
            })
            .collect();
        possible_positions
    }
}

impl Piece for King {}
