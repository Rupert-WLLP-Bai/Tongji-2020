//! Console Tools Library
//!
//! A cross-platform console manipulation library that provides Rust equivalents
//! for the cmd_console_tools functions used in the original C/C++ projects.
//!
//! This library uses `crossterm` to provide cross-platform terminal control.

pub mod color;
pub mod cursor;
pub mod screen;
pub mod input;

// Re-export commonly used functions
pub use color::{cct_setcolor, cct_setconsoleborder};
pub use cursor::{cct_gotoxy, cct_setcursor, cct_getxy};
pub use screen::{cct_cls, cct_getconsoleborder, cct_setconsoleborder_size};
pub use input::{cct_read_keyboard_and_mouse, KeyMouseEvent};

use std::io::{self, Write};

/// Initialize console for use with console-tools
/// Call this at the start of your program
pub fn init() -> io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    Ok(())
}

/// Cleanup console state
/// Call this before your program exits
pub fn cleanup() -> io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

/// Flush stdout to ensure all output is displayed
pub fn flush() -> io::Result<()> {
    io::stdout().flush()
}
