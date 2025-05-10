// use raylib::ffi::Color;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(960, 960).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        draw_tiles(&mut d);
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
            rldraw.draw_rectangle(120 * x, 120 * y, 120, 120, color);
        }
    }
}
