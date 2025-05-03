use crate::{
    piece::{HorizontalRange, PlusRange, StarRange, VerticalRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, PieceType, Position,
    Recognizable,
};
use std::collections::HashSet;

// ================================= Queen ====================================
pub struct Queen {
    color: Color,
    position: Position,
}

// TODO: Make this a macro
impl CurrentPosition for Queen {
    fn get_current_position(&self) -> Position {
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
        let (bl, br, ul, ur) = board.get_limits();

        let vertical_axis = {
            let upper = Position::new(self.position.x, ul.0.y);
            let bottom = Position::new(self.position.x, bl.0.y);

            Position::vertical_range(&bottom, &upper)
        };
        let horizontal_axis = {
            let left = Position::new(bl.0.x, self.position.y);
            let right = Position::new(ur.0.x, self.position.y);
            Position::horizontal_range(&left, &right)
        };

        // This is the maximum possible range from the Rook's position
        let plus_range = PlusRange::from(horizontal_axis, vertical_axis);
        let diagonal_range = Position::diagonal_range(self.get_current_position(), bl, br, ul, ur);
        let max_range = StarRange::from(diagonal_range, plus_range);

        let (teammates, opponents): (Vec<_>, Vec<_>) = board
            .get_pieces()
            .partition(|piece| piece.get_color() == self.color);
        let teammates: HashSet<_> = teammates
            .into_iter()
            .map(|piece| piece.get_current_position())
            .collect();
        let opponents: HashSet<_> = opponents
            .into_iter()
            .map(|piece| piece.get_current_position())
            .collect();

        let possible_positions = max_range
            .0
            .into_iter()
            // You can't move to a position where teammates are standing
            .filter(|possible_position| teammates.contains(possible_position))
            .map(|possible_position| {
                // If you move to a Possition with an opponent, it has a side effect
                if opponents.contains(&possible_position) {
                    Move::new(possible_position, Some(Effect::Capture))
                } else {
                    Move::new(possible_position, None)
                }
            })
            .collect();
        possible_positions
    }
}
