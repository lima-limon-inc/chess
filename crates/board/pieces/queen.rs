use crate::{
    piece::{Piece, PlusRange, StarRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, PieceType, Position,
    Recognizable,
};
use std::collections::HashSet;

// ================================= Queen ====================================
pub struct Queen {
    color: Color,
    position: Position,
}

impl Queen {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

// TODO: Make this a macro
impl CurrentPosition for Queen {
    fn get_position(&self) -> Position {
        self.position
    }
}

// TODO: Make this a macro
impl Recognizable for Queen {
    fn get_type(&self) -> PieceType {
        PieceType::Queen
    }
}

// TODO: Make this a macro
impl Colored for Queen {
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

impl Moveset for Queen {
    fn move_to(&mut self, destination: Position) {
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        let vertical_axis = { board.vertical_range(self.get_position(), None, self.color) };
        let horizontal_axis = { board.horizontal_range(self.get_position(), None, self.color) };

        // This is the maximum possible range from the Rook's position
        let plus_range = PlusRange::from(horizontal_axis, vertical_axis);
        let diagonal_range = board.diagonal_range(self.get_position(), None, self.color);
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

impl Piece for Queen {}
