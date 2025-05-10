use crate::{
    piece::PlusRange, Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, Piece,
    PieceType, Position, Recognizable,
};
use std::collections::HashSet;

// ================================== Rook =====================================
pub struct Rook {
    color: Color,
    position: Position,
    already_moved: bool,
}

impl Rook {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            already_moved: false,
        }
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

        let castling = {
            if self.already_moved == true {
                return Vec::new();
            }

            //TODO: Check if there are no kings
            let king = board
                .find_pieces(Some(PieceType::King), Some(self.color))
                .nth(0)
                .expect("More than one king in the board");

            if king.was_moved() == true {
                return Vec::new();
            }
            let distance = king.get_position().sub_x(self.get_position().x);
            let normalized_distance = distance.x.0.abs();
            let direction: i8 = distance.x.0 / normalized_distance;

            let king_position = self.get_position().add_x(distance.x);

            let rook_destination = king_position.sub_x((1 * direction).into());
            let king_destination = king_position.sub_x((2 * direction).into());

            let rook_move = Move::new(
                self.get_position(),
                rook_destination,
                Some(Effect::Castling {
                    origin: king.get_position(),
                    destination: king_destination,
                }),
            );

            vec![rook_move]
        }
        .into_iter();

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
            .chain(castling)
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

impl Piece for Rook {
    fn was_moved(&self) -> bool {
        self.already_moved
    }
}
