use std::fmt;
use rand::{thread_rng, Rng};

/// Fixed board size of 10 in both dimensions
pub const BOARD_SIZE: usize = 10;

/// Struct representing the game board
pub struct Board {
    /// Board representation as a BOARD_SIZExBOARD_SIZE Cell Array, filled with dead cells on initialisation
    gamestate: [[Cell; BOARD_SIZE] ; BOARD_SIZE],
}

impl Board {
    /// Construct a new Board consisting of a BOARD_SIZExBOARD_SIZE two-dimensional array
    pub fn new() -> Board {
        Board {
            gamestate: [[Cell::Dead; BOARD_SIZE] ; BOARD_SIZE]
        }
    }

    /// Immutably returns the Board's current gamestate
    pub fn gamestate(&self) -> &[[Cell; BOARD_SIZE] ; BOARD_SIZE] {
        &self.gamestate
    }

    /// Mutably returns the Board's current gamestate
    pub fn mut_gamestate(&mut self) -> &mut [[Cell; BOARD_SIZE] ; BOARD_SIZE] {
        &mut self.gamestate
    }

    /// Returns true if the Cell at (x,y) is alive
    ///
    /// false otherwise
    /// 
    /// # Arguments
    /// 
    /// * `x` - row index
    /// * `y` - column index
    /// 
    pub fn state(&self, x: usize, y: usize) -> bool {
        if let Cell::Alive = self.gamestate[x][y] {
            true
        } else {
            false
        }
    }

    /// Sets 'cells' amount of cells inside the current Board to 'Alive'
    /// 
    /// # Arguments
    /// 
    /// * `cells` - Amount of cells that should be set to 'Alive'
    /// 
    pub fn randomize(&mut self, cells: u8){
        let mut counter = 0;
        while counter < cells {
            let x = thread_rng().gen_range(0..BOARD_SIZE);
            let y = thread_rng().gen_range(0..BOARD_SIZE);
            match self.gamestate[x][y] {
                Cell::Dead => {
                    self.gamestate[x][y] = Cell::Alive;
                    counter += 1;
                },
                Cell::Alive => continue,
            }
        }
    }

    /// Primary update function
    /// 
    /// Calculates amount of Live neighbours for every Cell in every row and every column
    /// 
    /// and creates a new board based on the rules:
    /// 
    /// * A Live cell with 2 or 3 neighbours stays Live
    /// * A Dead cell with 3 neighbours becomes a Live cell
    /// * Every other Live cell stays Live and every other Dead cell stays Dead
    pub fn update_board(&self) -> Board {
        let mut board = Board::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let neighbours = self.count_live_neighbours(row as i8, col as i8);
                let row_u = row as usize;
                let col_u = col as usize;
                match neighbours {
                    0 | 1 => board.gamestate[row_u][col_u] = Cell::Dead,
                    2 => board.gamestate[row_u][col_u] = self.gamestate[row_u][col_u],
                    3 => board.gamestate[row_u][col_u] = Cell::Alive,
                    _ => board.gamestate[row_u][col_u] = Cell::Dead,
                }
            }
        }
        return board;
    }

    /// Counts and returns the amount of Live neighbours for the Cell
    /// 
    /// at row x column y
    /// 
    /// # Arguments
    /// 
    /// * `x` - row
    /// * `y` - column
    pub fn count_live_neighbours(&self, x: i8, y: i8) -> u8 {
        let mut counter = 0;
        for row in x-1..x+2 {
            for col in y-1..y+2 {
                if (row, col) == (x, y) {
                    continue;
                }
                let board_sizei = BOARD_SIZE as i8;
                let y = if col % board_sizei < 0 {col % board_sizei + board_sizei} else {col % board_sizei};
                let x = if row % board_sizei < 0 {row % board_sizei + board_sizei} else {row % board_sizei};
                if let true = self.state(x as usize, y as usize) {
                    counter += 1;
                } else {
                    continue;
                }
            }
        }
        return counter;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = 0;
        let mut y = 0;
        while x < BOARD_SIZE {
            if x < 9 {
                write!(f, " {} |", self.gamestate[x][y]).expect("Error while writing Board state to stdout");
            } else {
                write!(f, " {} ", self.gamestate[x][y]).expect("Error while writing Board state to stdout");
            }
            x+=1;
            if x == BOARD_SIZE {
                write!(f, "\n").expect("Error while writing Board state to stdout");
                x = 0;
                y+=1;
                if y == BOARD_SIZE {
                    return Ok(())
                } else {
                    write!(f, "----------------------------------------\n").expect("Error while writing Board state to stdout")
                }
            }
        }
        return Ok(())
    }
}

/// Enum representing the two states of a Cell in Conway's Game of Life
/// 
/// * `Alive` - Cell is considered Alive
/// * `Dead` - Cell is considered Dead
#[derive(Clone, Copy)]
pub enum Cell {
    Alive,
    Dead,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Cell::Alive => write!(f, "O"),
            Cell::Dead => write!(f, "X")
        }
    }
}