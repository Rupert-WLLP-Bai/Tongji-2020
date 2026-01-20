// 4-b8: Tower of Hanoi solver using recursion
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用递归解决汉诺塔问题,无循环实现
// 2. 使用enum表示柱子,类型安全且语义清晰
// 3. 使用Result类型处理输入验证,符合Rust错误处理习惯
// 4. 提取输入验证为独立函数,便于测试
// 5. 使用Vec收集移动步骤,便于测试和验证
// 6. 避免使用static变量,通过参数传递状态

use std::io;

/// 汉诺塔的柱子
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tower {
    A,
    B,
    C,
}

impl Tower {
    /// 从字符创建Tower
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

    /// 计算第三个柱子
    /// 给定两个柱子,返回剩余的第三个柱子
    fn get_third(a: Tower, b: Tower) -> Tower {
        // Rust改进: 使用match穷举所有情况,编译器保证完整性
        match (a, b) {
            (Tower::A, Tower::B) | (Tower::B, Tower::A) => Tower::C,
            (Tower::A, Tower::C) | (Tower::C, Tower::A) => Tower::B,
            (Tower::B, Tower::C) | (Tower::C, Tower::B) => Tower::A,
            _ => panic!("起始柱和目标柱不能相同"),
        }
    }
}

/// 单步移动记录
#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    step: usize,
    disk: i32,
    from: Tower,
    to: Tower,
}

impl Move {
    fn format(&self) -> String {
        format!(
            "{:5}:{:3}# {}-->{}",
            self.step,
            self.disk,
            self.from.to_char(),
            self.to.to_char()
        )
    }
}

/// 汉诺塔求解器(递归实现)
///
/// # Arguments
/// * `n` - 盘子数量
/// * `src` - 起始柱
/// * `tmp` - 中间柱
/// * `dst` - 目标柱
/// * `step` - 当前步数(用于编号)
/// * `moves` - 累积移动步骤的可变引用
///
/// # Returns
/// * `usize` - 更新后的步数
///
/// # Rust改进说明
/// - 不使用static变量,通过参数传递状态
/// - 返回步数而非修改全局状态,更函数式
/// - 使用Vec收集结果,便于测试
fn hanoi_impl(
    n: i32,
    src: Tower,
    tmp: Tower,
    dst: Tower,
    step: usize,
    moves: &mut Vec<Move>,
) -> usize {
    if n == 1 {
        // 基础情况: 只有一个盘子,直接移动
        moves.push(Move {
            step: step + 1,
            disk: n,
            from: src,
            to: dst,
        });
        step + 1
    } else {
        // 递归情况:
        // 1. 将n-1个盘子从src移到tmp(使用dst作为中间柱)
        let step = hanoi_impl(n - 1, src, dst, tmp, step, moves);

        // 2. 将第n个盘子从src移到dst
        moves.push(Move {
            step: step + 1,
            disk: n,
            from: src,
            to: dst,
        });
        let step = step + 1;

        // 3. 将n-1个盘子从tmp移到dst(使用src作为中间柱)
        hanoi_impl(n - 1, tmp, src, dst, step, moves)
    }
}

/// 求解汉诺塔问题
///
/// # Arguments
/// * `n` - 盘子数量(1-16)
/// * `src` - 起始柱
/// * `dst` - 目标柱
///
/// # Returns
/// * `Vec<Move>` - 所有移动步骤
pub fn hanoi(n: i32, src: Tower, dst: Tower) -> Vec<Move> {
    let mut moves = Vec::new();
    let tmp = Tower::get_third(src, dst);
    hanoi_impl(n, src, tmp, dst, 0, &mut moves);
    moves
}

/// 验证层数输入(1-16)
fn validate_level(input: &str) -> Option<i32> {
    input
        .trim()
        .parse::<i32>()
        .ok()
        .filter(|&n| (1..=16).contains(&n))
}

/// 验证柱子输入(A/B/C,不区分大小写)
fn validate_tower(input: &str) -> Option<Tower> {
    let trimmed = input.trim();
    if trimmed.len() == 1 {
        Tower::from_char(trimmed.chars().next().unwrap())
    } else {
        None
    }
}

/// 读取层数
fn read_level() -> i32 {
    loop {
        println!("请输入汉诺塔的层数(1-16)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(level) = validate_level(&input) {
            return level;
        }
    }
}

/// 读取柱子
fn read_tower(prompt: &str, exclude: Option<Tower>) -> Tower {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(tower) = validate_tower(&input) {
            if let Some(excluded) = exclude {
                if tower == excluded {
                    println!("起始柱不能与目标柱相同");
                    continue;
                }
            }
            return tower;
        }
    }
}

fn main() {
    // 读取输入
    let level = read_level();
    let start = read_tower("请输入起始柱(A-C)", None);
    let end = read_tower("请输入目标柱(A-C)", Some(start));

    // 求解并打印
    let moves = hanoi(level, start, end);
    for m in moves {
        println!("{}", m.format());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tower_from_char() {
        // 测试大小写转换
        assert_eq!(Tower::from_char('A'), Some(Tower::A));
        assert_eq!(Tower::from_char('a'), Some(Tower::A));
        assert_eq!(Tower::from_char('B'), Some(Tower::B));
        assert_eq!(Tower::from_char('b'), Some(Tower::B));
        assert_eq!(Tower::from_char('C'), Some(Tower::C));
        assert_eq!(Tower::from_char('c'), Some(Tower::C));
        assert_eq!(Tower::from_char('D'), None);
    }

    #[test]
    fn test_tower_get_third() {
        // 测试计算第三个柱子
        assert_eq!(Tower::get_third(Tower::A, Tower::B), Tower::C);
        assert_eq!(Tower::get_third(Tower::B, Tower::A), Tower::C);
        assert_eq!(Tower::get_third(Tower::A, Tower::C), Tower::B);
        assert_eq!(Tower::get_third(Tower::C, Tower::A), Tower::B);
        assert_eq!(Tower::get_third(Tower::B, Tower::C), Tower::A);
        assert_eq!(Tower::get_third(Tower::C, Tower::B), Tower::A);
    }

    #[test]
    #[should_panic(expected = "起始柱和目标柱不能相同")]
    fn test_tower_get_third_same() {
        // 测试相同柱子会panic
        Tower::get_third(Tower::A, Tower::A);
    }

    #[test]
    fn test_validate_level() {
        // 测试有效层数
        assert_eq!(validate_level("1"), Some(1));
        assert_eq!(validate_level("8"), Some(8));
        assert_eq!(validate_level("16"), Some(16));

        // 测试无效层数
        assert_eq!(validate_level("0"), None);
        assert_eq!(validate_level("17"), None);
        assert_eq!(validate_level("-1"), None);
        assert_eq!(validate_level("abc"), None);
    }

    #[test]
    fn test_validate_tower() {
        // 测试有效柱子
        assert_eq!(validate_tower("A"), Some(Tower::A));
        assert_eq!(validate_tower("a"), Some(Tower::A));
        assert_eq!(validate_tower("B"), Some(Tower::B));
        assert_eq!(validate_tower("C"), Some(Tower::C));

        // 测试无效柱子
        assert_eq!(validate_tower("D"), None);
        assert_eq!(validate_tower("AB"), None);
        assert_eq!(validate_tower(""), None);
    }

    #[test]
    fn test_hanoi_one_disk() {
        // 测试1个盘子
        let moves = hanoi(1, Tower::A, Tower::C);
        assert_eq!(moves.len(), 1);
        assert_eq!(
            moves[0],
            Move {
                step: 1,
                disk: 1,
                from: Tower::A,
                to: Tower::C,
            }
        );
    }

    #[test]
    fn test_hanoi_two_disks() {
        // 测试2个盘子
        let moves = hanoi(2, Tower::A, Tower::C);
        assert_eq!(moves.len(), 3);

        // 验证移动顺序
        assert_eq!(moves[0].disk, 1);
        assert_eq!(moves[0].from, Tower::A);
        assert_eq!(moves[0].to, Tower::B);

        assert_eq!(moves[1].disk, 2);
        assert_eq!(moves[1].from, Tower::A);
        assert_eq!(moves[1].to, Tower::C);

        assert_eq!(moves[2].disk, 1);
        assert_eq!(moves[2].from, Tower::B);
        assert_eq!(moves[2].to, Tower::C);
    }

    #[test]
    fn test_hanoi_three_disks() {
        // 测试3个盘子
        let moves = hanoi(3, Tower::A, Tower::C);
        assert_eq!(moves.len(), 7); // 2^3 - 1 = 7

        // 验证步数连续
        for (i, m) in moves.iter().enumerate() {
            assert_eq!(m.step, i + 1);
        }
    }

    #[test]
    fn test_hanoi_move_count() {
        // 测试移动次数公式: 2^n - 1
        for n in 1..=10 {
            let moves = hanoi(n, Tower::A, Tower::C);
            let expected = (1 << n) - 1; // 2^n - 1
            assert_eq!(moves.len(), expected);
        }
    }

    #[test]
    fn test_hanoi_different_towers() {
        // 测试不同的起始和目标柱组合
        let moves_ac = hanoi(2, Tower::A, Tower::C);
        let moves_ab = hanoi(2, Tower::A, Tower::B);
        let moves_bc = hanoi(2, Tower::B, Tower::C);

        // 移动次数应该相同
        assert_eq!(moves_ac.len(), 3);
        assert_eq!(moves_ab.len(), 3);
        assert_eq!(moves_bc.len(), 3);
    }

    #[test]
    fn test_move_format() {
        // 测试移动格式化
        let m = Move {
            step: 1,
            disk: 3,
            from: Tower::A,
            to: Tower::C,
        };
        assert_eq!(m.format(), "    1:  3# A-->C");
    }

    #[test]
    fn test_hanoi_validity() {
        // 测试汉诺塔解的有效性
        // 规则: 大盘子不能放在小盘子上面
        let moves = hanoi(4, Tower::A, Tower::C);

        // 模拟三个柱子的状态
        let mut towers: [Vec<i32>; 3] = [vec![4, 3, 2, 1], vec![], vec![]];

        for m in moves {
            let from_idx = m.from as usize;
            let to_idx = m.to as usize;

            // 从源柱取出盘子
            let disk = towers[from_idx].pop().expect("源柱应该有盘子");
            assert_eq!(disk, m.disk, "移动的盘子应该匹配");

            // 检查目标柱顶部
            if let Some(&top) = towers[to_idx].last() {
                assert!(disk < top, "不能将大盘子放在小盘子上");
            }

            // 放到目标柱
            towers[to_idx].push(disk);
        }

        // 验证最终状态: 所有盘子都在目标柱C上
        assert!(towers[0].is_empty());
        assert!(towers[1].is_empty());
        assert_eq!(towers[2], vec![4, 3, 2, 1]);
    }
}
