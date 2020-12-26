mod board;

use board::{Board, Cell};

fn main() {
    let mut game_board: Board = Board::new();
    /* Oscillator
    game_board.mut_gamestate()[4][5] = Cell::Alive;
    game_board.mut_gamestate()[5][5] = Cell::Alive;
    game_board.mut_gamestate()[6][5] = Cell::Alive;
    */
    /* Glider
    game_board.mut_gamestate()[4][5] = Cell::Alive;
    game_board.mut_gamestate()[5][5] = Cell::Alive;
    game_board.mut_gamestate()[6][5] = Cell::Alive;
    game_board.mut_gamestate()[5][3] = Cell::Alive;
    game_board.mut_gamestate()[6][4] = Cell::Alive;
    */
    game_board.randomize(33);
    println!("{}", game_board);

    loop {
        game_board = game_board.update_board();
        println!("{}", game_board);
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }
}
