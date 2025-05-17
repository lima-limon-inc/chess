use raylib::prelude::*;

use std::collections::BTreeMap;
use std::env;

use board::{Board, Color as PieceColor, Effect, Move, PieceType, Position};

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
        let b_b = rl.load_texture(&thread, "images/bB.png").unwrap();
        images.insert((PieceType::Bishop, PieceColor::Black), b_b);
        let b_k = rl.load_texture(&thread, "images/bK.png").unwrap();
        images.insert((PieceType::King, PieceColor::Black), b_k);
        let b_n = rl.load_texture(&thread, "images/bN.png").unwrap();
        images.insert((PieceType::Knight, PieceColor::Black), b_n);
        let b_p = rl.load_texture(&thread, "images/bP.png").unwrap();
        images.insert((PieceType::Pawn, PieceColor::Black), b_p);
        let b_q = rl.load_texture(&thread, "images/bQ.png").unwrap();
        images.insert((PieceType::Queen, PieceColor::Black), b_q);
        let b_r = rl.load_texture(&thread, "images/bR.png").unwrap();
        images.insert((PieceType::Rook, PieceColor::Black), b_r);
        let w_b = rl.load_texture(&thread, "images/wB.png").unwrap();
        images.insert((PieceType::Bishop, PieceColor::White), w_b);
        let w_k = rl.load_texture(&thread, "images/wK.png").unwrap();
        images.insert((PieceType::King, PieceColor::White), w_k);
        let w_n = rl.load_texture(&thread, "images/wN.png").unwrap();
        images.insert((PieceType::Knight, PieceColor::White), w_n);
        let w_p = rl.load_texture(&thread, "images/wP.png").unwrap();
        images.insert((PieceType::Pawn, PieceColor::White), w_p);
        let w_q = rl.load_texture(&thread, "images/wQ.png").unwrap();
        images.insert((PieceType::Queen, PieceColor::White), w_q);
        let w_r = rl.load_texture(&thread, "images/wR.png").unwrap();
        images.insert((PieceType::Rook, PieceColor::White), w_r);

        GuiBoard {
            board,
            rl,
            thread,
            images,
        }
    }
    pub fn start(&mut self) {
        let mut available_moves: Option<Vec<Move>> = None;

        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(&self.thread);

            draw_tiles(&mut d);
            if let Some(ref moves) = available_moves {
                draw_moves(&mut d, moves);
            }
            draw_pieces(&self.images, &mut d, &self.board);

            if let Some(position) = get_clicked_tile(&d) {
                if let Some(ref moves) = available_moves {
                    let desired_move = moves.iter().find(|mov| mov.destination == position);
                    if let Some(mov) = desired_move {
                        self.board.execute_move(*mov);
                        available_moves = None;
                    } else {
                        available_moves = self.board.get_moves_from(position);
                    };
                } else {
                    available_moves = self.board.get_moves_from(position);
                }
            }
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
    let drawable_piece = board.get_pieces();

    for piece in drawable_piece {
        let x: i32 = piece.get_position().x.0.into();
        let y = piece.get_position().y.0;
        let y: i32 = (7 - y).into();

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

fn get_clicked_tile(b: &RaylibDrawHandle) -> Option<Position> {
    let was_click = b.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
    if was_click == false {
        return None;
    };

    let mouse_position = b.get_mouse_position();
    // TODO: Add constant
    let x_tile = (mouse_position.x / 120.0).floor();
    let y_tile = (mouse_position.y / 120.0).floor();

    // TODO: Make this a function from-gui to board
    let x_tile: i8 = x_tile as i8;
    let y_tile: i8 = 7 - y_tile as i8;

    Some(Position::new(x_tile.into(), y_tile.into()))
}

// /// Get representation of piece
fn translate_piece(piece_id: PieceType) -> String {
    // ♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟
    match piece_id {
        PieceType::Bishop => "b".into(),
        _ => "a".into(),
    }
}

fn draw_moves(rldraw: &mut RaylibDrawHandle, moves: &Vec<Move>) {
    for mov in moves {
        let destination = mov.destination;
        let x: i32 = destination.x.0.into();
        let y: i32 = destination.y.0.into();
        let y = 7 - y;

        let color = match mov.effect {
            None => Color::GREEN,
            Some(Effect::Capture) => Color::RED,
            Some(Effect::Castling { .. }) => Color::YELLOW,
            Some(Effect::Promotion(_)) => Color::BLUE,
        };

        rldraw.draw_rectangle(TILE_SIZE * x, TILE_SIZE * y, TILE_SIZE, TILE_SIZE, color);
    }
}
