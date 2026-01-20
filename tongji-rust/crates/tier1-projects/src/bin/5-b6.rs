// 5-b6: Tower of Hanoi with visualization
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Vec<i32>作为栈，替代固定数组，更安全且符合Rust习惯
// 2. 使用enum Tower代替字符，类型安全且避免无效值
// 3. 使用struct HanoiState封装状态，避免全局变量
// 4. 使用match表达式替代switch，更安全且强制穷尽匹配
// 5. 提取格式化逻辑为独立函数，提高代码复用性
// 6. 使用Result处理输入错误，提供更好的错误信息
// 7. 递归函数使用&mut self，避免全局状态

use std::fmt;
use std::io;

/// 汉诺塔的柱子
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tower {
    A,
    B,
    C,
}

impl Tower {
    /// 从字符解析Tower
    fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Tower::A),
            'B' => Some(Tower::B),
            'C' => Some(Tower::C),
            _ => None,
        }
    }

    /// 计算中间柱（给定起始和目标柱）
    fn middle(start: Tower, end: Tower) -> Tower {
        match (start, end) {
            (Tower::A, Tower::B) | (Tower::B, Tower::A) => Tower::C,
            (Tower::A, Tower::C) | (Tower::C, Tower::A) => Tower::B,
            (Tower::B, Tower::C) | (Tower::C, Tower::B) => Tower::A,
            _ => panic!("起始柱和目标柱不能相同"),
        }
    }
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tower::A => write!(f, "A"),
            Tower::B => write!(f, "B"),
            Tower::C => write!(f, "C"),
        }
    }
}

/// 汉诺塔状态
struct HanoiState {
    /// 三个柱子的栈（Vec作为栈使用，末尾是栈顶）
    towers: [Vec<i32>; 3],
    /// 移动步数计数器
    step_count: usize,
}

impl HanoiState {
    /// 创建新的汉诺塔状态
    fn new(level: i32, start: Tower) -> Self {
        let mut state = HanoiState {
            towers: [Vec::new(), Vec::new(), Vec::new()],
            step_count: 0,
        };

        // Rust改进: 使用迭代器生成初始栈，从大到小
        let disks: Vec<i32> = (1..=level).rev().collect();
        state.towers[start as usize] = disks;

        state
    }

    /// 获取柱子的索引
    fn tower_index(tower: Tower) -> usize {
        tower as usize
    }

    /// 移动一个盘子从src到dst
    fn move_disk(&mut self, src: Tower, dst: Tower) {
        let src_idx = Self::tower_index(src);
        let dst_idx = Self::tower_index(dst);

        // Rust改进: 使用Option的expect提供清晰的错误信息
        let disk = self.towers[src_idx]
            .pop()
            .expect("源柱子不应该为空");

        self.towers[dst_idx].push(disk);
    }

    /// 格式化单个柱子的显示
    fn format_tower(&self, tower: Tower) -> String {
        let idx = Self::tower_index(tower);
        let mut result = String::new();

        // 处理10的特殊对齐
        if let Some(&first) = self.towers[idx].first() {
            if first != 10 {
                result.push(' ');
            }
        }

        for &disk in &self.towers[idx] {
            if disk == 0 {
                result.push_str("  ");
            } else if disk == 10 {
                result.push_str("10 ");
            } else {
                result.push_str(&format!("{} ", disk));
            }
        }

        result
    }

    /// 显示当前状态
    fn display(&self, prefix: &str) {
        print!("{}", prefix);
        print!("A:{}", self.format_tower(Tower::A));
        print!("B:{}", self.format_tower(Tower::B));
        print!("C:{}", self.format_tower(Tower::C));
        println!();
    }

    /// 执行汉诺塔递归算法
    fn hanoi(&mut self, n: i32, src: Tower, tmp: Tower, dst: Tower) {
        if n == 1 {
            self.step_count += 1;
            print!("第{:4} 步({:2}): {}-->{} ", self.step_count, n, src, dst);
            self.move_disk(src, dst);
            self.display("");
        } else {
            // Rust改进: 递归调用更清晰，无需forward declaration
            self.hanoi(n - 1, src, dst, tmp);
            self.step_count += 1;
            print!("第{:4} 步({:2}): {}-->{} ", self.step_count, n, src, dst);
            self.move_disk(src, dst);
            self.display("");
            self.hanoi(n - 1, tmp, src, dst);
        }
    }
}

/// 读取并验证层数（1-10）
fn read_level() -> io::Result<i32> {
    loop {
        println!("请输入汉诺塔的层数(1-10)");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Rust改进: 使用filter验证范围
        if let Some(level) = input.trim().parse::<i32>().ok().filter(|&n| (1..=10).contains(&n)) {
            return Ok(level);
        }
    }
}

/// 读取并验证柱子选择
fn read_tower(prompt: &str, exclude: Option<Tower>) -> io::Result<Tower> {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Some(first_char) = input.trim().chars().next() {
            if let Some(tower) = Tower::from_char(first_char) {
                // 检查是否与排除的柱子相同
                if let Some(excluded) = exclude {
                    if tower == excluded {
                        println!("目标柱不能与起始柱相同");
                        continue;
                    }
                }
                return Ok(tower);
            }
        }
    }
}

fn main() {
    // 读取输入
    let level = read_level().expect("读取层数失败");
    let start = read_tower("请输入起始柱(A-C)", None).expect("读取起始柱失败");
    let end = read_tower("请输入目标柱(A-C)", Some(start)).expect("读取目标柱失败");

    // Rust改进: 使用enum的方法计算中间柱，类型安全
    let mid = Tower::middle(start, end);

    // 创建初始状态并显示
    let mut state = HanoiState::new(level, start);
    state.display("初始:                ");

    // 执行汉诺塔算法
    state.hanoi(level, start, mid, end);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tower_from_char() {
        assert_eq!(Tower::from_char('A'), Some(Tower::A));
        assert_eq!(Tower::from_char('a'), Some(Tower::A));
        assert_eq!(Tower::from_char('B'), Some(Tower::B));
        assert_eq!(Tower::from_char('b'), Some(Tower::B));
        assert_eq!(Tower::from_char('C'), Some(Tower::C));
        assert_eq!(Tower::from_char('c'), Some(Tower::C));
        assert_eq!(Tower::from_char('D'), None);
        assert_eq!(Tower::from_char('1'), None);
    }

    #[test]
    fn test_tower_middle() {
        assert_eq!(Tower::middle(Tower::A, Tower::B), Tower::C);
        assert_eq!(Tower::middle(Tower::B, Tower::A), Tower::C);
        assert_eq!(Tower::middle(Tower::A, Tower::C), Tower::B);
        assert_eq!(Tower::middle(Tower::C, Tower::A), Tower::B);
        assert_eq!(Tower::middle(Tower::B, Tower::C), Tower::A);
        assert_eq!(Tower::middle(Tower::C, Tower::B), Tower::A);
    }

    #[test]
    #[should_panic(expected = "起始柱和目标柱不能相同")]
    fn test_tower_middle_same() {
        Tower::middle(Tower::A, Tower::A);
    }

    #[test]
    fn test_hanoi_state_new() {
        let state = HanoiState::new(3, Tower::A);
        assert_eq!(state.towers[0], vec![3, 2, 1]);
        assert_eq!(state.towers[1], vec![]);
        assert_eq!(state.towers[2], vec![]);
        assert_eq!(state.step_count, 0);
    }

    #[test]
    fn test_hanoi_state_new_different_tower() {
        let state = HanoiState::new(2, Tower::B);
        assert_eq!(state.towers[0], vec![]);
        assert_eq!(state.towers[1], vec![2, 1]);
        assert_eq!(state.towers[2], vec![]);
    }

    #[test]
    fn test_move_disk() {
        let mut state = HanoiState::new(3, Tower::A);
        state.move_disk(Tower::A, Tower::B);

        assert_eq!(state.towers[0], vec![3, 2]);
        assert_eq!(state.towers[1], vec![1]);
        assert_eq!(state.towers[2], vec![]);
    }

    #[test]
    fn test_move_disk_chain() {
        let mut state = HanoiState::new(2, Tower::A);
        state.move_disk(Tower::A, Tower::B);
        state.move_disk(Tower::A, Tower::C);
        state.move_disk(Tower::B, Tower::C);

        assert_eq!(state.towers[0], vec![]);
        assert_eq!(state.towers[1], vec![]);
        assert_eq!(state.towers[2], vec![2, 1]);
    }

    #[test]
    fn test_hanoi_single_disk() {
        let mut state = HanoiState::new(1, Tower::A);
        let mid = Tower::middle(Tower::A, Tower::C);
        state.hanoi(1, Tower::A, mid, Tower::C);

        assert_eq!(state.towers[0], vec![]);
        assert_eq!(state.towers[2], vec![1]);
        assert_eq!(state.step_count, 1);
    }

    #[test]
    fn test_hanoi_two_disks() {
        let mut state = HanoiState::new(2, Tower::A);
        let mid = Tower::middle(Tower::A, Tower::C);
        state.hanoi(2, Tower::A, mid, Tower::C);

        assert_eq!(state.towers[0], vec![]);
        assert_eq!(state.towers[2], vec![2, 1]);
        assert_eq!(state.step_count, 3); // 2^2 - 1 = 3
    }

    #[test]
    fn test_hanoi_three_disks() {
        let mut state = HanoiState::new(3, Tower::A);
        let mid = Tower::middle(Tower::A, Tower::C);
        state.hanoi(3, Tower::A, mid, Tower::C);

        assert_eq!(state.towers[0], vec![]);
        assert_eq!(state.towers[2], vec![3, 2, 1]);
        assert_eq!(state.step_count, 7); // 2^3 - 1 = 7
    }

    #[test]
    fn test_hanoi_step_count_formula() {
        // 验证步数公式: 2^n - 1
        for n in 1..=5 {
            let mut state = HanoiState::new(n, Tower::A);
            let mid = Tower::middle(Tower::A, Tower::C);
            state.hanoi(n, Tower::A, mid, Tower::C);

            let expected_steps = (1 << n) - 1; // 2^n - 1
            assert_eq!(state.step_count, expected_steps as usize);
        }
    }

    #[test]
    fn test_hanoi_different_start_end() {
        // 测试从B到A
        let mut state = HanoiState::new(2, Tower::B);
        let mid = Tower::middle(Tower::B, Tower::A);
        state.hanoi(2, Tower::B, mid, Tower::A);

        assert_eq!(state.towers[0], vec![2, 1]);
        assert_eq!(state.towers[1], vec![]);
        assert_eq!(state.step_count, 3);
    }

    #[test]
    fn test_format_tower_empty() {
        let state = HanoiState::new(3, Tower::A);
        let formatted = state.format_tower(Tower::B);
        assert_eq!(formatted, "");
    }

    #[test]
    fn test_format_tower_with_disks() {
        let state = HanoiState::new(3, Tower::A);
        let formatted = state.format_tower(Tower::A);
        assert!(formatted.contains("3"));
        assert!(formatted.contains("2"));
        assert!(formatted.contains("1"));
    }

    #[test]
    fn test_format_tower_with_ten() {
        let state = HanoiState::new(10, Tower::A);
        let formatted = state.format_tower(Tower::A);
        assert!(formatted.contains("10"));
    }

    #[test]
    fn test_tower_index() {
        assert_eq!(HanoiState::tower_index(Tower::A), 0);
        assert_eq!(HanoiState::tower_index(Tower::B), 1);
        assert_eq!(HanoiState::tower_index(Tower::C), 2);
    }
}
