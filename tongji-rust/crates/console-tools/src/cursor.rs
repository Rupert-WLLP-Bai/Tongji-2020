//! Cursor control functions
//!
//! Provides functions for controlling cursor position and visibility.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
};
use std::io::{self, stdout};

/// Move cursor to specified position
///
/// # Arguments
/// * `x` - Column position (0-based)
/// * `y` - Row position (0-based)
///
/// # Example
/// ```no_run
/// use console_tools::cct_gotoxy;
/// // Move cursor to column 10, row 5
/// cct_gotoxy(10, 5).unwrap();
/// ```
pub fn cct_gotoxy(x: u16, y: u16) -> io::Result<()> {
    execute!(stdout(), MoveTo(x, y))?;
    Ok(())
}

/// Set cursor visibility
///
/// # Arguments
/// * `visible` - true to show cursor, false to hide
///
/// # Example
/// ```no_run
/// use console_tools::cct_setcursor;
/// // Hide cursor
/// cct_setcursor(false).unwrap();
/// // Show cursor
/// cct_setcursor(true).unwrap();
/// ```
pub fn cct_setcursor(visible: bool) -> io::Result<()> {
    if visible {
        execute!(stdout(), Show)?;
    } else {
        execute!(stdout(), Hide)?;
    }
    Ok(())
}

/// Get current cursor position
///
/// Returns (x, y) tuple with current cursor position
///
/// # Example
/// ```no_run
/// use console_tools::cct_getxy;
/// let (x, y) = cct_getxy().unwrap();
/// println!("Cursor at: ({}, {})", x, y);
/// ```
pub fn cct_getxy() -> io::Result<(u16, u16)> {
    let pos = crossterm::cursor::position()?;
    Ok(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_functions_compile() {
        // These functions require a terminal, so we just test compilation
        // Actual testing would need to be done in integration tests
    }
}
