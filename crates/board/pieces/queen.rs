use crate::{
    piece::{HorizontalRange, PlusRange, VerticalRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, PieceType, Position,
    Recognizable,
};

// ================================= Queen ====================================
pub struct Queen {
    color: Color,
    position: Position,
}

impl Moveset for Queen {
    fn move_to(&mut self, destination: Position) {
        self.position = destination;
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        todo!();
    }
}
