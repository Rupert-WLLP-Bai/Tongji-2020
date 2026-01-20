/// Hanoi Tower Game Library
///
/// This library provides the core logic for the Hanoi Tower puzzle game.
/// It includes state management, game logic, and validation.

pub mod state;
pub mod game;
pub mod validation;
pub mod display;

pub use state::{GameState, Pillar};
pub use game::hanoi;
pub use validation::{MoveError, validate_move};
