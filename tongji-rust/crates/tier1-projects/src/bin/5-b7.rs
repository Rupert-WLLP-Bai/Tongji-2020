// 5-b7: Hanoi Tower visualization with console graphics
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用枚举类型Tower代替字符，类型安全且语义清晰
// 2. 使用Vec<Vec<u8>>表示三个柱子的栈，避免全局可变状态
// 3. 将状态封装在HanoiState结构体中，所有权清晰
// 4. 使用console-tools库实现跨平台控制台操作
// 5. 递归函数传递状态引用，避免全局变量
// 6. 使用match表达式替代大量if-else，更清晰

use console_tools::{cct_cls, cct_gotoxy};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// 汉诺塔柱子枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tower {
    A,
    B,
    C,
}

impl Tower {
    /// 从字符转换为Tower
    fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Tower::A),
            'B' => Some(Tower::B),
            'C' => Some(Tower::C),
            _ => None,
        }
    }

    /// 转换为字符
    fn to_char(self) -> char {
        match self {
            Tower::A => 'A',
            Tower::B => 'B',
            Tower::C => 'C',
        }
    }

    /// 计算第三个柱子（给定起始和目标柱子）
    fn get_auxiliary(src: Tower, dst: Tower) -> Tower {
        match (src, dst) {
            (Tower::A, Tower::B) | (Tower::B, Tower::A) => Tower::C,
            (Tower::A, Tower::C) | (Tower::C, Tower::A) => Tower::B,
            (Tower::B, Tower::C) | (Tower::C, Tower::B) => Tower::A,
            _ => panic!("起始柱和目标柱不能相同"),
        }
    }
}

/// 汉诺塔状态
struct HanoiState {
    /// 三个柱子的栈，存储盘子编号（1-10）
    towers: [Vec<u8>; 3],
    /// 移动步数
    step_count: usize,
    /// 延时速度 (0-5)
    speed: u8,
    /// 是否显示内部数组
    show_array: bool,
}

impl HanoiState {
    /// 创建新的汉诺塔状态
    fn new(level: u8, start: Tower, speed: u8, show_array: bool) -> Self {
        let mut state = HanoiState {
            towers: [Vec::new(), Vec::new(), Vec::new()],
            step_count: 0,
            speed,
            show_array,
        };

        // 初始化起始柱子，从大到小放置盘子
        let tower_idx = state.tower_index(start);
        for i in (1..=level).rev() {
            state.towers[tower_idx].push(i);
        }

        state
    }

    /// 获取柱子索引
    fn tower_index(&self, tower: Tower) -> usize {
        match tower {
            Tower::A => 0,
            Tower::B => 1,
            Tower::C => 2,
        }
    }

    /// 移动盘子
    fn move_disk(&mut self, from: Tower, to: Tower) {
        let from_idx = self.tower_index(from);
        let to_idx = self.tower_index(to);

        if let Some(disk) = self.towers[from_idx].pop() {
            self.towers[to_idx].push(disk);
            self.step_count += 1;
        }
    }

    /// 打印当前状态
    fn print_state(&self, from: Tower, to: Tower, disk: u8) {
        print!("({:2}): {}-->{} ", disk, from.to_char(), to.to_char());

        if self.show_array {
            // 打印三个柱子的内部数组
            for (i, tower_name) in ['A', 'B', 'C'].iter().enumerate() {
                print!("{}:", tower_name);
                if !self.towers[i].is_empty() && self.towers[i][0] != 10 {
                    print!(" ");
                }
                for &disk in &self.towers[i] {
                    if disk == 10 {
                        print!("10 ");
                    } else {
                        print!("{} ", disk);
                    }
                }
                // 填充空格
                for _ in 0..(10 - self.towers[i].len()) {
                    print!("  ");
                }
            }
            println!();
        }
    }

    /// 打印垂直显示的柱子
    fn print_vertical(&self) {
        // 打印三个柱子的垂直视图
        for (i, x) in [10, 20, 30].iter().enumerate() {
            let mut y = 11;
            let _ = cct_gotoxy(*x, y);

            // 从底部向上打印
            for j in 0..10 {
                if j < self.towers[i].len() {
                    let disk = self.towers[i][j];
                    if disk == 10 {
                        print!("10");
                    } else {
                        print!(" {}", disk);
                    }
                } else {
                    print!("  ");
                }
                y -= 1;
                let _ = cct_gotoxy(*x, y);
            }
        }
    }

    /// 打印菜单栏
    fn print_menu() {
        let _ = cct_gotoxy(9, 12);
        print!("=========================");
        let _ = cct_gotoxy(9, 13);
        print!("  A         B         C  ");
    }

    /// 等待用户输入或延时
    fn wait(&self) {
        if self.speed == 0 {
            // 按回车继续
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        } else {
            // 延时
            let delay_ms = 800 - (self.speed as u64) * 160;
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }
}

/// 汉诺塔递归求解
fn hanoi(n: u8, src: Tower, aux: Tower, dst: Tower, state: &mut HanoiState) {
    if n == 1 {
        // 移动一个盘子
        state.move_disk(src, dst);
        HanoiState::print_menu();
        let _ = cct_gotoxy(0, 17);
        print!("第{:4} 步", state.step_count);
        state.print_state(src, dst, n);
        state.print_vertical();
        state.wait();
    } else {
        // 递归求解
        hanoi(n - 1, src, dst, aux, state);
        state.move_disk(src, dst);
        HanoiState::print_menu();
        let _ = cct_gotoxy(0, 17);
        print!("第{:4} 步", state.step_count);
        state.print_state(src, dst, n);
        state.print_vertical();
        state.wait();
        hanoi(n - 1, aux, src, dst, state);
    }
}

/// 读取整数输入
fn read_integer(prompt: &str, min: i32, max: i32) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<i32>() {
            if num >= min && num <= max {
                return num;
            }
        }
    }
}

/// 读取柱子输入
fn read_tower(prompt: &str, exclude: Option<Tower>) -> Tower {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(c) = input.trim().chars().next() {
            if let Some(tower) = Tower::from_char(c) {
                if let Some(excluded) = exclude {
                    if tower == excluded {
                        println!(
                            "起始柱({})不能与目标柱({})相同",
                            excluded.to_char(),
                            tower.to_char()
                        );
                        continue;
                    }
                }
                return tower;
            }
        }
    }
}

fn main() {
    // 读取输入参数
    let level = read_integer("请输入汉诺塔的层数(1-10)\n", 1, 10) as u8;
    let start = read_tower("请输入起始柱(A-C)\n", None);
    let end = read_tower("请输入目标柱(A-C)\n", Some(start));
    let speed = read_integer(
        "请输入移动速度(0-5: 0-按回车单步演示 1-延时最长 5-延时最短)\n",
        0,
        5,
    ) as u8;
    let show = read_integer("请输入是否显示内部数组值(0-不显示 1-显示)\n", 0, 1);

    let show_array = show == 1;
    let aux = Tower::get_auxiliary(start, end);

    // 清屏并初始化
    let _ = cct_cls();
    println!(
        "从{}移动到{} ,共{}层 ,延时设置为 {} , {}显示内部数组值",
        start.to_char(),
        end.to_char(),
        level,
        speed,
        if show_array { "" } else { "不" }
    );

    HanoiState::print_menu();
    let _ = cct_gotoxy(0, 17);

    // 创建初始状态
    let mut state = HanoiState::new(level, start, speed, show_array);

    // 打印初始状态
    if show_array {
        print!("初始:                ");
        for (i, tower_name) in ['A', 'B', 'C'].iter().enumerate() {
            print!("{}:", tower_name);
            if !state.towers[i].is_empty() && state.towers[i][0] != 10 {
                print!(" ");
            }
            for &disk in &state.towers[i] {
                if disk == 10 {
                    print!("10 ");
                } else {
                    print!("{} ", disk);
                }
            }
            for _ in 0..(10 - state.towers[i].len()) {
                print!("  ");
            }
        }
        println!();
    }

    state.print_vertical();
    state.wait();

    // 开始求解
    hanoi(level, start, aux, end, &mut state);

    let _ = cct_gotoxy(0, 25);
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
    }

    #[test]
    fn test_tower_to_char() {
        assert_eq!(Tower::A.to_char(), 'A');
        assert_eq!(Tower::B.to_char(), 'B');
        assert_eq!(Tower::C.to_char(), 'C');
    }

    #[test]
    fn test_get_auxiliary() {
        assert_eq!(Tower::get_auxiliary(Tower::A, Tower::B), Tower::C);
        assert_eq!(Tower::get_auxiliary(Tower::A, Tower::C), Tower::B);
        assert_eq!(Tower::get_auxiliary(Tower::B, Tower::C), Tower::A);
        assert_eq!(Tower::get_auxiliary(Tower::B, Tower::A), Tower::C);
        assert_eq!(Tower::get_auxiliary(Tower::C, Tower::A), Tower::B);
        assert_eq!(Tower::get_auxiliary(Tower::C, Tower::B), Tower::A);
    }

    #[test]
    fn test_hanoi_state_new() {
        let state = HanoiState::new(3, Tower::A, 1, false);
        assert_eq!(state.towers[0], vec![3, 2, 1]);
        assert_eq!(state.towers[1], vec![]);
        assert_eq!(state.towers[2], vec![]);
        assert_eq!(state.step_count, 0);
    }

    #[test]
    fn test_move_disk() {
        let mut state = HanoiState::new(3, Tower::A, 1, false);
        state.move_disk(Tower::A, Tower::B);
        assert_eq!(state.towers[0], vec![3, 2]);
        assert_eq!(state.towers[1], vec![1]);
        assert_eq!(state.step_count, 1);
    }

    #[test]
    fn test_tower_index() {
        let state = HanoiState::new(1, Tower::A, 1, false);
        assert_eq!(state.tower_index(Tower::A), 0);
        assert_eq!(state.tower_index(Tower::B), 1);
        assert_eq!(state.tower_index(Tower::C), 2);
    }

    #[test]
    fn test_hanoi_moves_count() {
        // 测试汉诺塔移动次数是否正确 (2^n - 1)
        for level in 1..=5 {
            let mut state = HanoiState::new(level, Tower::A, 5, false);
            hanoi(level, Tower::A, Tower::B, Tower::C, &mut state);
            let expected_moves = (1 << level) - 1; // 2^n - 1
            assert_eq!(state.step_count, expected_moves as usize);
        }
    }

    #[test]
    fn test_hanoi_final_state() {
        // 测试汉诺塔最终状态是否正确
        let level = 4;
        let mut state = HanoiState::new(level, Tower::A, 5, false);
        hanoi(level, Tower::A, Tower::B, Tower::C, &mut state);

        // 最终所有盘子应该在目标柱上
        assert_eq!(state.towers[0], vec![]);
        assert_eq!(state.towers[1], vec![]);
        assert_eq!(state.towers[2], vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_hanoi_different_start_end() {
        // 测试不同起始和目标柱
        let level = 3;
        let mut state = HanoiState::new(level, Tower::B, 5, false);
        hanoi(level, Tower::B, Tower::C, Tower::A, &mut state);

        assert_eq!(state.towers[0], vec![3, 2, 1]);
        assert_eq!(state.towers[1], vec![]);
        assert_eq!(state.towers[2], vec![]);
    }
}
