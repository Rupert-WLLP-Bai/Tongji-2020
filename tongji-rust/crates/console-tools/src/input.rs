//! Input handling functions
//!
//! Provides functions for reading keyboard and mouse input.

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use std::io;

/// Key and mouse event types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyMouseEvent {
    /// Keyboard key press
    Key {
        code: KeyCode,
        modifiers: KeyModifiers,
    },
    /// Mouse click
    MouseClick {
        button: MouseButton,
        x: u16,
        y: u16,
    },
    /// Mouse movement
    MouseMove {
        x: u16,
        y: u16,
    },
    /// No event (timeout)
    None,
}

/// Read keyboard and mouse input
///
/// This function blocks until an event occurs or timeout is reached.
///
/// # Arguments
/// * `timeout_ms` - Timeout in milliseconds (0 = no timeout, wait indefinitely)
///
/// # Returns
/// Returns a KeyMouseEvent representing the input event
///
/// # Example
/// ```no_run
/// use console_tools::{cct_read_keyboard_and_mouse, KeyMouseEvent};
///
/// // Wait for input with 1 second timeout
/// match cct_read_keyboard_and_mouse(1000).unwrap() {
///     KeyMouseEvent::Key { code, .. } => {
///         println!("Key pressed: {:?}", code);
///     }
///     KeyMouseEvent::MouseClick { button, x, y } => {
///         println!("Mouse clicked at ({}, {}) with {:?}", x, y, button);
///     }
///     KeyMouseEvent::None => {
///         println!("Timeout");
///     }
///     _ => {}
/// }
/// ```
pub fn cct_read_keyboard_and_mouse(timeout_ms: u64) -> io::Result<KeyMouseEvent> {
    // Enable mouse capture
    crossterm::execute!(
        io::stdout(),
        event::EnableMouseCapture
    )?;

    let result = if timeout_ms == 0 {
        // No timeout - wait indefinitely
        match event::read()? {
            Event::Key(key_event) => Ok(KeyMouseEvent::Key {
                code: key_event.code,
                modifiers: key_event.modifiers,
            }),
            Event::Mouse(mouse_event) => Ok(parse_mouse_event(mouse_event)),
            _ => Ok(KeyMouseEvent::None),
        }
    } else {
        // With timeout
        let timeout = std::time::Duration::from_millis(timeout_ms);
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key_event) => Ok(KeyMouseEvent::Key {
                    code: key_event.code,
                    modifiers: key_event.modifiers,
                }),
                Event::Mouse(mouse_event) => Ok(parse_mouse_event(mouse_event)),
                _ => Ok(KeyMouseEvent::None),
            }
        } else {
            Ok(KeyMouseEvent::None)
        }
    };

    // Disable mouse capture
    crossterm::execute!(
        io::stdout(),
        event::DisableMouseCapture
    )?;

    result
}

/// Parse mouse event into KeyMouseEvent
fn parse_mouse_event(event: MouseEvent) -> KeyMouseEvent {
    match event.kind {
        MouseEventKind::Down(button) | MouseEventKind::Up(button) => {
            KeyMouseEvent::MouseClick {
                button,
                x: event.column,
                y: event.row,
            }
        }
        MouseEventKind::Drag(_) | MouseEventKind::Moved => {
            KeyMouseEvent::MouseMove {
                x: event.column,
                y: event.row,
            }
        }
        _ => KeyMouseEvent::None,
    }
}

/// Read a single character from keyboard (blocking)
///
/// # Example
/// ```no_run
/// use console_tools::input::read_char;
/// let ch = read_char().unwrap();
/// println!("You pressed: {}", ch);
/// ```
pub fn read_char() -> io::Result<char> {
    loop {
        if let Event::Key(KeyEvent { code: KeyCode::Char(c), .. }) = event::read()? {
            return Ok(c);
        }
    }
}

/// Read a line of text from keyboard
///
/// # Example
/// ```no_run
/// use console_tools::input::read_line;
/// let line = read_line().unwrap();
/// println!("You entered: {}", line);
/// ```
pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_mouse_event() {
        let event = KeyMouseEvent::Key {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::empty(),
        };
        assert!(matches!(event, KeyMouseEvent::Key { .. }));
    }

    #[test]
    fn test_mouse_click_event() {
        let event = KeyMouseEvent::MouseClick {
            button: MouseButton::Left,
            x: 10,
            y: 20,
        };
        assert!(matches!(event, KeyMouseEvent::MouseClick { .. }));
    }
}
