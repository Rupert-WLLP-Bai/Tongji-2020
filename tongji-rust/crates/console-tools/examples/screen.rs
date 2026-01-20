//! Example: Screen functions demonstration
//!
//! Demonstrates screen clearing and size detection

use console_tools::{cct_cls, cct_gotoxy, cct_setcolor, cct_getconsoleborder, flush};
use std::io;

fn main() -> io::Result<()> {
    // Get console size
    let (width, height) = cct_getconsoleborder()?;

    // Clear screen
    cct_cls()?;
    cct_setcolor(15, 0)?;

    // Title
    cct_gotoxy(0, 0)?;
    println!("Console Tools - Screen Demo");
    println!("===========================\n");

    // Display console information
    println!("Console size: {} columns x {} rows", width, height);
    println!("\nDrawing border around console...\n");
    flush()?;

    // Draw border
    // Top border
    cct_gotoxy(0, 0)?;
    cct_setcolor(14, 1)?; // Yellow on blue
    for _ in 0..width {
        print!("═");
    }

    // Bottom border
    cct_gotoxy(0, height - 1)?;
    for _ in 0..width {
        print!("═");
    }

    // Side borders
    for y in 1..height - 1 {
        cct_gotoxy(0, y)?;
        print!("║");
        cct_gotoxy(width - 1, y)?;
        print!("║");
    }

    // Corners
    cct_gotoxy(0, 0)?;
    print!("╔");
    cct_gotoxy(width - 1, 0)?;
    print!("╗");
    cct_gotoxy(0, height - 1)?;
    print!("╚");
    cct_gotoxy(width - 1, height - 1)?;
    print!("╝");

    // Reset colors and position
    cct_setcolor(15, 0)?;
    cct_gotoxy(2, height / 2)?;
    println!("Press Enter to clear screen and exit...");
    flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Clear screen before exit
    cct_cls()?;
    println!("Screen cleared. Goodbye!");

    Ok(())
}
