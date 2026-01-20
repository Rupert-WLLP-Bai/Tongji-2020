// 4-b13: Recursive triangle tower printer
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用enum表示正/倒三角形，类型安全替代magic number
// 2. 递归函数使用不可变引用，避免不必要的复制
// 3. 使用String构建输出，避免多次IO调用，提升性能
// 4. 提取字符序列生成为独立函数，便于测试和复用
// 5. 使用迭代器和collect()，更符合Rust函数式风格
// 6. 输入验证逻辑提取为独立函数，增强可测试性

use std::io::{self, Write};

/// 三角形塔的方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TowerOrder {
    /// 正三角形塔(中间为A)
    Normal,
    /// 倒三角形塔(两边为A)
    Inverted,
}

/// 生成从start到end的字符序列(正序)
///
/// # Arguments
/// * `start` - 起始字符
/// * `end` - 结束字符(包含)
///
/// # Returns
/// * 字符序列字符串
fn print_sequential(start: char, end: char) -> String {
    // Rust改进: 使用迭代器和collect()，比递归更高效且更符合Rust习惯
    if end < start {
        String::new()
    } else {
        (start..=end).collect()
    }
}

/// 生成从start到end的字符序列(倒序)
///
/// # Arguments
/// * `start` - 起始字符
/// * `end` - 结束字符(包含)
///
/// # Returns
/// * 字符序列字符串(倒序)
fn print_reverse(start: char, end: char) -> String {
    // Rust改进: 使用rev()反转迭代器，零成本抽象
    if end < start {
        String::new()
    } else {
        (start..=end).rev().collect()
    }
}

/// 递归打印三角形塔(内部实现)
///
/// # Arguments
/// * `start_ch` - 当前行起始字符
/// * `end_ch` - 结束字符
/// * `order` - 三角形方向
/// * `initial_end` - 初始的结束字符(用于计算缩进)
/// * `output` - 输出缓冲区
fn print_tower_recursive(
    start_ch: char,
    end_ch: char,
    order: TowerOrder,
    initial_end: char,
    output: &mut String,
) {
    if start_ch > end_ch {
        return;
    }

    match order {
        TowerOrder::Normal => {
            // 正三角形: 先递归打印上面的行，再打印当前行
            print_tower_recursive(start_ch, (end_ch as u8 - 1) as char, order, initial_end, output);

            // 计算缩进: 从顶部到当前的距离 = initial_end - end_ch
            // 例如: A到D时，A行缩进3，B行缩进2，C行缩进1，D行缩进0
            let indent = (initial_end as u8 - end_ch as u8) as usize;
            output.push_str(&" ".repeat(indent));

            // 打印当前行: 从end_ch倒序到start_ch+1，再从start_ch正序到end_ch
            // 例如: 'A'到'C' => "CB" + "ABC" = "CBABC"
            output.push_str(&print_reverse((start_ch as u8 + 1) as char, end_ch));
            output.push_str(&print_sequential(start_ch, end_ch));
            output.push('\n');
        }
        TowerOrder::Inverted => {
            // 倒三角形: 先打印当前行，再递归打印下面的行
            // 缩进: 当前深度 = start_ch - 'A'
            let indent = (start_ch as u8 - b'A') as usize;
            output.push_str(&" ".repeat(indent));

            // 打印当前行: 从start_ch正序到end_ch-1，再从end_ch倒序到start_ch
            // 例如: 'A'到'C' => "AB" + "CBA" = "ABCBA"
            output.push_str(&print_sequential(start_ch, (end_ch as u8 - 1) as char));
            output.push_str(&print_reverse(start_ch, end_ch));
            output.push('\n');

            print_tower_recursive((start_ch as u8 + 1) as char, end_ch, order, initial_end, output);
        }
    }
}

/// 打印三角形塔(公共接口)
///
/// # Arguments
/// * `start_ch` - 起始字符
/// * `end_ch` - 结束字符
/// * `order` - 三角形方向
///
/// # Returns
/// * 三角形塔的字符串表示
pub fn print_tower(start_ch: char, end_ch: char, order: TowerOrder) -> String {
    // Rust改进: 使用String作为缓冲区，一次性输出，避免多次IO调用
    let mut output = String::new();
    print_tower_recursive(start_ch, end_ch, order, end_ch, &mut output);
    output
}

/// 验证输入字符是否为A-Z
///
/// # Arguments
/// * `input` - 输入字符串
///
/// # Returns
/// * `Some(char)` - 如果输入有效
/// * `None` - 如果输入无效
fn validate_input(input: &str) -> Option<char> {
    // Rust改进: 使用chars().next()获取第一个字符，filter验证范围
    input
        .trim()
        .chars()
        .next()
        .filter(|&ch| ch >= 'A' && ch <= 'Z')
}

fn main() {
    // 读取结束字符
    let end_ch = loop {
        print!("请输入结束字符(A~Z)\n");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(ch) = validate_input(&input) {
            break ch;
        }
    };

    let width = ((end_ch as u8 - b'A') as usize) * 2 + 1;

    // 正三角字母塔
    println!("{}", "=".repeat(width));
    println!("正三角字母塔");
    println!("{}", "=".repeat(width));
    print!("{}", print_tower('A', end_ch, TowerOrder::Normal));
    println!();

    // 倒三角字母塔
    println!("{}", "=".repeat(width));
    println!("倒三角字母塔");
    println!("{}", "=".repeat(width));
    print!("{}", print_tower('A', end_ch, TowerOrder::Inverted));
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_sequential() {
        assert_eq!(print_sequential('A', 'C'), "ABC");
        assert_eq!(print_sequential('A', 'A'), "A");
        assert_eq!(print_sequential('B', 'A'), ""); // end < start
        assert_eq!(print_sequential('X', 'Z'), "XYZ");
    }

    #[test]
    fn test_print_reverse() {
        assert_eq!(print_reverse('A', 'C'), "CBA");
        assert_eq!(print_reverse('A', 'A'), "A");
        assert_eq!(print_reverse('B', 'A'), ""); // end < start
        assert_eq!(print_reverse('X', 'Z'), "ZYX");
    }

    #[test]
    fn test_validate_input() {
        assert_eq!(validate_input("A"), Some('A'));
        assert_eq!(validate_input("Z"), Some('Z'));
        assert_eq!(validate_input("M"), Some('M'));
        assert_eq!(validate_input("  C  \n"), Some('C'));

        // 无效输入
        assert_eq!(validate_input("a"), None); // 小写
        assert_eq!(validate_input("1"), None); // 数字
        assert_eq!(validate_input(""), None); // 空
        assert_eq!(validate_input("AB"), Some('A')); // 多字符，取第一个
    }

    #[test]
    fn test_print_tower_normal_single() {
        // 单字符正三角形
        let result = print_tower('A', 'A', TowerOrder::Normal);
        assert_eq!(result, "A\n");
    }

    #[test]
    fn test_print_tower_normal_small() {
        // 小型正三角形 (A-C)
        // 输出格式: 顶部缩进最多，底部无缩进
        let result = print_tower('A', 'C', TowerOrder::Normal);
        let expected = "  A\n BAB\nCBABC\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_print_tower_inverted_single() {
        // 单字符倒三角形
        let result = print_tower('A', 'A', TowerOrder::Inverted);
        assert_eq!(result, "A\n");
    }

    #[test]
    fn test_print_tower_inverted_small() {
        // 小型倒三角形 (A-C)
        // 输出格式: 顶部无缩进，底部缩进最多
        let result = print_tower('A', 'C', TowerOrder::Inverted);
        let expected = "ABCBA\n BCB\n  C\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_print_tower_normal_medium() {
        // 中型正三角形 (A-D)
        let result = print_tower('A', 'D', TowerOrder::Normal);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "   A");
        assert_eq!(lines[1], "  BAB");
        assert_eq!(lines[2], " CBABC");
        assert_eq!(lines[3], "DCBABCD");
    }

    #[test]
    fn test_print_tower_inverted_medium() {
        // 中型倒三角形 (A-D)
        let result = print_tower('A', 'D', TowerOrder::Inverted);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "ABCDCBA");
        assert_eq!(lines[1], " BCDCB");
        assert_eq!(lines[2], "  CDC");
        assert_eq!(lines[3], "   D");
    }

    #[test]
    fn test_tower_order_enum() {
        // 测试枚举类型
        assert_eq!(TowerOrder::Normal, TowerOrder::Normal);
        assert_ne!(TowerOrder::Normal, TowerOrder::Inverted);
    }

    #[test]
    fn test_print_tower_empty() {
        // 测试start > end的情况
        let result = print_tower('B', 'A', TowerOrder::Normal);
        assert_eq!(result, "");

        let result = print_tower('B', 'A', TowerOrder::Inverted);
        assert_eq!(result, "");
    }
}
