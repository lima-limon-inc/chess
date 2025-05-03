use crate::{
    piece::{HorizontalRange, PlusRange, Promoted, VerticalRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, Piece, PieceType, Position,
    Recognizable,
};
use std::collections::HashSet;

// ================================== Rook =====================================
pub struct Rook {
    color: Color,
    position: Position,
}

impl Rook {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Moveset for Rook {
    fn move_to(&mut self, destination: Position) {
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        let vertical_axis = { board.vertical_range(self.get_position(), None) };
        let horizontal_axis = { board.horizontal_range(self.get_position(), None) };

        // This is the maximum possible range from the Rook's position
        let plus_range = PlusRange::from(horizontal_axis, vertical_axis);

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

        let possible_positions = plus_range
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

impl Colored for Rook {
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

impl Recognizable for Rook {
    fn get_type(&self) -> PieceType {
        PieceType::Rook
    }
}

impl CurrentPosition for Rook {
    fn get_position(&self) -> Position {
        self.position
    }
}

impl Piece for Rook {}

impl Promoted for Rook {}
