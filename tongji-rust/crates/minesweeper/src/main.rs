/// Minesweeper Game - Main Entry Point
use minesweeper::{display, flood_fill, generate_mines, validation, Board, Difficulty};
use std::io::{self, Write};

fn main() {
    println!("=== Minesweeper Game ===\n");

    loop {
        // Get difficulty
        let difficulty = get_difficulty();
        if difficulty.is_none() {
            println!("Goodbye!");
            break;
        }
        let difficulty = difficulty.unwrap();

        // Create board
        let mut board = Board::new(difficulty);
        let mut first_move = true;
        let mut game_over = false;

        println!("\nStarting new game: {:?}", difficulty);
        println!("Commands: <x> <y> [f] - Reveal cell or flag (f)");
        println!("          q - Quit game\n");

        display::display_board(&board, false);

        // Game loop
        while !game_over {
            // Get player input
            let action = get_action(&board);

            match action {
                Action::Quit => {
                    println!("Quitting game...");
                    break;
                }
                Action::Reveal(x, y) => {
                    // Generate mines on first move
                    if first_move {
                        generate_mines(&mut board, x, y);
                        first_move = false;
                    }

                    // Try to reveal the cell
                    let result = validation::try_reveal(&mut board, x, y);

                    match result {
                        validation::RevealResult::Safe => {
                            // If count is 0, flood fill
                            if board.get_count(x, y) == 0 {
                                flood_fill(&mut board, x, y);
                            }

                            display::display_board(&board, false);

                            // Check win condition
                            if validation::is_win(&board) {
                                println!("\n🎉 Congratulations! You won!");
                                display::display_stats(&board);
                                game_over = true;
                            }
                        }
                        validation::RevealResult::Mine => {
                            display::display_board(&board, true);
                            println!("\n💥 Game Over! You hit a mine!");
                            display::display_stats(&board);
                            game_over = true;
                        }
                        validation::RevealResult::AlreadyRevealed => {
                            println!("Cell already revealed!");
                        }
                        validation::RevealResult::Flagged => {
                            println!("Cell is flagged! Remove flag first.");
                        }
                    }
                }
                Action::Flag(x, y) => {
                    if board.toggle_flag(x, y) {
                        display::display_board(&board, false);
                    } else {
                        println!("Cannot flag revealed cell!");
                    }
                }
            }
        }

        println!("\n{}", "=".repeat(50));
    }
}

enum Action {
    Reveal(usize, usize),
    Flag(usize, usize),
    Quit,
}

fn get_difficulty() -> Option<Difficulty> {
    loop {
        println!("Select difficulty:");
        println!("  1. Easy (9×9, 10 mines)");
        println!("  2. Medium (16×16, 40 mines)");
        println!("  3. Hard (30×16, 99 mines)");
        println!("  0. Quit");
        print!("Choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "0" => return None,
            "1" => return Some(Difficulty::Easy),
            "2" => return Some(Difficulty::Medium),
            "3" => return Some(Difficulty::Hard),
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_action(board: &Board) -> Action {
    loop {
        print!("\nEnter command (x y [f] or q): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        if parts[0] == "q" {
            return Action::Quit;
        }

        if parts.len() < 2 {
            println!("Invalid command. Use: x y [f]");
            continue;
        }

        let x = match parts[0].parse::<usize>() {
            Ok(n) if n < board.width => n,
            _ => {
                println!("Invalid x coordinate (0-{})", board.width - 1);
                continue;
            }
        };

        let y = match parts[1].parse::<usize>() {
            Ok(n) if n < board.height => n,
            _ => {
                println!("Invalid y coordinate (0-{})", board.height - 1);
                continue;
            }
        };

        let is_flag = parts.len() > 2 && parts[2] == "f";

        if is_flag {
            return Action::Flag(x, y);
        } else {
            return Action::Reveal(x, y);
        }
    }
}
