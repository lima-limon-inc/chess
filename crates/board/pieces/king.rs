use crate::{
    piece::{PlusRange, StarRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, Piece, PieceType, Position,
    Recognizable,
};

use std::collections::HashSet;
// ================================== King =====================================
pub struct King {
    color: Color,
    position: Position,
    already_moved: bool,
}

impl King {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            already_moved: false,
        }
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

        let vertical_axis = { board.vertical_range(self.get_position(), Some(1), self.color) };
        let horizontal_axis = { board.horizontal_range(self.get_position(), Some(1), self.color) };
        // This is the maximum possible range from the Rook's position
        let plus_range = PlusRange::from(horizontal_axis, vertical_axis);

        let diagonal_range = board.diagonal_range(self.get_position(), Some(1), self.color);

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

        let castling = {
            if self.already_moved == true {
                return Vec::new();
            }

            //TODO: Check if there are no kings
            let rooks: Vec<_> = board
                .find_pieces(Some(PieceType::Rook), Some(self.color))
                // Only get rooks that haven't been moved
                .filter(|rook| rook.was_moved() == false)
                .map(|rook| {
                    let distance = self.get_position().sub_x(rook.get_position().x);
                    let normalized_distance = distance.x.0.abs();
                    let direction: i8 = distance.x.0 / normalized_distance;

                    let rook_position = rook.get_position();

                    let rook_destination = self.get_position().sub_x((1 * direction).into());
                    let king_destination = self.get_position().sub_x((2 * direction).into());

                    Move::new(
                        self.get_position(),
                        king_destination,
                        Some(Effect::Castling {
                            origin: rook_position,
                            destination: rook_destination,
                        }),
                    )
                })
                .collect();

            rooks
        }
        .into_iter();

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
            .chain(castling)
            .collect();
        possible_positions
    }
}

impl Piece for King {
    fn was_moved(&self) -> bool {
        self.already_moved
    }
}
