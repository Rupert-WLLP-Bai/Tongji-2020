// 4-b7: Tower of Hanoi (汉诺塔) using recursion
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用enum表示柱子，类型安全避免无效柱子名称
// 2. 使用Result处理输入验证，避免多层嵌套循环
// 3. 使用match表达式替代复杂的if-else，更清晰
// 4. 提取递归逻辑为独立函数，便于单元测试
// 5. 使用Vec收集移动步骤，便于测试验证
// 6. 实现Display trait提供友好的输出格式
// 7. 使用TryFrom trait实现安全的字符到枚举转换

use std::fmt;
use std::io::{self, Write};

/// 汉诺塔的柱子
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tower {
    A,
    B,
    C,
}

impl Tower {
    /// 计算中间柱（给定起始柱和目标柱）
    ///
    /// # Arguments
    /// * `src` - 起始柱
    /// * `dst` - 目标柱
    ///
    /// # Returns
    /// * `Tower` - 中间柱
    fn middle(src: Tower, dst: Tower) -> Tower {
        // Rust改进: 使用match穷举所有情况，编译器保证完整性
        match (src, dst) {
            (Tower::A, Tower::B) | (Tower::B, Tower::A) => Tower::C,
            (Tower::A, Tower::C) | (Tower::C, Tower::A) => Tower::B,
            (Tower::B, Tower::C) | (Tower::C, Tower::B) => Tower::A,
            // 起始柱和目标柱相同的情况不应该发生
            _ => unreachable!("Source and destination towers should not be the same"),
        }
    }
}

// Rust改进: 实现Display trait提供友好的输出
impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tower::A => write!(f, "A"),
            Tower::B => write!(f, "B"),
            Tower::C => write!(f, "C"),
        }
    }
}

// Rust改进: 实现TryFrom trait进行安全的字符转换
impl TryFrom<char> for Tower {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_uppercase() {
            'A' => Ok(Tower::A),
            'B' => Ok(Tower::B),
            'C' => Ok(Tower::C),
            _ => Err(format!("无效的柱子名称: {}", c)),
        }
    }
}

/// 移动记录
#[derive(Debug, Clone, PartialEq, Eq)]
struct Move {
    disk: u32,
    from: Tower,
    to: Tower,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}# {}-->{}", self.disk, self.from, self.to)
    }
}

/// 汉诺塔递归求解
///
/// # Arguments
/// * `n` - 盘子数量
/// * `src` - 起始柱
/// * `tmp` - 中间柱
/// * `dst` - 目标柱
/// * `moves` - 存储移动步骤的向量
///
/// # Algorithm
/// 1. 如果只有一个盘子，直接移动
/// 2. 否则：
///    - 将n-1个盘子从src移动到tmp（使用dst作为中间柱）
///    - 将第n个盘子从src移动到dst
///    - 将n-1个盘子从tmp移动到dst（使用src作为中间柱）
fn hanoi(n: u32, src: Tower, tmp: Tower, dst: Tower, moves: &mut Vec<Move>) {
    if n == 1 {
        // 基础情况: 只有一个盘子，直接移动
        moves.push(Move {
            disk: n,
            from: src,
            to: dst,
        });
    } else {
        // 递归情况:
        // 1. 将n-1个盘子从src移到tmp
        hanoi(n - 1, src, dst, tmp, moves);
        // 2. 将第n个盘子从src移到dst
        moves.push(Move {
            disk: n,
            from: src,
            to: dst,
        });
        // 3. 将n-1个盘子从tmp移到dst
        hanoi(n - 1, tmp, src, dst, moves);
    }
}

/// 汉诺塔求解（返回移动步骤）
///
/// # Arguments
/// * `n` - 盘子数量
/// * `src` - 起始柱
/// * `dst` - 目标柱
///
/// # Returns
/// * `Vec<Move>` - 移动步骤列表
fn solve_hanoi(n: u32, src: Tower, dst: Tower) -> Vec<Move> {
    let mut moves = Vec::new();
    let tmp = Tower::middle(src, dst);
    hanoi(n, src, tmp, dst, &mut moves);
    moves
}

/// 从标准输入读取汉诺塔层数（1-16）
fn read_level() -> io::Result<u32> {
    loop {
        print!("请输入汉诺塔的层数(1-16)\n");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Rust改进: 使用match + if guard进行范围检查
        match input.trim().parse::<u32>() {
            Ok(level) if (1..=16).contains(&level) => return Ok(level),
            Ok(_) => continue,  // 超出范围，继续循环
            Err(_) => continue, // 解析失败，继续循环
        }
    }
}

/// 从标准输入读取柱子名称
fn read_tower(prompt: &str, exclude: Option<Tower>) -> io::Result<Tower> {
    loop {
        print!("{}\n", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input.len() != 1 {
            continue;
        }

        // Rust改进: 使用TryFrom trait进行转换
        if let Ok(tower) = Tower::try_from(input.chars().next().unwrap()) {
            // 检查是否与排除的柱子相同
            if let Some(excluded) = exclude {
                if tower == excluded {
                    println!("起始柱不能与目标柱相同");
                    continue;
                }
            }
            return Ok(tower);
        }
    }
}

fn main() {
    // 读取层数
    let level = match read_level() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("读取层数失败: {}", e);
            return;
        }
    };

    // 读取起始柱
    let start = match read_tower("请输入起始柱(A-C)", None) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("读取起始柱失败: {}", e);
            return;
        }
    };

    // 读取目标柱（不能与起始柱相同）
    let end = match read_tower("请输入目标柱(A-C)", Some(start)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("读取目标柱失败: {}", e);
            return;
        }
    };

    // 求解并输出
    let moves = solve_hanoi(level, start, end);
    for m in moves {
        println!("{}", m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tower_middle() {
        // 测试中间柱计算
        assert_eq!(Tower::middle(Tower::A, Tower::B), Tower::C);
        assert_eq!(Tower::middle(Tower::A, Tower::C), Tower::B);
        assert_eq!(Tower::middle(Tower::B, Tower::C), Tower::A);
        // 测试对称性
        assert_eq!(Tower::middle(Tower::B, Tower::A), Tower::C);
        assert_eq!(Tower::middle(Tower::C, Tower::A), Tower::B);
        assert_eq!(Tower::middle(Tower::C, Tower::B), Tower::A);
    }

    #[test]
    fn test_tower_try_from() {
        // 测试大写字母
        assert_eq!(Tower::try_from('A'), Ok(Tower::A));
        assert_eq!(Tower::try_from('B'), Ok(Tower::B));
        assert_eq!(Tower::try_from('C'), Ok(Tower::C));

        // 测试小写字母（应该转换为大写）
        assert_eq!(Tower::try_from('a'), Ok(Tower::A));
        assert_eq!(Tower::try_from('b'), Ok(Tower::B));
        assert_eq!(Tower::try_from('c'), Ok(Tower::C));

        // 测试无效输入
        assert!(Tower::try_from('D').is_err());
        assert!(Tower::try_from('1').is_err());
        assert!(Tower::try_from(' ').is_err());
    }

    #[test]
    fn test_hanoi_one_disk() {
        // 一个盘子：只需一步
        let moves = solve_hanoi(1, Tower::A, Tower::C);
        assert_eq!(moves.len(), 1);
        assert_eq!(
            moves[0],
            Move {
                disk: 1,
                from: Tower::A,
                to: Tower::C
            }
        );
    }

    #[test]
    fn test_hanoi_two_disks() {
        // 两个盘子：需要3步
        let moves = solve_hanoi(2, Tower::A, Tower::C);
        assert_eq!(moves.len(), 3);

        // 验证移动序列
        assert_eq!(
            moves[0],
            Move {
                disk: 1,
                from: Tower::A,
                to: Tower::B
            }
        );
        assert_eq!(
            moves[1],
            Move {
                disk: 2,
                from: Tower::A,
                to: Tower::C
            }
        );
        assert_eq!(
            moves[2],
            Move {
                disk: 1,
                from: Tower::B,
                to: Tower::C
            }
        );
    }

    #[test]
    fn test_hanoi_three_disks() {
        // 三个盘子：需要7步
        let moves = solve_hanoi(3, Tower::A, Tower::C);
        assert_eq!(moves.len(), 7);

        // 验证第一步和最后一步
        assert_eq!(moves[0].disk, 1);
        assert_eq!(moves[0].from, Tower::A);

        assert_eq!(moves[6].disk, 1);
        assert_eq!(moves[6].to, Tower::C);
    }

    #[test]
    fn test_hanoi_move_count() {
        // 验证移动次数公式: 2^n - 1
        for n in 1..=10 {
            let moves = solve_hanoi(n, Tower::A, Tower::C);
            let expected_count = (1 << n) - 1; // 2^n - 1
            assert_eq!(moves.len(), expected_count as usize, "Failed for n={}", n);
        }
    }

    #[test]
    fn test_hanoi_validity() {
        // 验证移动的有效性：较大的盘子不能放在较小的盘子上
        let moves = solve_hanoi(4, Tower::A, Tower::C);

        // 模拟三个柱子的状态
        let mut towers: [Vec<u32>; 3] = [vec![4, 3, 2, 1], vec![], vec![]];

        for m in moves {
            let from_idx = match m.from {
                Tower::A => 0,
                Tower::B => 1,
                Tower::C => 2,
            };
            let to_idx = match m.to {
                Tower::A => 0,
                Tower::B => 1,
                Tower::C => 2,
            };

            // 从源柱取出盘子
            let disk = towers[from_idx]
                .pop()
                .expect("Source tower should not be empty");
            assert_eq!(disk, m.disk, "Disk number mismatch");

            // 检查目标柱是否可以放置（目标柱为空或顶部盘子更大）
            if let Some(&top) = towers[to_idx].last() {
                assert!(
                    disk < top,
                    "Cannot place larger disk {} on smaller disk {}",
                    disk,
                    top
                );
            }

            // 放置盘子到目标柱
            towers[to_idx].push(disk);
        }

        // 验证最终状态：所有盘子都在目标柱C上
        assert!(towers[0].is_empty(), "Tower A should be empty");
        assert!(towers[1].is_empty(), "Tower B should be empty");
        assert_eq!(
            towers[2],
            vec![4, 3, 2, 1],
            "All disks should be on Tower C"
        );
    }

    #[test]
    fn test_hanoi_different_towers() {
        // 测试不同的起始和目标柱组合
        let test_cases = [
            (Tower::A, Tower::B),
            (Tower::A, Tower::C),
            (Tower::B, Tower::A),
            (Tower::B, Tower::C),
            (Tower::C, Tower::A),
            (Tower::C, Tower::B),
        ];

        for (src, dst) in test_cases {
            let moves = solve_hanoi(3, src, dst);
            assert_eq!(moves.len(), 7);
            assert_eq!(moves[0].from, src);
            assert_eq!(moves[6].to, dst);
        }
    }

    #[test]
    fn test_move_display() {
        // 测试Move的Display实现
        let m = Move {
            disk: 3,
            from: Tower::A,
            to: Tower::C,
        };
        assert_eq!(format!("{}", m), "3# A-->C");
    }

    #[test]
    fn test_tower_display() {
        // 测试Tower的Display实现
        assert_eq!(format!("{}", Tower::A), "A");
        assert_eq!(format!("{}", Tower::B), "B");
        assert_eq!(format!("{}", Tower::C), "C");
    }
}
