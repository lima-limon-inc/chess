use crate::{
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, Piece, PieceType, Position,
    Recognizable, XAxis, YAxis,
};
use std::collections::HashSet;

// ================================= Knight ====================================
pub struct Knight {
    color: Color,
    position: Position,
}

impl Knight {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

// TODO: Make this a macro
impl CurrentPosition for Knight {
    fn get_position(&self) -> Position {
        self.position
    }
}

// TODO: Make this a macro
impl Recognizable for Knight {
    fn get_type(&self) -> PieceType {
        PieceType::Knight
    }
}

// TODO: Make this a macro
impl Colored for Knight {
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

impl Moveset for Knight {
    fn move_to(&mut self, destination: Position) {
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
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

        let possible_positions = vec![
            // First two possitions
            self.position.add_y(YAxis::new(2)).add_x(XAxis::new(1)),
            self.position.add_y(YAxis::new(2)).sub_x(XAxis::new(1)),
            // Second level
            self.position.add_y(YAxis::new(1)).add_x(XAxis::new(2)),
            self.position.add_y(YAxis::new(1)).sub_x(XAxis::new(2)),
            // Third level
            self.position.sub_y(YAxis::new(1)).add_x(XAxis::new(2)),
            self.position.sub_y(YAxis::new(1)).sub_x(XAxis::new(2)),
            // Forth
            self.position.sub_y(YAxis::new(2)).add_x(XAxis::new(1)),
            self.position.sub_y(YAxis::new(2)).sub_x(XAxis::new(1)),
        ]
        .into_iter()
        .filter(|pos| board.is_inside(pos))
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

impl Piece for Knight {}
