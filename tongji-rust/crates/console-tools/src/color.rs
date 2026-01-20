//! Color management functions
//!
//! Provides functions for setting console text and background colors.

use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
};
use std::io::{self, stdout};

/// Console color codes (matching Windows console colors)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsoleColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Yellow = 6,
    White = 7,
    BrightBlack = 8,
    BrightBlue = 9,
    BrightGreen = 10,
    BrightCyan = 11,
    BrightRed = 12,
    BrightMagenta = 13,
    BrightYellow = 14,
    BrightWhite = 15,
}

impl ConsoleColor {
    /// Convert console color code to crossterm Color
    pub fn to_crossterm_color(self) -> Color {
        match self {
            ConsoleColor::Black => Color::Black,
            ConsoleColor::Blue => Color::DarkBlue,
            ConsoleColor::Green => Color::DarkGreen,
            ConsoleColor::Cyan => Color::DarkCyan,
            ConsoleColor::Red => Color::DarkRed,
            ConsoleColor::Magenta => Color::DarkMagenta,
            ConsoleColor::Yellow => Color::DarkYellow,
            ConsoleColor::White => Color::Grey,
            ConsoleColor::BrightBlack => Color::DarkGrey,
            ConsoleColor::BrightBlue => Color::Blue,
            ConsoleColor::BrightGreen => Color::Green,
            ConsoleColor::BrightCyan => Color::Cyan,
            ConsoleColor::BrightRed => Color::Red,
            ConsoleColor::BrightMagenta => Color::Magenta,
            ConsoleColor::BrightYellow => Color::Yellow,
            ConsoleColor::BrightWhite => Color::White,
        }
    }

    /// Create from u8 color code
    pub fn from_u8(code: u8) -> Option<Self> {
        match code {
            0 => Some(ConsoleColor::Black),
            1 => Some(ConsoleColor::Blue),
            2 => Some(ConsoleColor::Green),
            3 => Some(ConsoleColor::Cyan),
            4 => Some(ConsoleColor::Red),
            5 => Some(ConsoleColor::Magenta),
            6 => Some(ConsoleColor::Yellow),
            7 => Some(ConsoleColor::White),
            8 => Some(ConsoleColor::BrightBlack),
            9 => Some(ConsoleColor::BrightBlue),
            10 => Some(ConsoleColor::BrightGreen),
            11 => Some(ConsoleColor::BrightCyan),
            12 => Some(ConsoleColor::BrightRed),
            13 => Some(ConsoleColor::BrightMagenta),
            14 => Some(ConsoleColor::BrightYellow),
            15 => Some(ConsoleColor::BrightWhite),
            _ => None,
        }
    }
}

/// Set console text and background color
///
/// # Arguments
/// * `fg` - Foreground color code (0-15)
/// * `bg` - Background color code (0-15)
///
/// # Example
/// ```no_run
/// use console_tools::cct_setcolor;
/// // Set white text on black background
/// cct_setcolor(15, 0).unwrap();
/// ```
pub fn cct_setcolor(fg: u8, bg: u8) -> io::Result<()> {
    let fg_color = ConsoleColor::from_u8(fg)
        .unwrap_or(ConsoleColor::White)
        .to_crossterm_color();
    let bg_color = ConsoleColor::from_u8(bg)
        .unwrap_or(ConsoleColor::Black)
        .to_crossterm_color();

    execute!(
        stdout(),
        SetForegroundColor(fg_color),
        SetBackgroundColor(bg_color)
    )?;

    Ok(())
}

/// Set console border color (background color for the entire console)
///
/// # Arguments
/// * `bg` - Background color code (0-15)
///
/// # Example
/// ```no_run
/// use console_tools::cct_setconsoleborder;
/// // Set black border
/// cct_setconsoleborder(0).unwrap();
/// ```
pub fn cct_setconsoleborder(bg: u8) -> io::Result<()> {
    let bg_color = ConsoleColor::from_u8(bg)
        .unwrap_or(ConsoleColor::Black)
        .to_crossterm_color();

    execute!(stdout(), SetBackgroundColor(bg_color))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        assert_eq!(ConsoleColor::from_u8(0), Some(ConsoleColor::Black));
        assert_eq!(ConsoleColor::from_u8(15), Some(ConsoleColor::BrightWhite));
        assert_eq!(ConsoleColor::from_u8(16), None);
    }

    #[test]
    fn test_crossterm_color_conversion() {
        let color = ConsoleColor::Red;
        let ct_color = color.to_crossterm_color();
        assert_eq!(ct_color, Color::DarkRed);
    }
}
