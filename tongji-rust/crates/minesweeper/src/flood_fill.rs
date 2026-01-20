/// Flood fill algorithm for revealing cells in Minesweeper
use crate::board::Board;

/// Recursively reveal cells starting from (x, y)
///
/// This implements the classic flood fill algorithm:
/// - If the cell has adjacent mines, reveal it and stop
/// - If the cell has no adjacent mines, reveal it and recursively reveal all neighbors
pub fn flood_fill(board: &mut Board, x: usize, y: usize) {
    // Base cases
    if !board.is_valid(x, y) {
        return;
    }

    // Don't reveal if already revealed or flagged
    if board.is_revealed(x, y) || board.is_flagged(x, y) {
        return;
    }

    // Don't reveal if it's a mine
    if board.has_mine(x, y) {
        return;
    }

    // Reveal this cell
    board.reveal(x, y);

    // If this cell has adjacent mines, stop here
    if board.get_count(x, y) > 0 {
        return;
    }

    // Recursively reveal all neighbors (only if count is 0)
    for (nx, ny) in board.get_neighbors(x, y) {
        flood_fill(board, nx, ny);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Difficulty;

    #[test]
    fn test_flood_fill_single_cell() {
        let mut board = Board::new(Difficulty::Easy);
        // Place a mine at (1,1) so (0,0) has count > 0
        board.set_mine(1, 1);
        board.set_count(0, 0, 1);

        flood_fill(&mut board, 0, 0);

        // Only (0,0) should be revealed
        assert!(board.is_revealed(0, 0));
        assert!(!board.is_revealed(1, 0));
    }

    #[test]
    fn test_flood_fill_area() {
        let mut board = Board::new(Difficulty::Easy);
        // No mines, so flood fill should reveal a large area

        // Set all counts to 0 (no adjacent mines)
        for y in 0..board.height {
            for x in 0..board.width {
                board.set_count(x, y, 0);
            }
        }

        flood_fill(&mut board, 4, 4);

        // All cells should be revealed
        for y in 0..board.height {
            for x in 0..board.width {
                assert!(board.is_revealed(x, y), "Cell ({}, {}) not revealed", x, y);
            }
        }
    }

    #[test]
    fn test_flood_fill_stops_at_mines() {
        let mut board = Board::new(Difficulty::Easy);

        // Create a mine boundary
        for x in 0..board.width {
            board.set_mine(x, 4);
        }

        // Set counts for cells adjacent to mines
        for y in 0..board.height {
            for x in 0..board.width {
                if !board.has_mine(x, y) {
                    let count = board
                        .get_neighbors(x, y)
                        .iter()
                        .filter(|(nx, ny)| board.has_mine(*nx, *ny))
                        .count() as u8;
                    board.set_count(x, y, count);
                }
            }
        }

        flood_fill(&mut board, 0, 0);

        // Cells above the mine line should be revealed
        for x in 0..board.width {
            assert!(board.is_revealed(x, 0));
            assert!(board.is_revealed(x, 1));
            assert!(board.is_revealed(x, 2));
            assert!(board.is_revealed(x, 3)); // Adjacent to mines
        }

        // Mine cells should not be revealed
        for x in 0..board.width {
            assert!(!board.is_revealed(x, 4));
        }

        // Cells below mines should not be revealed
        for x in 0..board.width {
            assert!(!board.is_revealed(x, 5));
        }
    }

    #[test]
    fn test_flood_fill_respects_flags() {
        let mut board = Board::new(Difficulty::Easy);

        // Set all counts to 0
        for y in 0..board.height {
            for x in 0..board.width {
                board.set_count(x, y, 0);
            }
        }

        // Flag a cell
        board.toggle_flag(4, 4);

        flood_fill(&mut board, 0, 0);

        // Flagged cell should not be revealed
        assert!(!board.is_revealed(4, 4));
        assert!(board.is_flagged(4, 4));
    }

    #[test]
    fn test_flood_fill_corners() {
        let mut board = Board::new(Difficulty::Easy);

        // Set all counts to 0
        for y in 0..board.height {
            for x in 0..board.width {
                board.set_count(x, y, 0);
            }
        }

        // Test all four corners
        for (x, y) in [(0, 0), (8, 0), (0, 8), (8, 8)] {
            let mut test_board = board.clone();
            flood_fill(&mut test_board, x, y);

            // All cells should be revealed
            assert_eq!(test_board.count_revealed(), 81);
        }
    }

    #[test]
    fn test_flood_fill_already_revealed() {
        let mut board = Board::new(Difficulty::Easy);
        board.set_count(0, 0, 0);

        // Reveal a cell
        board.reveal(0, 0);
        let revealed_before = board.count_revealed();

        // Flood fill on already revealed cell should do nothing
        flood_fill(&mut board, 0, 0);
        assert_eq!(board.count_revealed(), revealed_before);
    }

    #[test]
    fn test_flood_fill_boundaries() {
        let mut board = Board::new(Difficulty::Easy);

        // Create a vertical wall of mines at x=2
        for y in 0..board.height {
            board.set_mine(2, y);
        }

        // Calculate counts
        for y in 0..board.height {
            for x in 0..board.width {
                if !board.has_mine(x, y) {
                    let count = board
                        .get_neighbors(x, y)
                        .iter()
                        .filter(|(nx, ny)| board.has_mine(*nx, *ny))
                        .count() as u8;
                    board.set_count(x, y, count);
                }
            }
        }

        flood_fill(&mut board, 0, 0);

        // Cells on the left side should be revealed up to the boundary
        assert!(board.is_revealed(0, 0));
        assert!(board.is_revealed(1, 0)); // Adjacent to mine wall
        assert!(!board.is_revealed(2, 0)); // Mine
        assert!(!board.is_revealed(3, 0)); // Beyond the wall
    }
}
