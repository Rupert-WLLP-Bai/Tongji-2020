/// Minesweeper Game Library
///
/// This library provides the core logic for the Minesweeper game.
/// It includes board management, mine generation, flood fill, and validation.

pub mod board;
pub mod mine_gen;
pub mod flood_fill;
pub mod validation;
pub mod display;

pub use board::{Board, Difficulty, CellState};
pub use mine_gen::generate_mines;
pub use flood_fill::flood_fill;
pub use validation::{RevealResult, is_win, is_loss};
