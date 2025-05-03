use crate::board::Board;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub struct BottomLeft(pub Position);
pub struct BottomRight(pub Position);
pub struct UpperLeft(pub Position);
pub struct UpperRight(pub Position);

// TODO: Make macro
// TODO: Implement for everyone
impl From<UpperLeft> for Position {
    fn from(value: UpperLeft) -> Self {
        value.0
    }
}

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
pub struct XAxis(pub u8);
impl XAxis {
    pub fn new(x: u8) -> Self {
        XAxis(x)
    }
}
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub struct YAxis(pub u8);
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

impl Sub for YAxis {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        YAxis::new(self.0 - other.0)
    }
}

impl Add for YAxis {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        YAxis::new(self.0 + other.0)
    }
}

impl SubAssign for YAxis {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0 - other.0;
    }
}

impl AddAssign for XAxis {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0 + other.0;
    }
}

impl SubAssign for XAxis {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0 - other.0;
    }
}

impl Sub for XAxis {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        XAxis::new(self.0 - other.0)
    }
}

impl Add for XAxis {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        XAxis::new(self.0 + other.0)
    }
}

impl From<u8> for YAxis {
    fn from(value: u8) -> Self {
        YAxis::new(value)
    }
}

impl From<u8> for XAxis {
    fn from(value: u8) -> Self {
        XAxis::new(value)
    }
}

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
/// Range that is in the form of a multiplication symbol x
pub struct DiagonalRange(pub Vec<Position>);
/// Range that is in the form of a star, aka PlusRange + DiagonalRange
pub struct StarRange(pub Vec<Position>);

impl PlusRange {
    pub fn from(horizontal: HorizontalRange, vertical: VerticalRange) -> Self {
        let pos = horizontal.0.into_iter().chain(vertical.0).collect();
        PlusRange(pos)
    }
}

impl StarRange {
    pub fn from(diagonal: DiagonalRange, plus: PlusRange) -> Self {
        let pos = diagonal.0.into_iter().chain(plus.0).collect();
        StarRange(pos)
    }
}

impl Position {
    pub fn new(x: XAxis, y: YAxis) -> Self {
        Self { x, y }
    }

    pub fn add_y(self, y_axis: YAxis) -> Self {
        Position::new(self.x, self.y + y_axis)
    }

    pub fn sub_y(self, y_axis: YAxis) -> Self {
        Position::new(self.x, self.y - y_axis)
    }

    pub fn add_x(self, x_axis: XAxis) -> Self {
        Position::new(self.x + x_axis, self.y)
    }

    pub fn sub_x(self, x_axis: XAxis) -> Self {
        Position::new(self.x - x_axis, self.y)
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
    fn move_to(&mut self, destination: Position) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagonal_range() {
        panic!()
    }
}
