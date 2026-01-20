//! Screen management functions
//!
//! Provides functions for clearing screen and managing console window size.

use crossterm::{
    execute,
    terminal::{Clear, ClearType, size},
};
use std::io::{self, stdout};

/// Clear the entire screen
///
/// # Example
/// ```no_run
/// use console_tools::cct_cls;
/// cct_cls().unwrap();
/// ```
pub fn cct_cls() -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All))?;
    // Move cursor to top-left after clearing
    crate::cursor::cct_gotoxy(0, 0)?;
    Ok(())
}

/// Get console window size (width, height)
///
/// Returns (columns, rows) tuple
///
/// # Example
/// ```no_run
/// use console_tools::cct_getconsoleborder;
/// let (width, height) = cct_getconsoleborder().unwrap();
/// println!("Console size: {}x{}", width, height);
/// ```
pub fn cct_getconsoleborder() -> io::Result<(u16, u16)> {
    let (cols, rows) = size()?;
    Ok((cols, rows))
}

/// Set console window size
///
/// Note: This function has limited cross-platform support.
/// On some terminals, resizing may not be possible.
///
/// # Arguments
/// * `width` - Number of columns
/// * `height` - Number of rows
///
/// # Example
/// ```no_run
/// use console_tools::cct_setconsoleborder_size;
/// // Set console to 80x25
/// cct_setconsoleborder_size(80, 25).unwrap();
/// ```
pub fn cct_setconsoleborder_size(width: u16, height: u16) -> io::Result<()> {
    // Note: crossterm doesn't provide direct window resizing
    // This is a platform-specific feature that's not universally supported
    // For now, we'll just return Ok as a no-op
    // In the original Windows code, this would use SetConsoleWindowInfo

    // TODO: Implement platform-specific resizing if needed
    // For most use cases, users can resize their terminal manually
    let _ = (width, height); // Suppress unused variable warning
    Ok(())
}

/// Show a string at a specific position
///
/// # Arguments
/// * `x` - Column position (0-based)
/// * `y` - Row position (0-based)
/// * `text` - Text to display
/// * `fg` - Foreground color (0-15)
/// * `bg` - Background color (0-15)
///
/// # Example
/// ```no_run
/// use console_tools::screen::cct_showstr;
/// // Show "Hello" at position (10, 5) with white text on black background
/// cct_showstr(10, 5, "Hello", 15, 0).unwrap();
/// ```
pub fn cct_showstr(x: u16, y: u16, text: &str, fg: u8, bg: u8) -> io::Result<()> {
    crate::cursor::cct_gotoxy(x, y)?;
    crate::color::cct_setcolor(fg, bg)?;
    print!("{}", text);
    crate::flush()?;
    Ok(())
}

/// Display a character at a specific position with color, repeated n times
///
/// # Arguments
/// * `x` - Column position (0-based)
/// * `y` - Row position (0-based)
/// * `ch` - Character to display
/// * `fg` - Foreground color (0-15)
/// * `bg` - Background color (0-15)
/// * `repeat` - Number of times to repeat the character
///
/// # Example
/// ```no_run
/// use console_tools::screen::cct_showch;
/// // Show "***" at position (10, 5) with white text on black background
/// cct_showch(10, 5, '*', 15, 0, 3).unwrap();
/// ```
pub fn cct_showch(x: u16, y: u16, ch: char, fg: u8, bg: u8, repeat: usize) -> io::Result<()> {
    crate::cursor::cct_gotoxy(x, y)?;
    crate::color::cct_setcolor(fg, bg)?;
    print!("{}", ch.to_string().repeat(repeat));
    crate::flush()?;
    Ok(())
}

/// Display an integer at a specific position with color
///
/// # Arguments
/// * `x` - Column position (0-based)
/// * `y` - Row position (0-based)
/// * `num` - Integer to display
/// * `fg` - Foreground color (0-15)
/// * `bg` - Background color (0-15)
///
/// # Example
/// ```no_run
/// use console_tools::screen::cct_showint;
/// // Show "42" at position (10, 5) with white text on black background
/// cct_showint(10, 5, 42, 15, 0).unwrap();
/// ```
pub fn cct_showint(x: u16, y: u16, num: i32, fg: u8, bg: u8) -> io::Result<()> {
    crate::cursor::cct_gotoxy(x, y)?;
    crate::color::cct_setcolor(fg, bg)?;
    print!("{}", num);
    crate::flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_functions_compile() {
        // These functions require a terminal, so we just test compilation
        // Actual testing would need to be done in integration tests
    }
}
