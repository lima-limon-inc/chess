use crate::{
    piece::{HorizontalRange, PlusRange, StarRange, VerticalRange},
    Board, Color, Colored, CurrentPosition, Effect, Move, Moveset, PieceType, Position,
    Recognizable,
};

use std::collections::HashSet;
// ================================== Pawn =====================================
pub struct Pawn {
    color: Color,
    position: Position,
}
