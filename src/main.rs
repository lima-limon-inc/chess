use board::Color;
use board::Moveset;
use board::Piece;
use board::Position;
use board::Rook;
use board::{XAxis, YAxis};

fn main() {
    let a = Piece::Rook(Rook {
        position: Position {
            x: XAxis::new(7u8),
            y: YAxis::new(9u8),
        },
        color: Color::Black,
    });
    // a.move_to(Position { x: 9, y: 10 });

    println!("Hellgo, world!");
}
