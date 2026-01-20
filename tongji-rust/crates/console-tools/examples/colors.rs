//! Example: Color demonstration
//!
//! Demonstrates the color functions of console-tools

use console_tools::{cct_cls, cct_setcolor, cct_gotoxy, flush};
use std::io;

fn main() -> io::Result<()> {
    // Clear screen
    cct_cls()?;

    // Display color palette
    cct_gotoxy(0, 0)?;
    println!("Console Tools - Color Demo");
    println!("==========================\n");

    // Show all 16 colors
    for bg in 0..16 {
        cct_gotoxy(0, 3 + bg)?;
        for fg in 0..16 {
            cct_setcolor(fg as u8, bg as u8)?;
            print!(" {:2X} ", fg);
        }
        cct_setcolor(15, 0)?; // Reset to white on black
        print!("  BG: {:2}", bg);
        flush()?;
    }

    // Reset colors
    cct_setcolor(15, 0)?;
    cct_gotoxy(0, 20)?;
    println!("\nPress Enter to exit...");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
