/// Minesweeper Game Library
///
/// This library provides the core logic for the Minesweeper game.
/// It includes board management, mine generation, flood fill, and validation.
pub mod board;
pub mod display;
pub mod flood_fill;
pub mod mine_gen;
pub mod validation;

pub use board::{Board, CellState, Difficulty};
pub use flood_fill::flood_fill;
pub use mine_gen::generate_mines;
pub use validation::{is_loss, is_win, RevealResult};
