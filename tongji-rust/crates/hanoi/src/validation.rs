/// Move validation for Hanoi Tower
use crate::state::{GameState, Pillar};
use std::fmt;

/// Errors that can occur during move validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveError {
    /// Source pillar is empty
    SourceEmpty,
    /// Cannot place larger disk on smaller disk
    LargerOnSmaller { disk: u8, top: u8 },
    /// Source and destination are the same
    SamePillar,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::SourceEmpty => write!(f, "Source pillar is empty"),
            MoveError::LargerOnSmaller { disk, top } => {
                write!(f, "Cannot place disk {} on smaller disk {}", disk, top)
            }
            MoveError::SamePillar => write!(f, "Source and destination are the same"),
        }
    }
}

impl std::error::Error for MoveError {}

/// Validate a move from one pillar to another
pub fn validate_move(state: &GameState, from: Pillar, to: Pillar) -> Result<(), MoveError> {
    // Check if source and destination are the same
    if from == to {
        return Err(MoveError::SamePillar);
    }

    // Check if source pillar is empty
    let disk = state.top(from).ok_or(MoveError::SourceEmpty)?;

    // Check if we can place this disk on the destination
    if let Some(top_disk) = state.top(to) {
        if disk > top_disk {
            return Err(MoveError::LargerOnSmaller {
                disk,
                top: top_disk,
            });
        }
    }

    Ok(())
}

/// Check if a disk can be placed on a pillar
pub fn can_place_on(state: &GameState, disk: u8, pillar: Pillar) -> bool {
    match state.top(pillar) {
        None => true, // Empty pillar, can place any disk
        Some(top) => disk < top, // Can only place smaller disk on larger disk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_move_success() {
        let mut state = GameState::new(3, Pillar::A, Pillar::C);
        // Move smallest disk from A to B should succeed
        assert!(validate_move(&state, Pillar::A, Pillar::B).is_ok());
    }

    #[test]
    fn test_validate_move_source_empty() {
        let state = GameState::new(3, Pillar::A, Pillar::C);
        // B is empty, cannot move from B
        assert_eq!(
            validate_move(&state, Pillar::B, Pillar::A),
            Err(MoveError::SourceEmpty)
        );
    }

    #[test]
    fn test_validate_move_larger_on_smaller() {
        let mut state = GameState::new(3, Pillar::A, Pillar::C);
        // Move disk 1 from A to B
        let disk = state.pop(Pillar::A).unwrap();
        state.push(Pillar::B, disk);

        // Now try to move disk 2 from A to B (should fail)
        let result = validate_move(&state, Pillar::A, Pillar::B);
        assert!(matches!(result, Err(MoveError::LargerOnSmaller { .. })));
    }

    #[test]
    fn test_validate_move_same_pillar() {
        let state = GameState::new(3, Pillar::A, Pillar::C);
        assert_eq!(
            validate_move(&state, Pillar::A, Pillar::A),
            Err(MoveError::SamePillar)
        );
    }

    #[test]
    fn test_can_place_on_empty() {
        let state = GameState::new(3, Pillar::A, Pillar::C);
        // Can place any disk on empty pillar
        assert!(can_place_on(&state, 3, Pillar::B));
        assert!(can_place_on(&state, 1, Pillar::B));
    }

    #[test]
    fn test_can_place_on_smaller() {
        let state = GameState::new(3, Pillar::A, Pillar::C);
        // Top of A is disk 1, can only place nothing on it
        assert!(!can_place_on(&state, 2, Pillar::A));
        assert!(!can_place_on(&state, 3, Pillar::A));
    }

    #[test]
    fn test_can_place_on_larger() {
        let mut state = GameState::new(3, Pillar::A, Pillar::C);
        // Move disk 1 and 2 away, leaving disk 3 on A
        state.pop(Pillar::A);
        state.pop(Pillar::A);

        // Can place smaller disks on disk 3
        assert!(can_place_on(&state, 1, Pillar::A));
        assert!(can_place_on(&state, 2, Pillar::A));
        assert!(!can_place_on(&state, 3, Pillar::A));
    }

    #[test]
    fn test_all_move_combinations() {
        let state = GameState::new(1, Pillar::A, Pillar::C);

        // Valid moves from A
        assert!(validate_move(&state, Pillar::A, Pillar::B).is_ok());
        assert!(validate_move(&state, Pillar::A, Pillar::C).is_ok());

        // Invalid moves from empty pillars
        assert!(validate_move(&state, Pillar::B, Pillar::A).is_err());
        assert!(validate_move(&state, Pillar::B, Pillar::C).is_err());
        assert!(validate_move(&state, Pillar::C, Pillar::A).is_err());
        assert!(validate_move(&state, Pillar::C, Pillar::B).is_err());
    }
}
