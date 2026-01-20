/// Game state management for Hanoi Tower
use std::fmt;

/// Represents a pillar in the Hanoi Tower game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pillar {
    A,
    B,
    C,
}

impl Pillar {
    /// Convert character to Pillar
    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Pillar::A),
            'B' => Some(Pillar::B),
            'C' => Some(Pillar::C),
            _ => None,
        }
    }

    /// Convert Pillar to character
    pub fn to_char(self) -> char {
        match self {
            Pillar::A => 'A',
            Pillar::B => 'B',
            Pillar::C => 'C',
        }
    }

    /// Get index (0, 1, or 2)
    pub fn index(self) -> usize {
        match self {
            Pillar::A => 0,
            Pillar::B => 1,
            Pillar::C => 2,
        }
    }
}

impl fmt::Display for Pillar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

/// Game state for Hanoi Tower
#[derive(Debug, Clone)]
pub struct GameState {
    /// Three stacks representing pillars A, B, C
    /// Each stack contains disk numbers (1 = smallest, n = largest)
    stacks: [Vec<u8>; 3],
    /// Move counter
    pub move_count: u32,
    /// Animation speed (0-5, where 0 = instant, 5 = slowest)
    pub speed: u8,
    /// Number of disks in the game
    pub disk_count: u8,
    /// Starting pillar
    pub start_pillar: Pillar,
    /// Target pillar
    pub target_pillar: Pillar,
}

impl GameState {
    /// Create a new game state with n disks starting on the given pillar
    pub fn new(n: u8, start: Pillar, target: Pillar) -> Self {
        assert!(n >= 1 && n <= 10, "Disk count must be between 1 and 10");

        let mut state = GameState {
            stacks: [Vec::new(), Vec::new(), Vec::new()],
            move_count: 0,
            speed: 0,
            disk_count: n,
            start_pillar: start,
            target_pillar: target,
        };

        // Initialize starting pillar with disks (largest at bottom)
        for disk in (1..=n).rev() {
            state.stacks[start.index()].push(disk);
        }

        state
    }

    /// Push a disk onto a pillar
    pub fn push(&mut self, pillar: Pillar, disk: u8) {
        self.stacks[pillar.index()].push(disk);
    }

    /// Pop a disk from a pillar
    pub fn pop(&mut self, pillar: Pillar) -> Option<u8> {
        self.stacks[pillar.index()].pop()
    }

    /// Get the top disk on a pillar without removing it
    pub fn top(&self, pillar: Pillar) -> Option<u8> {
        self.stacks[pillar.index()].last().copied()
    }

    /// Check if a pillar is empty
    pub fn is_empty(&self, pillar: Pillar) -> bool {
        self.stacks[pillar.index()].is_empty()
    }

    /// Get the number of disks on a pillar
    pub fn count(&self, pillar: Pillar) -> usize {
        self.stacks[pillar.index()].len()
    }

    /// Get a reference to the stack for a pillar
    pub fn stack(&self, pillar: Pillar) -> &[u8] {
        &self.stacks[pillar.index()]
    }

    /// Check if the game is complete (all disks on target pillar)
    pub fn is_complete(&self) -> bool {
        self.count(self.target_pillar) == self.disk_count as usize
    }

    /// Increment move counter
    pub fn increment_moves(&mut self) {
        self.move_count += 1;
    }

    /// Reset the game state
    pub fn reset(&mut self) {
        self.stacks = [Vec::new(), Vec::new(), Vec::new()];
        self.move_count = 0;

        // Re-initialize starting pillar
        for disk in (1..=self.disk_count).rev() {
            self.stacks[self.start_pillar.index()].push(disk);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pillar_from_char() {
        assert_eq!(Pillar::from_char('A'), Some(Pillar::A));
        assert_eq!(Pillar::from_char('a'), Some(Pillar::A));
        assert_eq!(Pillar::from_char('B'), Some(Pillar::B));
        assert_eq!(Pillar::from_char('C'), Some(Pillar::C));
        assert_eq!(Pillar::from_char('D'), None);
    }

    #[test]
    fn test_pillar_to_char() {
        assert_eq!(Pillar::A.to_char(), 'A');
        assert_eq!(Pillar::B.to_char(), 'B');
        assert_eq!(Pillar::C.to_char(), 'C');
    }

    #[test]
    fn test_new_game_state() {
        let state = GameState::new(3, Pillar::A, Pillar::C);
        assert_eq!(state.disk_count, 3);
        assert_eq!(state.move_count, 0);
        assert_eq!(state.count(Pillar::A), 3);
        assert_eq!(state.count(Pillar::B), 0);
        assert_eq!(state.count(Pillar::C), 0);
        assert_eq!(state.stack(Pillar::A), &[3, 2, 1]);
    }

    #[test]
    fn test_push_pop() {
        let mut state = GameState::new(1, Pillar::A, Pillar::C);
        let disk = state.pop(Pillar::A).unwrap();
        assert_eq!(disk, 1);
        assert!(state.is_empty(Pillar::A));

        state.push(Pillar::B, disk);
        assert_eq!(state.count(Pillar::B), 1);
        assert_eq!(state.top(Pillar::B), Some(1));
    }

    #[test]
    fn test_top() {
        let state = GameState::new(3, Pillar::A, Pillar::C);
        assert_eq!(state.top(Pillar::A), Some(1)); // Smallest disk on top
        assert_eq!(state.top(Pillar::B), None);
    }

    #[test]
    fn test_is_empty() {
        let state = GameState::new(3, Pillar::A, Pillar::C);
        assert!(!state.is_empty(Pillar::A));
        assert!(state.is_empty(Pillar::B));
        assert!(state.is_empty(Pillar::C));
    }

    #[test]
    fn test_is_complete() {
        let mut state = GameState::new(2, Pillar::A, Pillar::C);
        assert!(!state.is_complete());

        // Move all disks to target
        state.pop(Pillar::A);
        state.pop(Pillar::A);
        state.push(Pillar::C, 2);
        state.push(Pillar::C, 1);

        assert!(state.is_complete());
    }

    #[test]
    fn test_reset() {
        let mut state = GameState::new(3, Pillar::A, Pillar::C);
        state.pop(Pillar::A);
        state.increment_moves();

        state.reset();
        assert_eq!(state.move_count, 0);
        assert_eq!(state.count(Pillar::A), 3);
        assert_eq!(state.count(Pillar::B), 0);
    }
}
