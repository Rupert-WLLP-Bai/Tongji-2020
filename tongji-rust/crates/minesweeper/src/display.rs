/// Display functions for Minesweeper
use crate::board::{Board, CellState};

/// Display the board in text mode
pub fn display_board(board: &Board, show_mines: bool) {
    println!(
        "\n   {}",
        (0..board.width)
            .map(|i| format!("{:2}", i))
            .collect::<Vec<_>>()
            .join(" ")
    );

    for y in 0..board.height {
        print!("{:2} ", y);
        for x in 0..board.width {
            let cell = match board.get_state(x, y) {
                CellState::Hidden => {
                    if show_mines && board.has_mine(x, y) {
                        " * "
                    } else {
                        " . "
                    }
                }
                CellState::Revealed => {
                    if board.has_mine(x, y) {
                        " X "
                    } else {
                        let count = board.get_count(x, y);
                        if count == 0 {
                            "   "
                        } else {
                            &format!(" {} ", count)
                        }
                    }
                }
                CellState::Flagged => " F ",
            };
            print!("{}", cell);
        }
        println!();
    }

    println!("\nFlags: {}/{}", board.flag_count, board.mine_count);
    println!(
        "Revealed: {}/{}",
        board.count_revealed(),
        board.width * board.height - board.mine_count
    );
}

/// Display game statistics
pub fn display_stats(board: &Board) {
    println!("\n=== Game Statistics ===");
    println!("Board size: {}×{}", board.width, board.height);
    println!("Total mines: {}", board.mine_count);
    println!("Flags placed: {}", board.flag_count);
    println!("Cells revealed: {}", board.count_revealed());
    println!(
        "Cells remaining: {}",
        board.width * board.height - board.mine_count - board.count_revealed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Difficulty;

    #[test]
    fn test_display_functions() {
        let mut board = Board::new(Difficulty::Easy);
        board.set_mine(0, 0);
        board.reveal(1, 1);
        board.toggle_flag(2, 2);

        // These functions should not panic
        display_board(&board, false);
        display_board(&board, true);
        display_stats(&board);
    }
}
