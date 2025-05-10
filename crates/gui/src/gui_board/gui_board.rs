use raylib::prelude::*;

use std::collections::BTreeMap;
use std::env;

use board::{Board, Color as PieceColor, PieceType};

const TILE_SIZE: i32 = 120;

pub struct GuiBoard {
    board: Board,

    rl: RaylibHandle,

    thread: RaylibThread,

    images: BTreeMap<(PieceType, PieceColor), Texture2D>,
}

impl GuiBoard {
    pub fn init() -> Self {
        let (mut rl, thread) = raylib::init().size(960, 960).title("Hello, World").build();
        let board = Board::default();
        let mut images = BTreeMap::new();

        //TODO use an iterator and fold to make this cleaner. Maybe?
        let b_B = rl.load_texture(&thread, "images/bB.png").unwrap();
        images.insert((PieceType::Bishop, PieceColor::Black), b_B);
        let b_K = rl.load_texture(&thread, "images/bK.png").unwrap();
        images.insert((PieceType::King, PieceColor::Black), b_K);
        let b_N = rl.load_texture(&thread, "images/bN.png").unwrap();
        images.insert((PieceType::Knight, PieceColor::Black), b_N);
        let b_P = rl.load_texture(&thread, "images/bP.png").unwrap();
        images.insert((PieceType::Pawn, PieceColor::Black), b_P);
        let b_Q = rl.load_texture(&thread, "images/bQ.png").unwrap();
        images.insert((PieceType::Queen, PieceColor::Black), b_Q);
        let b_R = rl.load_texture(&thread, "images/bR.png").unwrap();
        images.insert((PieceType::Rook, PieceColor::Black), b_R);
        let w_B = rl.load_texture(&thread, "images/wB.png").unwrap();
        images.insert((PieceType::Bishop, PieceColor::White), w_B);
        let w_K = rl.load_texture(&thread, "images/wK.png").unwrap();
        images.insert((PieceType::King, PieceColor::White), w_K);
        let w_N = rl.load_texture(&thread, "images/wN.png").unwrap();
        images.insert((PieceType::Knight, PieceColor::White), w_N);
        let w_P = rl.load_texture(&thread, "images/wP.png").unwrap();
        images.insert((PieceType::Pawn, PieceColor::White), w_P);
        let w_Q = rl.load_texture(&thread, "images/wQ.png").unwrap();
        images.insert((PieceType::Queen, PieceColor::White), w_Q);
        let w_R = rl.load_texture(&thread, "images/wR.png").unwrap();
        images.insert((PieceType::Rook, PieceColor::White), w_R);

        GuiBoard {
            board,
            rl,
            thread,
            images,
        }
    }
    pub fn start(&mut self) {
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(&self.thread);
            draw_tiles(&mut d);
            draw_pieces(&self.images, &mut d, &self.board);
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

fn draw_pieces(
    images: &BTreeMap<(PieceType, PieceColor), Texture2D>,
    rldraw: &mut RaylibDrawHandle,
    board: &Board,
) {
    let drawable_piece = board
        .get_pieces()
        .map(|piece| (piece, translate_piece(piece.get_type())));

    for (piece, draw) in drawable_piece {
        let x: i32 = piece.get_position().x.0.into();
        let y: i32 = piece.get_position().y.0.into();

        let color = piece.get_color();
        let type_of = piece.get_type();

        let texture = images.get(&(type_of, color)).unwrap();

        rldraw.draw_texture(
            texture,
            // Oh, these numbers? Just corrections
            // TODO: Clean this shit up
            x * TILE_SIZE - 15,
            y * TILE_SIZE - 18,
            Color::WHITE,
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
