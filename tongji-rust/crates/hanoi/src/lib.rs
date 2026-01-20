pub mod display;
pub mod game;
/// Hanoi Tower Game Library
///
/// This library provides the core logic for the Hanoi Tower puzzle game.
/// It includes state management, game logic, and validation.
pub mod state;
pub mod validation;

pub use game::hanoi;
pub use state::{GameState, Pillar};
pub use validation::{validate_move, MoveError};
