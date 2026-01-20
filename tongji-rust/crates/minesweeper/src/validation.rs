/// Validation logic for Minesweeper
use crate::board::Board;

/// Result of revealing a cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevealResult {
    /// Cell revealed successfully
    Safe,
    /// Revealed a mine - game over
    Mine,
    /// Cell was already revealed
    AlreadyRevealed,
    /// Cell is flagged, cannot reveal
    Flagged,
}

/// Check if the player has won
///
/// Win condition: All non-mine cells are revealed
pub fn is_win(board: &Board) -> bool {
    let total_cells = board.width * board.height;
    let revealed = board.count_revealed();
    let mines = board.mine_count;

    revealed + mines == total_cells
}

/// Check if the player has lost by revealing a mine
pub fn is_loss(board: &Board, x: usize, y: usize) -> bool {
    board.has_mine(x, y) && board.is_revealed(x, y)
}

/// Attempt to reveal a cell
pub fn try_reveal(board: &mut Board, x: usize, y: usize) -> RevealResult {
    if !board.is_valid(x, y) {
        return RevealResult::AlreadyRevealed;
    }

    if board.is_revealed(x, y) {
        return RevealResult::AlreadyRevealed;
    }

    if board.is_flagged(x, y) {
        return RevealResult::Flagged;
    }

    if board.has_mine(x, y) {
        board.reveal(x, y);
        return RevealResult::Mine;
    }

    board.reveal(x, y);
    RevealResult::Safe
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Difficulty;

    #[test]
    fn test_is_win_empty_board() {
        let board = Board::new(Difficulty::Easy);
        assert!(!is_win(&board));
    }

    #[test]
    fn test_is_win_all_revealed() {
        let mut board = Board::new(Difficulty::Easy);

        // Place exactly 10 mines (matching the mine_count)
        // Place them in first row and second row
        for i in 0..9 {
            board.set_mine(i, 0);
        }
        board.set_mine(0, 1);
        board.mark_mines_generated();

        // Reveal all non-mine cells
        for y in 0..board.height {
            for x in 0..board.width {
                if !board.has_mine(x, y) {
                    board.reveal(x, y);
                }
            }
        }

        assert!(is_win(&board));
    }

    #[test]
    fn test_is_win_partial_reveal() {
        let mut board = Board::new(Difficulty::Easy);

        // Place mines
        for i in 0..10 {
            board.set_mine(i, 0);
        }

        // Reveal only some cells
        board.reveal(0, 1);
        board.reveal(1, 1);

        assert!(!is_win(&board));
    }

    #[test]
    fn test_is_loss_no_mine() {
        let mut board = Board::new(Difficulty::Easy);
        board.reveal(0, 0);

        assert!(!is_loss(&board, 0, 0));
    }

    #[test]
    fn test_is_loss_mine_revealed() {
        let mut board = Board::new(Difficulty::Easy);
        board.set_mine(0, 0);
        board.reveal(0, 0);

        assert!(is_loss(&board, 0, 0));
    }

    #[test]
    fn test_is_loss_mine_not_revealed() {
        let mut board = Board::new(Difficulty::Easy);
        board.set_mine(0, 0);

        assert!(!is_loss(&board, 0, 0));
    }

    #[test]
    fn test_try_reveal_safe() {
        let mut board = Board::new(Difficulty::Easy);
        assert_eq!(try_reveal(&mut board, 0, 0), RevealResult::Safe);
        assert!(board.is_revealed(0, 0));
    }

    #[test]
    fn test_try_reveal_mine() {
        let mut board = Board::new(Difficulty::Easy);
        board.set_mine(0, 0);

        assert_eq!(try_reveal(&mut board, 0, 0), RevealResult::Mine);
        assert!(board.is_revealed(0, 0));
    }

    #[test]
    fn test_try_reveal_already_revealed() {
        let mut board = Board::new(Difficulty::Easy);
        board.reveal(0, 0);

        assert_eq!(try_reveal(&mut board, 0, 0), RevealResult::AlreadyRevealed);
    }

    #[test]
    fn test_try_reveal_flagged() {
        let mut board = Board::new(Difficulty::Easy);
        board.toggle_flag(0, 0);

        assert_eq!(try_reveal(&mut board, 0, 0), RevealResult::Flagged);
        assert!(!board.is_revealed(0, 0));
    }

    #[test]
    fn test_win_condition_with_flags() {
        let mut board = Board::new(Difficulty::Easy);

        // Place exactly 10 mines (matching the mine_count)
        // Place them in first row and second row
        for i in 0..9 {
            board.set_mine(i, 0);
            board.toggle_flag(i, 0); // Flag the mines
        }
        board.set_mine(0, 1);
        board.toggle_flag(0, 1);
        board.mark_mines_generated();

        // Reveal all non-mine cells
        for y in 0..board.height {
            for x in 0..board.width {
                if !board.has_mine(x, y) {
                    board.reveal(x, y);
                }
            }
        }

        // Should win even if mines are flagged but not revealed
        assert!(is_win(&board));
    }
}
