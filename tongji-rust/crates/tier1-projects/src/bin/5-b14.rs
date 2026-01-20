// 5-b14: Minesweeper validator - validate mine grid and neighbor counts
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Vec<Vec<Cell>>替代多维数组，更灵活且类型安全
// 2. 使用enum Cell表示格子状态，比bool+char更清晰
// 3. 使用Result<(), String>返回错误信息，比返回-1/-2更明确
// 4. 提取验证逻辑为独立函数，每个函数单一职责
// 5. 使用迭代器和filter，避免手动索引越界检查
// 6. 边界处理使用checked_sub避免溢出，更安全

use std::io::{self, BufRead};

/// 扫雷格子类型
#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Mine,           // 地雷 '*'
    Number(u8),     // 数字 0-8
}

/// 扫雷网格
#[derive(Debug)]
struct MineGrid {
    grid: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl MineGrid {
    /// 从输入行创建扫雷网格
    ///
    /// # Arguments
    /// * `lines` - 输入的10行字符串
    ///
    /// # Returns
    /// * `Result<MineGrid, String>` - 成功返回网格，失败返回错误信息
    ///
    /// # Rust改进
    /// 使用Result类型明确表示可能的错误，比C的返回-1更清晰
    fn from_lines(lines: &[String]) -> Result<Self, String> {
        if lines.len() != 10 {
            return Err(format!("需要10行输入，实际{}行", lines.len()));
        }

        let mut grid = Vec::new();

        for line in lines {
            // Rust改进: 使用filter自动过滤空格和换行符，比C的手动循环更简洁
            let row: Vec<Cell> = line
                .chars()
                .filter(|&ch| ch != ' ' && ch != '\n')
                .map(|ch| match ch {
                    '*' => Cell::Mine,
                    '0'..='8' => Cell::Number(ch as u8 - b'0'),
                    _ => Cell::Number(0), // 其他字符当作0处理
                })
                .collect();

            if row.len() != 26 {
                return Err(format!("每行应有26个有效字符，实际{}个", row.len()));
            }

            grid.push(row);
        }

        Ok(MineGrid {
            grid,
            rows: 10,
            cols: 26,
        })
    }

    /// 获取指定位置的格子
    fn get(&self, row: usize, col: usize) -> Option<Cell> {
        self.grid.get(row).and_then(|r| r.get(col).copied())
    }

    /// 检查是否为地雷
    fn is_mine(&self, row: usize, col: usize) -> bool {
        matches!(self.get(row, col), Some(Cell::Mine))
    }

    /// 计算周围8个格子的地雷数量
    ///
    /// # Rust改进
    /// 1. 使用checked_sub避免usize下溢，比C的手动边界检查更安全
    /// 2. 使用迭代器和filter_map，比手动累加更函数式
    fn count_adjacent_mines(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        // 检查周围8个方向
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue; // 跳过自己
                }

                // Rust改进: 使用checked_add/checked_sub安全处理边界
                let new_row = if dr < 0 {
                    row.checked_sub(1)
                } else {
                    Some(row + dr as usize)
                };

                let new_col = if dc < 0 {
                    col.checked_sub(1)
                } else {
                    Some(col + dc as usize)
                };

                if let (Some(r), Some(c)) = (new_row, new_col) {
                    if r < self.rows && c < self.cols && self.is_mine(r, c) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// 验证地雷总数是否为50
    fn validate_mine_count(&self) -> Result<(), String> {
        let mine_count = self
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == Cell::Mine)
            .count();

        if mine_count != 50 {
            Err(format!("地雷数量应为50，实际为{}", mine_count))
        } else {
            Ok(())
        }
    }

    /// 验证所有非地雷格子的数字是否正确
    ///
    /// # Rust改进
    /// 使用enumerate和模式匹配，比C的嵌套循环更清晰
    fn validate_numbers(&self) -> Result<(), String> {
        for (row, row_cells) in self.grid.iter().enumerate() {
            for (col, &cell) in row_cells.iter().enumerate() {
                if let Cell::Number(num) = cell {
                    let expected = self.count_adjacent_mines(row, col);
                    if num != expected {
                        return Err(format!(
                            "位置({},{})数字错误: 期望{}, 实际{}",
                            row, col, expected, num
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// 完整验证扫雷网格
    ///
    /// # Rust改进
    /// 使用?操作符链式传播错误，比C的多个if判断更简洁
    fn validate(&self) -> Result<(), String> {
        self.validate_mine_count()?;
        self.validate_numbers()?;
        Ok(())
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::new();

    // 读取10行输入
    for i in 1..=10 {
        let mut line = String::new();
        if let Err(e) = stdin.lock().read_line(&mut line) {
            eprintln!("读取第{}行失败: {}", i, e);
            return;
        }
        lines.push(line);
    }

    // Rust改进: 使用match处理Result，比C的if-else更清晰
    match MineGrid::from_lines(&lines) {
        Ok(grid) => match grid.validate() {
            Ok(()) => println!("正确"),
            Err(_) => println!("错误"),
        },
        Err(_) => println!("错误"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn create_test_grid(data: &[&str]) -> MineGrid {
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        MineGrid::from_lines(&lines).unwrap()
    }

    #[test]
    fn test_cell_equality() {
        assert_eq!(Cell::Mine, Cell::Mine);
        assert_eq!(Cell::Number(3), Cell::Number(3));
        assert_ne!(Cell::Mine, Cell::Number(0));
        assert_ne!(Cell::Number(1), Cell::Number(2));
    }

    #[test]
    fn test_from_lines_invalid_row_count() {
        let lines: Vec<String> = vec!["test".to_string()];
        let result = MineGrid::from_lines(&lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_lines_invalid_col_count() {
        let lines: Vec<String> = vec!["abc".to_string(); 10];
        let result = MineGrid::from_lines(&lines);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_lines_with_spaces() {
        // 测试空格被正确过滤
        let lines: Vec<String> = vec![
            "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0".to_string();
            10
        ];
        let result = MineGrid::from_lines(&lines);
        assert!(result.is_ok());
        let grid = result.unwrap();
        assert_eq!(grid.rows, 10);
        assert_eq!(grid.cols, 26);
    }

    #[test]
    fn test_is_mine() {
        let lines: Vec<String> = vec![
            "*0000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        assert!(grid.is_mine(0, 0));
        assert!(!grid.is_mine(0, 1));
        assert!(!grid.is_mine(1, 0));
    }

    #[test]
    fn test_count_adjacent_mines_corner() {
        // 测试左上角
        let lines: Vec<String> = vec![
            "*1000000000000000000000000".to_string(),
            "11000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        assert_eq!(grid.count_adjacent_mines(0, 1), 1); // 右边有1个地雷
        assert_eq!(grid.count_adjacent_mines(1, 0), 1); // 上边有1个地雷
        assert_eq!(grid.count_adjacent_mines(1, 1), 1); // 左上有1个地雷
    }

    #[test]
    fn test_count_adjacent_mines_center() {
        // 测试中心位置
        let lines: Vec<String> = vec![
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "0000*1*0000000000000000000".to_string(),
            "00001110000000000000000000".to_string(),
            "0000*1*0000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        // 中心位置(5,5)周围有4个地雷
        assert_eq!(grid.count_adjacent_mines(5, 5), 4);
    }

    #[test]
    fn test_count_adjacent_mines_all_mines() {
        // 测试周围全是地雷
        let lines: Vec<String> = vec![
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "0000***0000000000000000000".to_string(),
            "0000*8*0000000000000000000".to_string(),
            "0000***0000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        assert_eq!(grid.count_adjacent_mines(5, 5), 8);
    }

    #[test]
    fn test_validate_mine_count_too_few() {
        // 只有1个地雷，应该失败
        let lines: Vec<String> = vec![
            "*0000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        assert!(grid.validate_mine_count().is_err());
    }

    #[test]
    fn test_validate_numbers_correct() {
        // 简单的正确案例
        let lines: Vec<String> = vec![
            "*1000000000000000000000000".to_string(),
            "11000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        assert!(grid.validate_numbers().is_ok());
    }

    #[test]
    fn test_validate_numbers_incorrect() {
        // 数字错误的案例
        let lines: Vec<String> = vec![
            "*9000000000000000000000000".to_string(), // 应该是1，不是9
            "11000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        assert!(grid.validate_numbers().is_err());
    }

    #[test]
    fn test_get_out_of_bounds() {
        let lines: Vec<String> = vec!["00000000000000000000000000".to_string(); 10];
        let grid = MineGrid::from_lines(&lines).unwrap();
        assert_eq!(grid.get(100, 100), None);
        assert_eq!(grid.get(0, 100), None);
        assert_eq!(grid.get(100, 0), None);
    }

    #[test]
    fn test_count_adjacent_mines_edge() {
        // 测试边缘位置
        let lines: Vec<String> = vec![
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "00000000000000000000000000".to_string(),
            "*100000000000000000000000*".to_string(),
        ];
        let grid = MineGrid::from_lines(&lines).unwrap();
        // 左下角和右下角各有1个地雷
        assert_eq!(grid.count_adjacent_mines(9, 1), 1);
        assert_eq!(grid.count_adjacent_mines(9, 24), 1);
    }
}
