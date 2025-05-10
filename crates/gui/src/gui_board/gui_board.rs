// use raylib::ffi::Color;
use raylib::prelude::*;

use board::{Board, Color as PieceColor, PieceType};

const TILE_SIZE: i32 = 120;

pub struct GuiBoard {
    board: Board,

    rl: RaylibHandle,

    thread: RaylibThread,
}

impl GuiBoard {
    pub fn init() -> Self {
        let (mut rl, thread) = raylib::init().size(960, 960).title("Hello, World").build();
        rl.load_texture(&thread, "images/bB.svg");
        let board = Board::default();

        GuiBoard { board, rl, thread }
    }
    pub fn start(&mut self) {
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(&self.thread);
            draw_tiles(&mut d);
            draw_pieces(&mut d, &self.board);
            get_clicked_tile(&d).map(|_| panic!());
        }
    }
}

fn draw_tiles(rldraw: &mut RaylibDrawHandle) {
    for y in 0..=7 {
        for x in 0..=7 {
            // Interesting trick
            let color = if (x + y) % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };
            rldraw.draw_rectangle(TILE_SIZE * x, TILE_SIZE * y, TILE_SIZE, TILE_SIZE, color);
        }
    }
}

fn draw_pieces(rldraw: &mut RaylibDrawHandle, board: &Board) {
    let drawable_piece = board
        .get_pieces()
        .map(|piece| (piece, translate_piece(piece.get_type())));

    for (piece, draw) in drawable_piece {
        let x: i32 = piece.get_position().x.0.into();
        let y: i32 = piece.get_position().y.0.into();
        rldraw.draw_text(
            draw.as_str(),
            (x * TILE_SIZE).into(),
            (y * TILE_SIZE).into(),
            50,
            Color::RED,
        );
    }
}

fn get_clicked_tile(b: &RaylibDrawHandle) -> Option<Vector2> {
    let was_click = b.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    if was_click == false {
        return None;
    };

    let mouse_position = b.get_mouse_position();
    // TODO: Add constant
    let x_tile = (mouse_position.x / 120.0).floor();
    let y_tile = 7.0 - (mouse_position.y / 120.0).floor();

    Some(Vector2::new(x_tile, y_tile))
}

// /// Get representation of piece
fn translate_piece(piece_id: PieceType) -> String {
    // ♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟
    match piece_id {
        PieceType::Bishop => "b".into(),
        _ => "a".into(),
    }
}
