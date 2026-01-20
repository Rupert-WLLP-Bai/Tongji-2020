/// Mine generation for Minesweeper
use crate::board::Board;
use rand::Rng;

/// Generate mines on the board, ensuring the first click is safe
///
/// # Arguments
/// * `board` - The game board
/// * `safe_x` - X coordinate of the safe zone center (first click)
/// * `safe_y` - Y coordinate of the safe zone center (first click)
///
/// The safe zone includes the clicked cell and all 8 surrounding cells.
pub fn generate_mines(board: &mut Board, safe_x: usize, safe_y: usize) {
    let mut rng = rand::thread_rng();
    let mut placed = 0;

    // Generate safe zone coordinates (3×3 area around first click)
    let safe_zone: Vec<(usize, usize)> = {
        let mut zone = vec![(safe_x, safe_y)];
        zone.extend(board.get_neighbors(safe_x, safe_y));
        zone
    };

    // Place mines randomly, avoiding safe zone
    while placed < board.mine_count {
        let x = rng.gen_range(0..board.width);
        let y = rng.gen_range(0..board.height);

        // Skip if already has mine or in safe zone
        if board.has_mine(x, y) || safe_zone.contains(&(x, y)) {
            continue;
        }

        board.set_mine(x, y);
        placed += 1;
    }

    // Calculate adjacent mine counts for all cells
    calculate_counts(board);
    board.mark_mines_generated();
}

/// Calculate adjacent mine counts for all cells
fn calculate_counts(board: &mut Board) {
    for y in 0..board.height {
        for x in 0..board.width {
            if !board.has_mine(x, y) {
                let count = count_adjacent_mines(board, x, y);
                board.set_count(x, y, count);
            }
        }
    }
}

/// Count mines adjacent to a cell
fn count_adjacent_mines(board: &Board, x: usize, y: usize) -> u8 {
    let neighbors = board.get_neighbors(x, y);
    neighbors
        .iter()
        .filter(|(nx, ny)| board.has_mine(*nx, *ny))
        .count() as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Difficulty;

    #[test]
    fn test_generate_mines_count() {
        let mut board = Board::new(Difficulty::Easy);
        generate_mines(&mut board, 4, 4);

        // Count actual mines placed
        let mut mine_count = 0;
        for y in 0..board.height {
            for x in 0..board.width {
                if board.has_mine(x, y) {
                    mine_count += 1;
                }
            }
        }

        assert_eq!(mine_count, board.mine_count);
        assert!(board.are_mines_generated());
    }

    #[test]
    fn test_generate_mines_safe_zone() {
        let mut board = Board::new(Difficulty::Easy);
        let safe_x = 4;
        let safe_y = 4;

        generate_mines(&mut board, safe_x, safe_y);

        // Check that safe zone has no mines
        assert!(!board.has_mine(safe_x, safe_y));
        for (nx, ny) in board.get_neighbors(safe_x, safe_y) {
            assert!(
                !board.has_mine(nx, ny),
                "Mine found in safe zone at ({}, {})",
                nx,
                ny
            );
        }
    }

    #[test]
    fn test_generate_mines_corner_safe_zone() {
        let mut board = Board::new(Difficulty::Easy);
        generate_mines(&mut board, 0, 0);

        // Corner safe zone should also be mine-free
        assert!(!board.has_mine(0, 0));
        for (nx, ny) in board.get_neighbors(0, 0) {
            assert!(!board.has_mine(nx, ny));
        }
    }

    #[test]
    fn test_calculate_counts() {
        let mut board = Board::new(Difficulty::Easy);

        // Manually place mines in a pattern
        board.set_mine(1, 1);
        board.set_mine(2, 1);

        calculate_counts(&mut board);

        // Cell (0,0) should have 1 adjacent mine
        assert_eq!(board.get_count(0, 0), 1);

        // Cell (1,0) should have 2 adjacent mines
        assert_eq!(board.get_count(1, 0), 2);

        // Cell (2,0) should have 2 adjacent mines
        assert_eq!(board.get_count(2, 0), 2);

        // Cell (3,0) should have 1 adjacent mine
        assert_eq!(board.get_count(3, 0), 1);
    }

    #[test]
    fn test_count_adjacent_mines() {
        let mut board = Board::new(Difficulty::Easy);

        // Place mines around (1,1)
        board.set_mine(0, 0);
        board.set_mine(1, 0);
        board.set_mine(2, 0);

        assert_eq!(count_adjacent_mines(&board, 1, 1), 3);
        assert_eq!(count_adjacent_mines(&board, 0, 1), 2);
        assert_eq!(count_adjacent_mines(&board, 3, 1), 1);
    }

    #[test]
    fn test_generate_mines_all_difficulties() {
        for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard] {
            let mut board = Board::new(difficulty);
            generate_mines(&mut board, 0, 0);

            let mut mine_count = 0;
            for y in 0..board.height {
                for x in 0..board.width {
                    if board.has_mine(x, y) {
                        mine_count += 1;
                    }
                }
            }

            assert_eq!(mine_count, difficulty.mine_count());
        }
    }
}
