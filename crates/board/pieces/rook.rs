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
        self.already_moved = true;
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        let vertical_axis = { board.vertical_range(self.get_position(), None, self.color) };
        let horizontal_axis = { board.horizontal_range(self.get_position(), None, self.color) };

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

        let castling = 'castle: {
            if self.already_moved == true {
                break 'castle Vec::new();
            }

            //TODO: Tidy this up
            let king: Vec<_> = board
                .find_pieces(Some(PieceType::King), Some(self.color))
                .collect();
            if king.len() == 0 {
                break 'castle Vec::new();
            } else if king.len() > 1 {
                panic!("More than one king in the board");
            };
            let king = king.into_iter().nth(0).unwrap();

            if king.was_moved() == true {
                break 'castle Vec::new();
            }
            let distance = king.get_position().sub_x(self.get_position().x);

            let normalized_distance = distance.x.0.abs();
            let direction: i8 = distance.x.0 / normalized_distance;

            //TODO: Clean this shit up
            // Checks if there are pieces in between the rook and the king
            {
                let mut temp;
                temp = self.get_position();
                while temp.x != direction.into() || temp == king.get_position() {
                    temp.x += direction.into();
                    if temp != king.get_position() && teammates.contains(&temp)
                        || opponents.contains(&temp)
                    {
                        break 'castle Vec::new();
                    }
                }
            };

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
            // .filter(|possible_position| !teammates.contains(possible_position))
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
