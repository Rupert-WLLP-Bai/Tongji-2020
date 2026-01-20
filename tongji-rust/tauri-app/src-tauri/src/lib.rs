use minesweeper::{Board, CellState as RustCellState, Difficulty, RevealResult};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum GameDifficulty {
    Easy,
    Medium,
    Hard,
}

impl From<GameDifficulty> for Difficulty {
    fn from(d: GameDifficulty) -> Self {
        match d {
            GameDifficulty::Easy => Difficulty::Easy,
            GameDifficulty::Medium => Difficulty::Medium,
            GameDifficulty::Hard => Difficulty::Hard,
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CellInfo {
    pub has_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub adjacent_mines: u8,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameStateInfo {
    pub board: Vec<Vec<CellInfo>>,
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
    pub flag_count: usize,
    pub game_over: bool,
    pub game_won: bool,
}

pub struct MinesweeperState {
    pub board: Mutex<Option<Board>>,
    pub game_over: Mutex<bool>,
    pub game_won: Mutex<bool>,
}

#[tauri::command]
fn init_minesweeper(
    state: tauri::State<MinesweeperState>,
    difficulty: GameDifficulty,
) -> GameStateInfo {
    let mut board_lock = state.board.lock().unwrap();
    let mut over_lock = state.game_over.lock().unwrap();
    let mut won_lock = state.game_won.lock().unwrap();

    let board = Board::new(Difficulty::from(difficulty));
    let info = get_game_info(&board, false, false);

    *board_lock = Some(board);
    *over_lock = false;
    *won_lock = false;

    info
}

#[tauri::command]
fn handle_click(
    state: tauri::State<MinesweeperState>,
    x: usize,
    y: usize,
) -> Option<GameStateInfo> {
    let mut board_lock = state.board.lock().unwrap();
    let mut over_lock = state.game_over.lock().unwrap();
    let mut won_lock = state.game_won.lock().unwrap();

    if *over_lock || *won_lock {
        return board_lock
            .as_ref()
            .map(|b| get_game_info(b, *over_lock, *won_lock));
    }

    if let Some(board) = board_lock.as_mut() {
        if !board.are_mines_generated() {
            minesweeper::mine_gen::generate_mines(board, x, y);
            board.mark_mines_generated();
        }

        match minesweeper::validation::try_reveal(board, x, y) {
            RevealResult::Safe => {
                if board.get_count(x, y) == 0 {
                    for (nx, ny) in board.get_neighbors(x, y) {
                        minesweeper::flood_fill::flood_fill(board, nx, ny);
                    }
                }
                if minesweeper::validation::is_win(board) {
                    *won_lock = true;
                }
            }
            RevealResult::Mine => {
                *over_lock = true;
            }
            _ => {}
        }

        Some(get_game_info(board, *over_lock, *won_lock))
    } else {
        None
    }
}

#[tauri::command]
fn handle_flag(state: tauri::State<MinesweeperState>, x: usize, y: usize) -> Option<GameStateInfo> {
    let mut board_lock = state.board.lock().unwrap();
    let over_lock = state.game_over.lock().unwrap();
    let won_lock = state.game_won.lock().unwrap();

    if *over_lock || *won_lock {
        return board_lock
            .as_ref()
            .map(|b| get_game_info(b, *over_lock, *won_lock));
    }

    if let Some(board) = board_lock.as_mut() {
        board.toggle_flag(x, y);
        Some(get_game_info(board, *over_lock, *won_lock))
    } else {
        None
    }
}

#[tauri::command]
fn auto_solve_step(state: tauri::State<MinesweeperState>) -> Option<GameStateInfo> {
    let mut board_lock = state.board.lock().unwrap();
    let mut over_lock = state.game_over.lock().unwrap();
    let mut won_lock = state.game_won.lock().unwrap();

    if let Some(board) = board_lock.as_mut() {
        if *over_lock || *won_lock {
            return Some(get_game_info(board, *over_lock, *won_lock));
        }

        if !board.are_mines_generated() {
            let x = board.width / 2;
            let y = board.height / 2;
            minesweeper::mine_gen::generate_mines(board, x, y);
            board.mark_mines_generated();
            minesweeper::validation::try_reveal(board, x, y);
            if board.get_count(x, y) == 0 {
                for (nx, ny) in board.get_neighbors(x, y) {
                    minesweeper::flood_fill::flood_fill(board, nx, ny);
                }
            }
            return Some(get_game_info(board, *over_lock, *won_lock));
        }

        let mut made_logical_progress = false;

        // Phase 1: Basic logic (existing logic)
        'outer: for y in 0..board.height {
            for x in 0..board.width {
                if !board.is_revealed(x, y) || board.get_count(x, y) == 0 {
                    continue;
                }

                let neighbors = board.get_neighbors(x, y);
                let mut flagged = 0;
                let mut hidden = Vec::new();

                for (nx, ny) in neighbors {
                    if board.is_flagged(nx, ny) {
                        flagged += 1;
                    } else if !board.is_revealed(nx, ny) {
                        hidden.push((nx, ny));
                    }
                }

                if hidden.is_empty() {
                    continue;
                }

                // Rule 1: If all remaining mines are flagged, reveal all hidden cells
                if flagged == board.get_count(x, y) {
                    for (hx, hy) in hidden {
                        minesweeper::validation::try_reveal(board, hx, hy);
                        if board.get_count(hx, hy) == 0 {
                            for (nx, ny) in board.get_neighbors(hx, hy) {
                                minesweeper::flood_fill::flood_fill(board, nx, ny);
                            }
                        }
                    }
                    made_logical_progress = true;
                    break 'outer;
                }
                // Rule 2: If hidden + flagged == mine count, flag all hidden cells
                else if (hidden.len() + flagged as usize) == board.get_count(x, y) as usize {
                    for (hx, hy) in hidden {
                        board.toggle_flag(hx, hy);
                    }
                    made_logical_progress = true;
                    break 'outer;
                }
            }
        }

        // Phase 2: Set subtraction logic (advanced reasoning)
        if !made_logical_progress {
            'subtraction: for y_a in 0..board.height {
                for x_a in 0..board.width {
                    // Cell A must be revealed and have a number
                    if !board.is_revealed(x_a, y_a) || board.get_count(x_a, y_a) == 0 {
                        continue;
                    }

                    // Get S_A (unknown neighbors of A) and R_A (remaining mines for A)
                    let neighbors_a = board.get_neighbors(x_a, y_a);
                    let mut s_a = Vec::new();
                    let mut flagged_a = 0;

                    for (nx, ny) in &neighbors_a {
                        if board.is_flagged(*nx, *ny) {
                            flagged_a += 1;
                        } else if !board.is_revealed(*nx, *ny) {
                            s_a.push((*nx, *ny));
                        }
                    }

                    if s_a.is_empty() {
                        continue;
                    }

                    let r_a = board.get_count(x_a, y_a) as i32 - flagged_a;

                    // Check all neighbors of A for potential cell B
                    for (x_b, y_b) in &neighbors_a {
                        // Cell B must be revealed and have a number
                        if !board.is_revealed(*x_b, *y_b) || board.get_count(*x_b, *y_b) == 0 {
                            continue;
                        }

                        // Get S_B (unknown neighbors of B) and R_B (remaining mines for B)
                        let neighbors_b = board.get_neighbors(*x_b, *y_b);
                        let mut s_b = Vec::new();
                        let mut flagged_b = 0;

                        for (nx, ny) in &neighbors_b {
                            if board.is_flagged(*nx, *ny) {
                                flagged_b += 1;
                            } else if !board.is_revealed(*nx, *ny) {
                                s_b.push((*nx, *ny));
                            }
                        }

                        if s_b.is_empty() {
                            continue;
                        }

                        let r_b = board.get_count(*x_b, *y_b) as i32 - flagged_b;

                        // Check if S_A is a subset of S_B
                        let s_a_is_subset = s_a.iter().all(|cell| s_b.contains(cell));

                        if !s_a_is_subset {
                            continue;
                        }

                        // Calculate S_B - S_A (difference set)
                        let s_diff: Vec<(usize, usize)> = s_b
                            .iter()
                            .filter(|cell| !s_a.contains(cell))
                            .copied()
                            .collect();

                        if s_diff.is_empty() {
                            continue;
                        }

                        let mine_diff = r_b - r_a;
                        let diff_count = s_diff.len() as i32;

                        // Subtraction logic: Flag cells
                        if mine_diff == diff_count && mine_diff > 0 {
                            for (hx, hy) in s_diff {
                                board.toggle_flag(hx, hy);
                            }
                            made_logical_progress = true;
                            break 'subtraction;
                        }
                        // Subtraction logic: Reveal safe cells
                        else if mine_diff == 0 && diff_count > 0 {
                            for (hx, hy) in s_diff {
                                minesweeper::validation::try_reveal(board, hx, hy);
                                if board.get_count(hx, hy) == 0 {
                                    for (nx, ny) in board.get_neighbors(hx, hy) {
                                        minesweeper::flood_fill::flood_fill(board, nx, ny);
                                    }
                                }
                            }
                            made_logical_progress = true;
                            break 'subtraction;
                        }
                    }
                }
            }
        }

        if !made_logical_progress {
            let mut hidden_cells = Vec::new();
            for y in 0..board.height {
                for x in 0..board.width {
                    if !board.is_revealed(x, y) && !board.is_flagged(x, y) {
                        hidden_cells.push((x, y));
                    }
                }
            }

            if !hidden_cells.is_empty() {
                let idx = rand::random::<usize>() % hidden_cells.len();
                let (gx, gy) = hidden_cells[idx];
                match minesweeper::validation::try_reveal(board, gx, gy) {
                    RevealResult::Safe => {
                        if board.get_count(gx, gy) == 0 {
                            for (nx, ny) in board.get_neighbors(gx, gy) {
                                minesweeper::flood_fill::flood_fill(board, nx, ny);
                            }
                        }
                    }
                    RevealResult::Mine => {
                        *over_lock = true;
                    }
                    _ => {}
                }
            }
        }

        if minesweeper::validation::is_win(board) {
            *won_lock = true;
        }

        let current_over = *over_lock;
        let current_won = *won_lock;
        Some(get_game_info(board, current_over, current_won))
    } else {
        None
    }
}

fn get_game_info(board: &Board, game_over: bool, game_won: bool) -> GameStateInfo {
    let mut cells = Vec::with_capacity(board.height);
    let reveal_all = game_over || game_won;
    for y in 0..board.height {
        let mut row = Vec::with_capacity(board.width);
        for x in 0..board.width {
            let state = board.get_state(x, y);
            let is_mine = board.has_mine(x, y);
            let revealed = board.is_revealed(x, y) || (reveal_all && is_mine);

            row.push(CellInfo {
                has_mine: is_mine,
                is_revealed: revealed,
                is_flagged: state == RustCellState::Flagged,
                adjacent_mines: board.get_count(x, y),
            });
        }
        cells.push(row);
    }

    GameStateInfo {
        board: cells,
        width: board.width,
        height: board.height,
        mine_count: board.mine_count,
        flag_count: board.flag_count,
        game_over,
        game_won,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(MinesweeperState {
            board: Mutex::new(None),
            game_over: Mutex::new(false),
            game_won: Mutex::new(false),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init_minesweeper,
            handle_click,
            handle_flag,
            auto_solve_step
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
