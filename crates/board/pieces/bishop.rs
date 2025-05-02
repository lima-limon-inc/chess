use crate::{
    piece::{HorizontalRange, PlusRange, VerticalRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, Piece, PieceType, Position,
    Recognizable,
};
use std::collections::HashSet;

// ================================= Bishop ====================================
pub struct Bishop {
    color: Color,
    position: Position,
    piece_type: PieceType,
}

// TODO: Make this a macro
impl CurrentPosition for Bishop {
    fn get_current_position(&self) -> Position {
        self.position
    }
}

// TODO: Make this a macro
impl Recognizable for Bishop {
    fn get_type(&self) -> PieceType {
        self.piece_type.clone()
    }
}

// TODO: Make this a macro
impl Colored for Bishop {
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

impl Moveset for Bishop {
    fn move_to(&mut self, destination: Position) {
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        let (bl, _, ul, ur) = board.get_limits();

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

        let possible_positions = plus_range
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

impl Piece for Bishop {}
