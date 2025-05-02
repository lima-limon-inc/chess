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

pub struct HorizontalRange(Vec<Position>);
pub struct VerticalRange(Vec<Position>);
/// Range that is in the form of a plus symbol +
pub struct PlusRange(Vec<Position>);

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
// ================================== Rook =====================================
pub struct Rook {
    pub color: Color,
    pub position: Position,
    pub piece_type: PieceType,
}

impl Moveset for Rook {
    fn move_to(&self, destination: Position) {
        todo!()
    }

    fn available_positions(&self, board: &Board) -> Vec<Move> {
        let (bl, _, ul, ur) = board.get_limits();

        let vertical_axis = {
            let upper = Position::new(self.position.x, ul.0.y);
            let bottom = Position::new(self.position.x, bl.0.y);

            Position::vertical_range(&bottom, &upper)
        };
        let horizontal_axis = {
            let left = Position::new(bl.0.x, self.position.y);
            let right = Position::new(ur.0.x, self.position.y);
            Position::horizontal_range(&left, &right)
        };

        // This is the maximum possible range from the Rook's position
        let plus_range = PlusRange::from(horizontal_axis, vertical_axis);

        let (teammates, opponents): (Vec<_>, Vec<_>) = board
            .get_pieces()
            .partition(|piece| piece.get_color() == self.color);
        let teammates: HashSet<_> = teammates
            .into_iter()
            .map(|piece| piece.get_current_position())
            .collect();
        let opponents: HashSet<_> = opponents
            .into_iter()
            .map(|piece| piece.get_current_position())
            .collect();

        let possible_positions = plus_range
            .0
            .into_iter()
            // You can't move to a position where teammates are standing
            .filter(|possible_position| teammates.contains(possible_position))
            .map(|possible_position| {
                // If you move to a Possition with an opponent, it has a side effect
                if opponents.contains(&possible_position) {
                    Move::new(possible_position, Some(Effect::Capture))
                } else {
                    Move::new(possible_position, None)
                }
            })
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
        self.piece_type.clone()
    }
}

impl CurrentPosition for Rook {
    fn get_current_position(&self) -> Position {
        self.position
    }
}
