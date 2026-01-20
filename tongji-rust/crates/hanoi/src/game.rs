/// Core game logic for Hanoi Tower
use crate::state::{GameState, Pillar};
use crate::validation::validate_move;
use std::thread;
use std::time::Duration;

/// Execute a single move in the game
pub fn execute_move(
    state: &mut GameState,
    from: Pillar,
    to: Pillar,
    display: bool,
) -> Result<(), String> {
    // Validate the move
    validate_move(state, from, to).map_err(|e| e.to_string())?;

    // Execute the move
    let disk = state.pop(from).unwrap();
    state.push(to, disk);
    state.increment_moves();

    // Display the move if requested
    if display {
        println!(
            "Step {}: Move disk {} from {} to {}",
            state.move_count, disk, from, to
        );
    }

    // Apply delay based on speed
    if state.speed > 0 {
        let delay_ms = state.speed as u64 * 100;
        thread::sleep(Duration::from_millis(delay_ms));
    }

    Ok(())
}

/// Solve the Hanoi Tower puzzle recursively
///
/// # Arguments
/// * `n` - Number of disks to move
/// * `src` - Source pillar
/// * `tmp` - Temporary/auxiliary pillar
/// * `dst` - Destination pillar
/// * `state` - Game state
/// * `display` - Whether to display moves
pub fn hanoi(
    n: u8,
    src: Pillar,
    tmp: Pillar,
    dst: Pillar,
    state: &mut GameState,
    display: bool,
) -> Result<(), String> {
    if n == 1 {
        execute_move(state, src, dst, display)?;
    } else {
        // Move n-1 disks from src to tmp using dst
        hanoi(n - 1, src, dst, tmp, state, display)?;
        // Move the largest disk from src to dst
        execute_move(state, src, dst, display)?;
        // Move n-1 disks from tmp to dst using src
        hanoi(n - 1, tmp, src, dst, state, display)?;
    }
    Ok(())
}

/// Calculate the minimum number of moves required for n disks
pub fn min_moves(n: u8) -> u32 {
    2_u32.pow(n as u32) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_move_success() {
        let mut state = GameState::new(3, Pillar::A, Pillar::C);
        assert!(execute_move(&mut state, Pillar::A, Pillar::B, false).is_ok());
        assert_eq!(state.move_count, 1);
        assert_eq!(state.count(Pillar::A), 2);
        assert_eq!(state.count(Pillar::B), 1);
        assert_eq!(state.top(Pillar::B), Some(1));
    }

    #[test]
    fn test_execute_move_invalid() {
        let mut state = GameState::new(3, Pillar::A, Pillar::C);
        // Try to move from empty pillar
        assert!(execute_move(&mut state, Pillar::B, Pillar::A, false).is_err());
    }

    #[test]
    fn test_hanoi_one_disk() {
        let mut state = GameState::new(1, Pillar::A, Pillar::C);
        assert!(hanoi(1, Pillar::A, Pillar::B, Pillar::C, &mut state, false).is_ok());
        assert_eq!(state.move_count, 1);
        assert!(state.is_complete());
    }

    #[test]
    fn test_hanoi_two_disks() {
        let mut state = GameState::new(2, Pillar::A, Pillar::C);
        assert!(hanoi(2, Pillar::A, Pillar::B, Pillar::C, &mut state, false).is_ok());
        assert_eq!(state.move_count, 3);
        assert!(state.is_complete());
    }

    #[test]
    fn test_hanoi_three_disks() {
        let mut state = GameState::new(3, Pillar::A, Pillar::C);
        assert!(hanoi(3, Pillar::A, Pillar::B, Pillar::C, &mut state, false).is_ok());
        assert_eq!(state.move_count, 7);
        assert!(state.is_complete());
    }

    #[test]
    fn test_hanoi_correctness() {
        // Test that the final state is correct for various disk counts
        for n in 1..=10 {
            let mut state = GameState::new(n, Pillar::A, Pillar::C);
            hanoi(n, Pillar::A, Pillar::B, Pillar::C, &mut state, false).unwrap();

            // All disks should be on target pillar
            assert!(state.is_complete());
            // Should be in correct order (largest at bottom)
            let stack = state.stack(Pillar::C);
            for i in 0..stack.len() - 1 {
                assert!(stack[i] > stack[i + 1], "Disks not in correct order");
            }
        }
    }

    #[test]
    fn test_min_moves() {
        assert_eq!(min_moves(1), 1);
        assert_eq!(min_moves(2), 3);
        assert_eq!(min_moves(3), 7);
        assert_eq!(min_moves(4), 15);
        assert_eq!(min_moves(5), 31);
        assert_eq!(min_moves(10), 1023);
    }

    #[test]
    fn test_hanoi_move_count() {
        // Verify that move count equals 2^n - 1 for all n
        for n in 1..=10 {
            let mut state = GameState::new(n, Pillar::A, Pillar::C);
            hanoi(n, Pillar::A, Pillar::B, Pillar::C, &mut state, false).unwrap();
            assert_eq!(state.move_count, min_moves(n));
        }
    }

    #[test]
    fn test_hanoi_different_start_end() {
        // Test starting from B and ending at A
        let mut state = GameState::new(3, Pillar::B, Pillar::A);
        assert!(hanoi(3, Pillar::B, Pillar::C, Pillar::A, &mut state, false).is_ok());
        assert_eq!(state.move_count, 7);
        assert!(state.is_complete());
    }

    #[test]
    fn test_hanoi_stack_integrity() {
        // Ensure no invalid moves occur during solving
        let mut state = GameState::new(5, Pillar::A, Pillar::C);
        hanoi(5, Pillar::A, Pillar::B, Pillar::C, &mut state, false).unwrap();

        // Final stack should have all disks in order
        let stack = state.stack(Pillar::C);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack, &[5, 4, 3, 2, 1]);
    }
}
