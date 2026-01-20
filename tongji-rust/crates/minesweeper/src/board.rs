/// Board state management for Minesweeper

/// Difficulty levels for Minesweeper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,   // 9×9, 10 mines
    Medium, // 16×16, 40 mines
    Hard,   // 16×30, 99 mines
}

impl Difficulty {
    /// Get board dimensions (width, height)
    pub fn dimensions(self) -> (usize, usize) {
        match self {
            Difficulty::Easy => (9, 9),
            Difficulty::Medium => (16, 16),
            Difficulty::Hard => (30, 16),
        }
    }

    /// Get number of mines
    pub fn mine_count(self) -> usize {
        match self {
            Difficulty::Easy => 10,
            Difficulty::Medium => 40,
            Difficulty::Hard => 99,
        }
    }
}

/// Cell state in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

/// Minesweeper board
#[derive(Debug, Clone)]
pub struct Board {
    /// Board dimensions
    pub width: usize,
    pub height: usize,
    /// Mine positions (true = mine, false = no mine)
    mines: Vec<Vec<bool>>,
    /// Adjacent mine counts for each cell
    counts: Vec<Vec<u8>>,
    /// Cell visibility state
    states: Vec<Vec<CellState>>,
    /// Number of mines
    pub mine_count: usize,
    /// Number of flags placed
    pub flag_count: usize,
    /// Whether mines have been generated
    mines_generated: bool,
}

impl Board {
    /// Create a new board with the given difficulty
    pub fn new(difficulty: Difficulty) -> Self {
        let (width, height) = difficulty.dimensions();
        let mine_count = difficulty.mine_count();

        Board {
            width,
            height,
            mines: vec![vec![false; width]; height],
            counts: vec![vec![0; width]; height],
            states: vec![vec![CellState::Hidden; width]; height],
            mine_count,
            flag_count: 0,
            mines_generated: false,
        }
    }

    /// Create a custom board
    pub fn custom(width: usize, height: usize, mine_count: usize) -> Self {
        Board {
            width,
            height,
            mines: vec![vec![false; width]; height],
            counts: vec![vec![0; width]; height],
            states: vec![vec![CellState::Hidden; width]; height],
            mine_count,
            flag_count: 0,
            mines_generated: false,
        }
    }

    /// Check if coordinates are valid
    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Check if a cell has a mine
    pub fn has_mine(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.mines[y][x]
    }

    /// Set a mine at the given position
    pub fn set_mine(&mut self, x: usize, y: usize) {
        if self.is_valid(x, y) {
            self.mines[y][x] = true;
        }
    }

    /// Get the adjacent mine count for a cell
    pub fn get_count(&self, x: usize, y: usize) -> u8 {
        if self.is_valid(x, y) {
            self.counts[y][x]
        } else {
            0
        }
    }

    /// Set the adjacent mine count for a cell
    pub fn set_count(&mut self, x: usize, y: usize, count: u8) {
        if self.is_valid(x, y) {
            self.counts[y][x] = count;
        }
    }

    /// Get the state of a cell
    pub fn get_state(&self, x: usize, y: usize) -> CellState {
        if self.is_valid(x, y) {
            self.states[y][x]
        } else {
            CellState::Hidden
        }
    }

    /// Set the state of a cell
    pub fn set_state(&mut self, x: usize, y: usize, state: CellState) {
        if self.is_valid(x, y) {
            self.states[y][x] = state;
        }
    }

    /// Reveal a cell
    pub fn reveal(&mut self, x: usize, y: usize) {
        if self.is_valid(x, y) && self.states[y][x] == CellState::Hidden {
            self.states[y][x] = CellState::Revealed;
        }
    }

    /// Toggle flag on a cell
    pub fn toggle_flag(&mut self, x: usize, y: usize) -> bool {
        if !self.is_valid(x, y) {
            return false;
        }

        match self.states[y][x] {
            CellState::Hidden => {
                self.states[y][x] = CellState::Flagged;
                self.flag_count += 1;
                true
            }
            CellState::Flagged => {
                self.states[y][x] = CellState::Hidden;
                self.flag_count -= 1;
                true
            }
            CellState::Revealed => false,
        }
    }

    /// Check if a cell is revealed
    pub fn is_revealed(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.states[y][x] == CellState::Revealed
    }

    /// Check if a cell is flagged
    pub fn is_flagged(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.states[y][x] == CellState::Flagged
    }

    /// Check if a cell is hidden
    pub fn is_hidden(&self, x: usize, y: usize) -> bool {
        self.is_valid(x, y) && self.states[y][x] == CellState::Hidden
    }

    /// Mark that mines have been generated
    pub fn mark_mines_generated(&mut self) {
        self.mines_generated = true;
    }

    /// Check if mines have been generated
    pub fn are_mines_generated(&self) -> bool {
        self.mines_generated
    }

    /// Count revealed cells
    pub fn count_revealed(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.states[y][x] == CellState::Revealed {
                    count += 1;
                }
            }
        }
        count
    }

    /// Get neighbors of a cell (up to 8 surrounding cells)
    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let x_i = x as isize;
        let y_i = y as isize;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x_i + dx;
                let ny = y_i + dy;

                if nx >= 0 && ny >= 0 {
                    let nx_u = nx as usize;
                    let ny_u = ny as usize;
                    if self.is_valid(nx_u, ny_u) {
                        neighbors.push((nx_u, ny_u));
                    }
                }
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_dimensions() {
        assert_eq!(Difficulty::Easy.dimensions(), (9, 9));
        assert_eq!(Difficulty::Medium.dimensions(), (16, 16));
        assert_eq!(Difficulty::Hard.dimensions(), (30, 16));
    }

    #[test]
    fn test_difficulty_mine_count() {
        assert_eq!(Difficulty::Easy.mine_count(), 10);
        assert_eq!(Difficulty::Medium.mine_count(), 40);
        assert_eq!(Difficulty::Hard.mine_count(), 99);
    }

    #[test]
    fn test_new_board() {
        let board = Board::new(Difficulty::Easy);
        assert_eq!(board.width, 9);
        assert_eq!(board.height, 9);
        assert_eq!(board.mine_count, 10);
        assert_eq!(board.flag_count, 0);
        assert!(!board.are_mines_generated());
    }

    #[test]
    fn test_is_valid() {
        let board = Board::new(Difficulty::Easy);
        assert!(board.is_valid(0, 0));
        assert!(board.is_valid(8, 8));
        assert!(!board.is_valid(9, 0));
        assert!(!board.is_valid(0, 9));
    }

    #[test]
    fn test_mine_operations() {
        let mut board = Board::new(Difficulty::Easy);
        assert!(!board.has_mine(0, 0));

        board.set_mine(0, 0);
        assert!(board.has_mine(0, 0));
    }

    #[test]
    fn test_count_operations() {
        let mut board = Board::new(Difficulty::Easy);
        assert_eq!(board.get_count(0, 0), 0);

        board.set_count(0, 0, 3);
        assert_eq!(board.get_count(0, 0), 3);
    }

    #[test]
    fn test_state_operations() {
        let mut board = Board::new(Difficulty::Easy);
        assert_eq!(board.get_state(0, 0), CellState::Hidden);

        board.reveal(0, 0);
        assert_eq!(board.get_state(0, 0), CellState::Revealed);
        assert!(board.is_revealed(0, 0));
    }

    #[test]
    fn test_toggle_flag() {
        let mut board = Board::new(Difficulty::Easy);
        assert_eq!(board.flag_count, 0);

        // Flag a cell
        assert!(board.toggle_flag(0, 0));
        assert!(board.is_flagged(0, 0));
        assert_eq!(board.flag_count, 1);

        // Unflag the cell
        assert!(board.toggle_flag(0, 0));
        assert!(board.is_hidden(0, 0));
        assert_eq!(board.flag_count, 0);

        // Cannot flag revealed cell
        board.reveal(1, 1);
        assert!(!board.toggle_flag(1, 1));
    }

    #[test]
    fn test_get_neighbors() {
        let board = Board::new(Difficulty::Easy);

        // Corner cell (0,0) has 3 neighbors
        let neighbors = board.get_neighbors(0, 0);
        assert_eq!(neighbors.len(), 3);

        // Center cell (4,4) has 8 neighbors
        let neighbors = board.get_neighbors(4, 4);
        assert_eq!(neighbors.len(), 8);

        // Edge cell (0,4) has 5 neighbors
        let neighbors = board.get_neighbors(0, 4);
        assert_eq!(neighbors.len(), 5);
    }

    #[test]
    fn test_count_revealed() {
        let mut board = Board::new(Difficulty::Easy);
        assert_eq!(board.count_revealed(), 0);

        board.reveal(0, 0);
        board.reveal(1, 1);
        assert_eq!(board.count_revealed(), 2);
    }
}
