pub mod gui_board;
use gui_board::GuiBoard;

fn main() {
    let mut board = GuiBoard::init();
    board.start();
}
