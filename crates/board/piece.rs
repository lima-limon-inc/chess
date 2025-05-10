use crate::board::Board;
use crate::pieces::Pawn;
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(Debug)]
pub enum ChoiceOfPromotablePiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct XAxis(pub i8);
impl XAxis {
    pub fn new(x: i8) -> Self {
        XAxis(x)
    }
}
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub struct YAxis(pub i8);
impl YAxis {
    pub fn new(x: i8) -> Self {
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

impl From<i8> for YAxis {
    fn from(value: i8) -> Self {
        YAxis::new(value)
    }
}

impl From<i8> for XAxis {
    fn from(value: i8) -> Self {
        XAxis::new(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

#[derive(Debug)]
pub enum Effect {
    /// Capture a piece in the board
    Capture,
    /// A king and a rook castle
    Castling {
        /// Other Piece origin
        origin: Position,
        /// Other Piece destination
        destination: Position,
    },
    /// A pawn is promoted. None means that a choice wasnt made yet
    Promotion(Option<ChoiceOfPromotablePiece>),
}

/// This represent a move done by a piece. This means
#[derive(Debug)]
pub struct Move {
    /// The place where the piece that will execute the move is standing on
    pub origin: Position,
    /// The place where the piece will end up
    pub destination: Position,
    /// Any "side effect" that the move may have
    pub effect: Option<Effect>,
}

impl Move {
    pub fn new(origin: Position, destination: Position, effect: Option<Effect>) -> Self {
        Self {
            origin,
            destination,
            effect,
        }
    }
}

// TODO: Remove all these traits, move to "piece" trait
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

/// A piece that can be promoted
pub trait Promotable: Piece {
    fn from(self, choice: ChoiceOfPromotablePiece) -> Box<dyn Promoted>;
}

/// A piece that can be promoted into
pub trait Promoted: Piece {}

/// A piece that can be promoted from (Pawn)
pub trait CurrentPosition {
    fn get_position(&self) -> Position;
}

/// Great name, huh? It means it returns a variant of PieceType
pub trait Recognizable {
    fn get_type(&self) -> PieceType {
        todo!()
    }
}

/// The piece can execute a Move
pub trait Piece: Colored + Moveset + Recognizable + CurrentPosition {
    fn was_moved(&self) -> bool {
        todo!()
    }
}
// trait Sub: Super {}
// trait Super {}

// fn upcast(x: &dyn Sub) -> &dyn Super {
//     x // implicit coercion
// }
