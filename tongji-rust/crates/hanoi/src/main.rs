/// Hanoi Tower Game - Main Entry Point
use hanoi::{GameState, Pillar, hanoi, game};
use std::io::{self, Write};

fn main() {
    println!("=== Hanoi Tower Game ===\n");

    loop {
        // Get number of disks
        let disk_count = get_disk_count();
        if disk_count == 0 {
            println!("Goodbye!");
            break;
        }

        // Get starting pillar
        let start = get_pillar("Enter starting pillar (A/B/C): ");

        // Get target pillar
        let target = get_pillar_different_from(
            "Enter target pillar (A/B/C): ",
            start
        );

        // Get auxiliary pillar (the remaining one)
        let aux = get_auxiliary_pillar(start, target);

        // Get display mode
        let display = get_display_mode();

        // Create game state
        let mut state = GameState::new(disk_count, start, target);

        println!("\nSolving Hanoi Tower with {} disks from {} to {} using {}...\n",
                 disk_count, start, target, aux);

        if display {
            hanoi::display::display_vertical(&state);
        }

        // Solve the puzzle
        match hanoi(disk_count, start, aux, target, &mut state, display) {
            Ok(_) => {
                println!("\n✓ Puzzle solved in {} moves!", state.move_count);
                println!("Minimum moves required: {}", game::min_moves(disk_count));

                if display {
                    hanoi::display::display_vertical(&state);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        println!("\n{}", "=".repeat(50));
    }
}

fn get_disk_count() -> u8 {
    loop {
        print!("Enter number of disks (1-10, 0 to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u8>() {
            Ok(n) if n == 0 => return 0,
            Ok(n) if n >= 1 && n <= 10 => return n,
            _ => println!("Invalid input. Please enter a number between 1 and 10."),
        }
    }
}

fn get_pillar(prompt: &str) -> Pillar {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(c) = input.trim().chars().next() {
            if let Some(pillar) = Pillar::from_char(c) {
                return pillar;
            }
        }

        println!("Invalid input. Please enter A, B, or C.");
    }
}

fn get_pillar_different_from(prompt: &str, exclude: Pillar) -> Pillar {
    loop {
        let pillar = get_pillar(prompt);
        if pillar != exclude {
            return pillar;
        }
        println!("Target must be different from source. Please try again.");
    }
}

fn get_auxiliary_pillar(start: Pillar, target: Pillar) -> Pillar {
    for pillar in [Pillar::A, Pillar::B, Pillar::C] {
        if pillar != start && pillar != target {
            return pillar;
        }
    }
    unreachable!()
}

fn get_display_mode() -> bool {
    loop {
        print!("Display each move? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid input. Please enter y or n."),
        }
    }
}
