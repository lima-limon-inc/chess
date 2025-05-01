use crate::board::Board;

pub enum Piece {
    Bishop(Bishop),
    King(King),
    Knight(Knight),
    Pawn(Pawn),
    Queen(Queen),
    Rook(Rook),
}
enum PromotablePiece {
    Bishop(Bishop),
    Knight(Knight),
    Queen(Queen),
    Rook(Rook),
}
enum ChoiseOfPromotablePiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}

pub enum Color {
    Black,
    White,
}

pub struct XAxis(u8);
impl XAxis {
    pub fn new(x: u8) -> Self {
        XAxis(x)
    }
}
pub struct YAxis(u8);
impl YAxis {
    pub fn new(x: u8) -> Self {
        YAxis(x)
    }
}

pub struct Position {
    pub x: XAxis,
    pub y: YAxis,
}

enum Effect {
    /// Capture a piece in the board
    Capture,
}

/// This represent a move done by a piece. This means
pub struct Move {
    destination: Position,
    effect: Option<Effect>,
}
// ================================= Bishop ====================================
struct Bishop {
    color: Color,
    position: Position,
}
// ================================== King =====================================
struct King {
    color: Color,
    position: Position,
}
// ================================= Knight ====================================
struct Knight {
    color: Color,
    position: Position,
}
// ================================== Pawn =====================================
struct Pawn {
    color: Color,
    position: Position,
}
impl Pawn {
    fn promote(self, to: ChoiseOfPromotablePiece) -> PromotablePiece {
        match to {
            ChoiseOfPromotablePiece::Bishop => PromotablePiece::Bishop(Bishop {
                color: self.color,
                position: self.position,
            }),
            ChoiseOfPromotablePiece::Knight => PromotablePiece::Knight(Knight {
                color: self.color,
                position: self.position,
            }),
            ChoiseOfPromotablePiece::Queen => PromotablePiece::Queen(Queen {
                color: self.color,
                position: self.position,
            }),
            ChoiseOfPromotablePiece::Rook => PromotablePiece::Rook(Rook {
                color: self.color,
                position: self.position,
            }),
        }
    }
}
// ================================= Queen ====================================
struct Queen {
    color: Color,
    position: Position,
}
// ================================== Rook =====================================
pub struct Rook {
    pub color: Color,
    pub position: Position,
}

impl Moveset for Rook {
    fn move_to(&self, destination: Position) {
        todo!()
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        let pieces = board.get_pieces();
        pieces.filter(|a| a == &&Piece::Bishop(Bishop))
    }
}

pub trait Moveset {
    fn move_to(&self, destination: Position);

    fn available_positions(&self, board: &Board) -> Vec<Move>;
}
