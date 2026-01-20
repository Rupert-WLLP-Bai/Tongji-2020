/// Display functions for Hanoi Tower
use crate::state::{GameState, Pillar};

/// Display the current state of all three pillars
pub fn display_state(state: &GameState) {
    println!("\n=== Hanoi Tower State ===");
    println!("Moves: {}", state.move_count);
    println!("\nPillar A: {:?}", state.stack(Pillar::A));
    println!("Pillar B: {:?}", state.stack(Pillar::B));
    println!("Pillar C: {:?}", state.stack(Pillar::C));
    println!();
}

/// Display pillars in a vertical format
pub fn display_vertical(state: &GameState) {
    let max_height = state.disk_count as usize;

    println!("\n=== Hanoi Tower ===");
    println!("Moves: {}\n", state.move_count);

    // Display from top to bottom
    for level in (0..max_height).rev() {
        print!("  ");
        for pillar in [Pillar::A, Pillar::B, Pillar::C] {
            let stack = state.stack(pillar);
            if level < stack.len() {
                print!("{:3} ", stack[level]);
            } else {
                print!(" |  ");
            }
        }
        println!();
    }

    // Display base
    println!("  --- --- ---");
    println!("   A   B   C");
    println!();
}

/// Display a simple text representation
pub fn display_simple(state: &GameState) {
    println!(
        "A: {:?}  B: {:?}  C: {:?}  (Moves: {})",
        state.stack(Pillar::A),
        state.stack(Pillar::B),
        state.stack(Pillar::C),
        state.move_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_functions() {
        let state = GameState::new(3, Pillar::A, Pillar::C);

        // These functions should not panic
        display_state(&state);
        display_vertical(&state);
        display_simple(&state);
    }
}
