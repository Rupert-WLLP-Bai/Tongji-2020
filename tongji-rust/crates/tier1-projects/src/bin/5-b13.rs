// 5-b13: Minesweeper grid generator
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Vec<Vec<bool>>代替固定大小数组，更灵活且类型安全
// 2. 使用HashSet确保不重复放置地雷，避免while循环的随机性能问题
// 3. 使用迭代器和函数式编程风格，代码更简洁
// 4. 提取核心逻辑为独立函数，便于单元测试
// 5. 使用const定义网格尺寸，便于修改和维护
// 6. 边界检查使用checked_sub避免溢出，更安全

use rand::Rng;
use std::collections::HashSet;

const ROWS: usize = 10;
const COLS: usize = 26;
const MINE_COUNT: usize = 50;

/// 生成随机地雷位置
///
/// # Arguments
/// * `rows` - 网格行数
/// * `cols` - 网格列数
/// * `count` - 地雷数量
///
/// # Returns
/// * `HashSet<(usize, usize)>` - 地雷位置集合
///
/// # Rust改进
/// 使用HashSet自动去重，避免C++版本中while循环可能的性能问题
fn generate_mines(rows: usize, cols: usize, count: usize) -> HashSet<(usize, usize)> {
    let mut rng = rand::thread_rng();
    let mut mines = HashSet::new();

    // Rust改进: HashSet自动处理重复，比while循环更高效
    while mines.len() < count {
        let row = rng.gen_range(0..rows);
        let col = rng.gen_range(0..cols);
        mines.insert((row, col));
    }

    mines
}

/// 计算指定位置周围的地雷数量
///
/// # Arguments
/// * `row` - 行索引
/// * `col` - 列索引
/// * `mines` - 地雷位置集合
/// * `rows` - 总行数
/// * `cols` - 总列数
///
/// # Returns
/// * `usize` - 周围地雷数量
///
/// # Rust改进
/// 1. 使用checked_sub避免usize下溢，更安全
/// 2. 使用迭代器和filter统计，代码更简洁
/// 3. 边界检查更清晰，避免数组越界
fn count_adjacent_mines(
    row: usize,
    col: usize,
    mines: &HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
) -> usize {
    let mut count = 0;

    // Rust改进: 使用saturating_sub安全处理边界，避免usize下溢
    let row_start = row.saturating_sub(1);
    let col_start = col.saturating_sub(1);
    let row_end = (row + 2).min(rows);
    let col_end = (col + 2).min(cols);

    // 遍历周围8个位置
    for r in row_start..row_end {
        for c in col_start..col_end {
            // 跳过中心位置
            if r == row && c == col {
                continue;
            }
            if mines.contains(&(r, c)) {
                count += 1;
            }
        }
    }

    count
}

/// 生成扫雷网格
///
/// # Arguments
/// * `rows` - 网格行数
/// * `cols` - 网格列数
/// * `mine_count` - 地雷数量
///
/// # Returns
/// * `(HashSet<(usize, usize)>, Vec<Vec<usize>>)` - 地雷位置和数字网格
///
/// # Rust改进
/// 返回元组而不是修改参数，更符合函数式编程风格
fn generate_minesweeper_grid(
    rows: usize,
    cols: usize,
    mine_count: usize,
) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let mines = generate_mines(rows, cols, mine_count);

    // Rust改进: 使用vec!宏初始化，比C++的数组初始化更简洁
    let mut grid = vec![vec![0; cols]; rows];

    // 计算每个位置周围的地雷数
    // Rust改进: 使用enumerate()避免手动索引
    for (row, row_data) in grid.iter_mut().enumerate() {
        for (col, cell) in row_data.iter_mut().enumerate() {
            if !mines.contains(&(row, col)) {
                *cell = count_adjacent_mines(row, col, &mines, rows, cols);
            }
        }
    }

    (mines, grid)
}

/// 打印扫雷网格
///
/// # Arguments
/// * `mines` - 地雷位置集合
/// * `grid` - 数字网格
///
/// # Rust改进
/// 使用迭代器和enumerate，代码更简洁
fn print_grid(mines: &HashSet<(usize, usize)>, grid: &[Vec<usize>]) {
    for (row, row_data) in grid.iter().enumerate() {
        for (col, &count) in row_data.iter().enumerate() {
            if mines.contains(&(row, col)) {
                print!("* ");
            } else {
                print!("{} ", count);
            }
        }
        println!();
    }
}

fn main() {
    let (mines, grid) = generate_minesweeper_grid(ROWS, COLS, MINE_COUNT);
    print_grid(&mines, &grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_mines_count() {
        // 测试生成的地雷数量正确
        let mines = generate_mines(10, 26, 50);
        assert_eq!(mines.len(), 50);
    }

    #[test]
    fn test_generate_mines_within_bounds() {
        // 测试地雷位置在边界内
        let mines = generate_mines(10, 26, 50);
        for (row, col) in mines {
            assert!(row < 10);
            assert!(col < 26);
        }
    }

    #[test]
    fn test_count_adjacent_mines_corner() {
        // 测试角落位置的地雷计数
        let mut mines = HashSet::new();
        mines.insert((0, 1));
        mines.insert((1, 0));
        mines.insert((1, 1));

        let count = count_adjacent_mines(0, 0, &mines, 10, 26);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_adjacent_mines_center() {
        // 测试中心位置的地雷计数
        let mut mines = HashSet::new();
        // 在(5,5)周围放置8个地雷
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr != 0 || dc != 0 {
                    mines.insert(((5_i32 + dr) as usize, (5_i32 + dc) as usize));
                }
            }
        }

        let count = count_adjacent_mines(5, 5, &mines, 10, 26);
        assert_eq!(count, 8);
    }

    #[test]
    fn test_count_adjacent_mines_no_mines() {
        // 测试没有地雷的情况
        let mines = HashSet::new();
        let count = count_adjacent_mines(5, 5, &mines, 10, 26);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_count_adjacent_mines_edge() {
        // 测试边缘位置
        let mut mines = HashSet::new();
        mines.insert((0, 0));
        mines.insert((0, 2));
        mines.insert((1, 1));

        let count = count_adjacent_mines(0, 1, &mines, 10, 26);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_generate_grid_structure() {
        // 测试生成的网格结构正确
        let (mines, grid) = generate_minesweeper_grid(10, 26, 50);

        assert_eq!(mines.len(), 50);
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 26);
    }

    #[test]
    fn test_generate_grid_mine_positions_zero() {
        // 测试地雷位置的数字为0（不计算）
        let (mines, grid) = generate_minesweeper_grid(10, 26, 50);

        for (row, col) in &mines {
            assert_eq!(grid[*row][*col], 0);
        }
    }

    #[test]
    fn test_generate_grid_counts_valid() {
        // 测试非地雷位置的数字在有效范围内(0-8)
        let (mines, grid) = generate_minesweeper_grid(10, 26, 50);

        for (row, row_data) in grid.iter().enumerate() {
            for (col, &count) in row_data.iter().enumerate() {
                if !mines.contains(&(row, col)) {
                    assert!(count <= 8, "Count at ({}, {}) is {}", row, col, count);
                }
            }
        }
    }

    #[test]
    fn test_small_grid() {
        // 测试小网格
        let (mines, grid) = generate_minesweeper_grid(3, 3, 2);

        assert_eq!(mines.len(), 2);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0].len(), 3);
    }

    #[test]
    fn test_count_adjacent_mines_boundary_check() {
        // 测试边界检查的正确性
        let mut mines = HashSet::new();
        mines.insert((0, 0));

        // 测试左上角
        let count = count_adjacent_mines(0, 0, &mines, 10, 26);
        assert_eq!(count, 0); // 自己不算

        // 测试右下角附近
        let count = count_adjacent_mines(9, 25, &mines, 10, 26);
        assert_eq!(count, 0);
    }
}
