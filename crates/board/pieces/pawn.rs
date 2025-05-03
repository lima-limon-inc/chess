use crate::{
    piece::{
        ChoiceOfPromotablePiece, HorizontalRange, PlusRange, Promotable, Promoted, StarRange,
        VerticalRange,
    },
    pieces::{Bishop, Knight, Queen, Rook},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, Piece, PieceType, Position,
    Recognizable, XAxis, YAxis,
};

use std::collections::HashSet;
// ================================== Pawn =====================================
pub struct Pawn {
    color: Color,
    position: Position,
    already_moved: bool,
}

impl Pawn {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            color,
            position,
            already_moved: false,
        }
    }
}

// TODO: Make this a macro
impl CurrentPosition for Pawn {
    fn get_current_position(&self) -> Position {
        self.position
    }
}

// TODO: Make this a macro
impl Recognizable for Pawn {
    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }
}

// TODO: Make this a macro
impl Colored for Pawn {
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

impl Moveset for Pawn {
    fn move_to(&mut self, destination: Position) {
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        // Black Pawns move "up" in the negative y position
        let move_up = if self.color == Color::Black {
            self.position.sub_y(YAxis::new(1))
        // White Pawns move "up" in the positive y position
        } else {
            self.position.add_y(YAxis::new(1))
        };
        let possible_move_positions = vec![move_up];

        let occupied_positions: HashSet<_> = board
            .get_pieces()
            .map(|piece| piece.get_current_position())
            .collect();

        let possible_move_positions = possible_move_positions
            .into_iter()
            .filter(|position| !occupied_positions.contains(position))
            .map(|position| Move::new(position, None));

        let enemy_possition: HashSet<_> = board
            .get_pieces()
            .into_iter()
            .filter(|piece| piece.get_color() != self.color)
            .map(|piece| piece.get_current_position())
            .collect();

        let possible_attack_positions = vec![
            Position::new(self.get_current_position().x - XAxis::new(1), move_up.y),
            Position::new(self.get_current_position().x + XAxis::new(1), move_up.y),
        ]
        .into_iter()
        // Remove moves outside the board
        .filter(|pos| board.is_inside(pos))
        // Remove attack moves that aren't attacking
        // TODO: Add en passant
        // TODO: Add initial double move
        .filter(|pos| enemy_possition.contains(pos))
        .map(|pos| Move::new(pos, Some(Effect::Capture)));

        let possible_move_positions: Vec<Move> = possible_move_positions
            .chain(possible_attack_positions)
            .collect();

        possible_move_positions
    }
}

impl Piece for Pawn {}

impl Promotable for Pawn {
    fn from(self, choice: ChoiceOfPromotablePiece) -> Box<dyn Promoted> {
        match choice {
            ChoiceOfPromotablePiece::Bishop => Box::new(Bishop::new(self.color, self.position)),
            ChoiceOfPromotablePiece::Knight => Box::new(Knight::new(self.color, self.position)),
            ChoiceOfPromotablePiece::Queen => Box::new(Queen::new(self.color, self.position)),
            ChoiceOfPromotablePiece::Rook => Box::new(Rook::new(self.color, self.position)),
        }
    }
}
