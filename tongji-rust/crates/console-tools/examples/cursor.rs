//! Example: Cursor control demonstration
//!
//! Demonstrates cursor movement and visibility control

use console_tools::{cct_cls, cct_gotoxy, cct_setcursor, cct_setcolor, flush};
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Clear screen
    cct_cls()?;
    cct_setcolor(15, 0)?;

    // Title
    cct_gotoxy(0, 0)?;
    println!("Console Tools - Cursor Demo");
    println!("===========================\n");

    // Draw a box
    cct_gotoxy(10, 5)?;
    println!("┌────────────────────┐");
    for i in 0..5 {
        cct_gotoxy(10, 6 + i)?;
        println!("│                    │");
    }
    cct_gotoxy(10, 11)?;
    println!("└────────────────────┘");

    // Animate cursor movement
    println!("\nMoving cursor around the box...");
    flush()?;
    thread::sleep(Duration::from_secs(1));

    // Move cursor in a pattern
    for x in 12..30 {
        cct_gotoxy(x, 7)?;
        print!("*");
        flush()?;
        thread::sleep(Duration::from_millis(50));
    }

    // Hide cursor
    cct_gotoxy(0, 15)?;
    println!("Hiding cursor...");
    flush()?;
    thread::sleep(Duration::from_secs(1));
    cct_setcursor(false)?;
    thread::sleep(Duration::from_secs(2));

    // Show cursor
    println!("Showing cursor...");
    flush()?;
    cct_setcursor(true)?;

    cct_gotoxy(0, 18)?;
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
