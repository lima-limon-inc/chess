use crate::board::Board;
use std::collections::HashSet;
use std::ops::AddAssign;

#[derive(Clone)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

enum ChoiseOfPromotablePiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct XAxis(u8);
impl XAxis {
    pub fn new(x: u8) -> Self {
        XAxis(x)
    }
}
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct YAxis(u8);
impl YAxis {
    pub fn new(x: u8) -> Self {
        YAxis(x)
    }
}
impl AddAssign for YAxis {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0;
    }
}

// impl From<u8> for YAxis {
//     fn from(value: u8) -> Self {
//         YAxis::new(value)
//     }
// }

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: XAxis,
    pub y: YAxis,
}

// TODO: Remove pub here
pub struct HorizontalRange(pub Vec<Position>);
pub struct VerticalRange(pub Vec<Position>);
/// Range that is in the form of a plus symbol +
pub struct PlusRange(pub Vec<Position>);

impl PlusRange {
    pub fn from(horizontal: HorizontalRange, vertical: VerticalRange) -> Self {
        let pos = horizontal.0.into_iter().chain(vertical.0).collect();
        PlusRange(pos)
    }
}

impl Position {
    pub fn new(x: XAxis, y: YAxis) -> Self {
        Self { x, y }
    }

    pub fn vertical_range(start: &Position, end: &Position) -> VerticalRange {
        debug_assert!(start.y < end.y);
        let range = (start.y.0..=end.y.0)
            .map(|y| Position::new(start.x, YAxis::new(y)))
            .collect();
        VerticalRange(range)
    }

    pub fn horizontal_range(start: &Position, end: &Position) -> HorizontalRange {
        debug_assert!(start.x < end.x);
        let range = (start.x.0..=end.x.0)
            .map(|x| Position::new(XAxis::new(x), start.y))
            .collect();
        HorizontalRange(range)
    }
}

pub enum Effect {
    /// Capture a piece in the board
    Capture,
}

/// This represent a move done by a piece. This means
pub struct Move {
    destination: Position,
    effect: Option<Effect>,
}

impl Move {
    pub fn new(destination: Position, effect: Option<Effect>) -> Self {
        Self {
            destination,
            effect,
        }
    }
}

pub trait Moveset {
    fn move_to(&self, destination: Position) {
        todo!()
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        todo!()
    }
}

pub trait Colored {
    fn get_color(&self) -> Color {
        todo!()
    }
}

/// A piece that can be promoted into
pub trait Promotable {}

/// A piece that can be promoted from (Pawn)
pub trait Promotee {}

/// A piece that can be promoted from (Pawn)
pub trait CurrentPosition {
    fn get_current_position(&self) -> Position;
}

/// Great name, huh? It means it returns a variant of PieceType
pub trait Recognizable {
    fn get_type(&self) -> PieceType {
        todo!()
    }
}

pub trait Piece: Colored + Moveset + Recognizable + CurrentPosition {}

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
// ================================= Queen ====================================
struct Queen {
    color: Color,
    position: Position,
}
