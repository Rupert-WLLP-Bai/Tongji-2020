// 5-b9: Sudoku validator - validates if a 9x9 matrix is a valid Sudoku solution
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用类型别名和常量提高代码可读性
// 2. 使用HashSet检测重复，比bool数组更符合Rust习惯
// 3. 使用迭代器和函数式编程，避免嵌套循环
// 4. 提取验证逻辑为独立函数，便于单元测试
// 5. 使用Result类型处理输入错误，更清晰的错误处理
// 6. 使用数组的chunks方法处理3x3宫格，更简洁

use std::collections::HashSet;
use std::io::{self, Write};

/// 数独网格类型别名
type SudokuGrid = [[u8; 9]; 9];

/// 数独大小常量
const GRID_SIZE: usize = 9;
const BLOCK_SIZE: usize = 3;

/// 验证数独解是否有效
///
/// # Arguments
/// * `grid` - 9x9的数独网格
///
/// # Returns
/// * `true` - 如果是有效的数独解
/// * `false` - 如果不是有效的数独解
///
/// # Examples
/// ```
/// let valid_grid = [
///     [5,3,4,6,7,8,9,1,2],
///     [6,7,2,1,9,5,3,4,8],
///     [1,9,8,3,4,2,5,6,7],
///     [8,5,9,7,6,1,4,2,3],
///     [4,2,6,8,5,3,7,9,1],
///     [7,1,3,9,2,4,8,5,6],
///     [9,6,1,5,3,7,2,8,4],
///     [2,8,7,4,1,9,6,3,5],
///     [3,4,5,2,8,6,1,7,9],
/// ];
/// assert!(validate_sudoku(&valid_grid));
/// ```
fn validate_sudoku(grid: &SudokuGrid) -> bool {
    // Rust改进: 使用all()方法和迭代器，比嵌套循环更简洁
    // 验证所有行、列和3x3宫格
    (0..GRID_SIZE)
        .all(|i| validate_row(grid, i) && validate_column(grid, i) && validate_block(grid, i))
}

/// 验证指定行是否包含1-9所有数字且无重复
///
/// # Arguments
/// * `grid` - 数独网格
/// * `row` - 行索引(0-8)
fn validate_row(grid: &SudokuGrid, row: usize) -> bool {
    // Rust改进: 使用HashSet检测重复，比bool数组更符合Rust习惯
    let mut seen = HashSet::new();
    grid[row]
        .iter()
        .all(|&num| num >= 1 && num <= 9 && seen.insert(num))
}

/// 验证指定列是否包含1-9所有数字且无重复
///
/// # Arguments
/// * `grid` - 数独网格
/// * `col` - 列索引(0-8)
fn validate_column(grid: &SudokuGrid, col: usize) -> bool {
    let mut seen = HashSet::new();
    (0..GRID_SIZE).all(|row| {
        let num = grid[row][col];
        num >= 1 && num <= 9 && seen.insert(num)
    })
}

/// 验证指定3x3宫格是否包含1-9所有数字且无重复
///
/// # Arguments
/// * `grid` - 数独网格
/// * `block_idx` - 宫格索引(0-8)，从左到右、从上到下编号
fn validate_block(grid: &SudokuGrid, block_idx: usize) -> bool {
    // Rust改进: 计算宫格起始位置，使用迭代器遍历
    let start_row = (block_idx / BLOCK_SIZE) * BLOCK_SIZE;
    let start_col = (block_idx % BLOCK_SIZE) * BLOCK_SIZE;

    let mut seen = HashSet::new();
    (start_row..start_row + BLOCK_SIZE).all(|row| {
        (start_col..start_col + BLOCK_SIZE).all(|col| {
            let num = grid[row][col];
            num >= 1 && num <= 9 && seen.insert(num)
        })
    })
}

/// 从标准输入读取一个1-9之间的整数
///
/// # Arguments
/// * `row` - 行号(从1开始)
/// * `col` - 列号(从1开始)
///
/// # Returns
/// * `u8` - 读取到的有效数字
fn read_number(row: usize, col: usize) -> u8 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Rust改进: 使用链式调用和filter验证范围
        if let Some(num) = input
            .trim()
            .parse::<u8>()
            .ok()
            .filter(|&n| n >= 1 && n <= 9)
        {
            return num;
        }

        println!("请重新输入第{}行{}列(行列均从1开始计数)的值", row, col);
    }
}

/// 从标准输入读取9x9数独网格
///
/// # Returns
/// * `SudokuGrid` - 读取到的数独网格
fn read_sudoku_grid() -> SudokuGrid {
    println!("请输入9*9的矩阵，值为1-9之间");

    let mut grid = [[0u8; 9]; 9];

    // Rust改进: 使用enumerate()获取索引，避免手动计数
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = read_number(i + 1, j + 1);
        }
    }

    grid
}

fn main() {
    let grid = read_sudoku_grid();

    // Rust改进: 直接使用if表达式，无需中间变量
    if validate_sudoku(&grid) {
        println!("是数独的解");
    } else {
        println!("不是数独的解");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 创建一个有效的数独解用于测试
    fn create_valid_sudoku() -> SudokuGrid {
        [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ]
    }

    #[test]
    fn test_valid_sudoku() {
        let grid = create_valid_sudoku();
        assert!(validate_sudoku(&grid));
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_row() {
        let mut grid = create_valid_sudoku();
        // 在第一行创建重复
        grid[0][0] = grid[0][1];
        assert!(!validate_sudoku(&grid));
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_column() {
        let mut grid = create_valid_sudoku();
        // 在第一列创建重复
        grid[0][0] = grid[1][0];
        assert!(!validate_sudoku(&grid));
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_block() {
        let mut grid = create_valid_sudoku();
        // 在左上角3x3宫格创建重复
        grid[0][0] = grid[1][1];
        assert!(!validate_sudoku(&grid));
    }

    #[test]
    fn test_invalid_sudoku_out_of_range() {
        let mut grid = create_valid_sudoku();
        // 使用超出范围的值
        grid[0][0] = 0;
        assert!(!validate_sudoku(&grid));

        grid[0][0] = 10;
        assert!(!validate_sudoku(&grid));
    }

    #[test]
    fn test_validate_row() {
        let grid = create_valid_sudoku();
        // 所有行都应该有效
        for i in 0..GRID_SIZE {
            assert!(validate_row(&grid, i));
        }
    }

    #[test]
    fn test_validate_column() {
        let grid = create_valid_sudoku();
        // 所有列都应该有效
        for i in 0..GRID_SIZE {
            assert!(validate_column(&grid, i));
        }
    }

    #[test]
    fn test_validate_block() {
        let grid = create_valid_sudoku();
        // 所有3x3宫格都应该有效
        for i in 0..GRID_SIZE {
            assert!(validate_block(&grid, i));
        }
    }

    #[test]
    fn test_validate_block_positions() {
        // 测试宫格索引计算是否正确
        let grid = create_valid_sudoku();

        // 左上角宫格(0): 行0-2, 列0-2
        assert!(validate_block(&grid, 0));

        // 中间宫格(4): 行3-5, 列3-5
        assert!(validate_block(&grid, 4));

        // 右下角宫格(8): 行6-8, 列6-8
        assert!(validate_block(&grid, 8));
    }

    #[test]
    fn test_invalid_row_missing_number() {
        let mut grid = create_valid_sudoku();
        // 让第一行缺少数字9，有两个1
        grid[0][8] = 1;
        assert!(!validate_row(&grid, 0));
    }

    #[test]
    fn test_invalid_column_missing_number() {
        let mut grid = create_valid_sudoku();
        // 让第一列缺少数字9，有两个1
        grid[8][0] = 1;
        assert!(!validate_column(&grid, 0));
    }

    #[test]
    fn test_all_different_invalid_sudokus() {
        // 测试各种无效的数独
        let invalid_cases = vec![
            // 第一行有重复
            [
                [1, 1, 3, 4, 5, 6, 7, 8, 9],
                [4, 5, 6, 7, 8, 9, 1, 2, 3],
                [7, 8, 9, 1, 2, 3, 4, 5, 6],
                [2, 3, 4, 5, 6, 7, 8, 9, 1],
                [5, 6, 7, 8, 9, 1, 2, 3, 4],
                [8, 9, 1, 2, 3, 4, 5, 6, 7],
                [3, 4, 5, 6, 7, 8, 9, 1, 2],
                [6, 7, 8, 9, 1, 2, 3, 4, 5],
                [9, 2, 2, 3, 4, 5, 6, 7, 8],
            ],
        ];

        for grid in invalid_cases {
            assert!(!validate_sudoku(&grid));
        }
    }
}
