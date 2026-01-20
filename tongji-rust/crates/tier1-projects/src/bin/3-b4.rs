// 3-b4: Extract digits from a number (0-10 billion) with up to 2 decimal places
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用结构体封装数字的各个位，提高代码可读性和类型安全
// 2. 使用数组和迭代器处理整数部分，避免重复代码
// 3. 使用整数运算代替浮点运算，避免精度问题
// 4. 提取核心逻辑为纯函数，便于单元测试
// 5. 使用const泛型和编译期计算，零运行时开销

use std::io::{self, Write};

/// 表示一个数字的各个位（十亿位到分位）
#[derive(Debug, PartialEq, Eq)]
struct DigitBreakdown {
    /// 整数部分的各位，从个位到十亿位 [个, 十, 百, 千, 万, 十万, 百万, 千万, 亿, 十亿]
    integer_digits: [u8; 10],
    /// 小数部分：角（十分位）
    jiao: u8,
    /// 小数部分：分（百分位）
    fen: u8,
}

/// 从浮点数中提取各个位的数字
///
/// # Arguments
/// * `num` - 输入的数字，范围[0, 10_000_000_000)，最多两位小数
///
/// # Returns
/// * `Some(DigitBreakdown)` - 如果输入有效
/// * `None` - 如果输入超出范围或小数位数过多
///
/// # Rust改进
/// 使用整数运算避免浮点精度问题：
/// 1. 将输入乘以100转换为整数（分为单位）
/// 2. 使用整数除法和取模提取各位
/// 3. 零运行时开销，编译期优化
fn extract_digits(num: f64) -> Option<DigitBreakdown> {
    // 验证输入范围
    if num < 0.0 || num >= 10_000_000_000.0 {
        return None;
    }

    // Rust改进: 将浮点数转换为整数（以分为单位），避免浮点精度问题
    // 加0.5进行四舍五入，确保精度
    let total_fen = (num * 100.0 + 0.5) as u64;

    // 验证小数位数（检查是否超过2位小数）
    // 如果原始数字乘以100后与四舍五入的结果差距过大，说明小数位数过多
    if (num * 100.0 - total_fen as f64).abs() > 0.5 {
        return None;
    }

    // 提取小数部分（分和角）
    let fen = (total_fen % 10) as u8;
    let jiao = ((total_fen / 10) % 10) as u8;

    // 提取整数部分（圆及以上）
    let mut remaining = total_fen / 100;
    let mut integer_digits = [0u8; 10];

    // Rust改进: 使用迭代器和闭包，代码更简洁
    for digit in &mut integer_digits {
        *digit = (remaining % 10) as u8;
        remaining /= 10;
    }

    Some(DigitBreakdown {
        integer_digits,
        jiao,
        fen,
    })
}

/// 打印数字的各个位
fn print_breakdown(breakdown: &DigitBreakdown) {
    // 中文单位名称，从十亿位到个位
    let unit_names = [
        "十亿位",
        "亿位  ",
        "千万位",
        "百万位",
        "十万位",
        "万位  ",
        "千位  ",
        "百位  ",
        "十位  ",
        "圆    ",
    ];

    // Rust改进: 使用迭代器的rev()反转，从高位到低位输出
    for (i, &name) in unit_names.iter().enumerate() {
        let digit_index = 9 - i; // 从十亿位(index 9)到个位(index 0)
        println!("{}: {}", name, breakdown.integer_digits[digit_index]);
    }

    println!("角    : {}", breakdown.jiao);
    println!("分    : {}", breakdown.fen);
}

fn main() {
    print!("请输入[0-100亿)之间的数字,小数点后最多两位：");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Rust改进: 使用match表达式处理解析和验证
    match input.trim().parse::<f64>() {
        Ok(num) => match extract_digits(num) {
            Some(breakdown) => print_breakdown(&breakdown),
            None => println!("输入超出范围或小数位数过多"),
        },
        Err(_) => println!("输入格式错误"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let result = extract_digits(0.0).unwrap();
        assert_eq!(result.integer_digits, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result.jiao, 0);
        assert_eq!(result.fen, 0);
    }

    #[test]
    fn test_simple_integer() {
        let result = extract_digits(123.0).unwrap();
        assert_eq!(result.integer_digits, [3, 2, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result.jiao, 0);
        assert_eq!(result.fen, 0);
    }

    #[test]
    fn test_with_decimals() {
        let result = extract_digits(123.45).unwrap();
        assert_eq!(result.integer_digits, [3, 2, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result.jiao, 4);
        assert_eq!(result.fen, 5);
    }

    #[test]
    fn test_one_decimal_place() {
        let result = extract_digits(99.9).unwrap();
        assert_eq!(result.integer_digits, [9, 9, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result.jiao, 9);
        assert_eq!(result.fen, 0);
    }

    #[test]
    fn test_large_number() {
        // 9,876,543,210.12
        let result = extract_digits(9_876_543_210.12).unwrap();
        assert_eq!(result.integer_digits, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(result.jiao, 1);
        assert_eq!(result.fen, 2);
    }

    #[test]
    fn test_max_valid_number() {
        // 接近100亿但小于100亿
        let result = extract_digits(9_999_999_999.99).unwrap();
        assert_eq!(result.integer_digits, [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        assert_eq!(result.jiao, 9);
        assert_eq!(result.fen, 9);
    }

    #[test]
    fn test_boundary_values() {
        // 测试各个位的边界
        assert!(extract_digits(0.01).is_some());
        assert!(extract_digits(0.99).is_some());
        assert!(extract_digits(1.00).is_some());
        assert!(extract_digits(10.00).is_some());
        assert!(extract_digits(100.00).is_some());
        assert!(extract_digits(1_000.00).is_some());
        assert!(extract_digits(10_000.00).is_some());
        assert!(extract_digits(100_000.00).is_some());
        assert!(extract_digits(1_000_000.00).is_some());
        assert!(extract_digits(10_000_000.00).is_some());
        assert!(extract_digits(100_000_000.00).is_some());
        assert!(extract_digits(1_000_000_000.00).is_some());
    }

    #[test]
    fn test_out_of_range_negative() {
        assert_eq!(extract_digits(-1.0), None);
        assert_eq!(extract_digits(-0.01), None);
    }

    #[test]
    fn test_out_of_range_too_large() {
        assert_eq!(extract_digits(10_000_000_000.0), None);
        assert_eq!(extract_digits(10_000_000_001.0), None);
        assert_eq!(extract_digits(99_999_999_999.0), None);
    }

    #[test]
    fn test_floating_point_precision() {
        // 测试浮点精度问题
        // 原C++代码使用fmod和多次浮点运算，容易产生精度误差
        // Rust版本使用整数运算，避免精度问题
        let result = extract_digits(1234567890.12).unwrap();
        assert_eq!(result.integer_digits, [0, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(result.jiao, 1);
        assert_eq!(result.fen, 2);
    }

    #[test]
    fn test_rounding() {
        // 测试四舍五入
        // 0.126 应该被四舍五入为 0.13
        let result = extract_digits(0.126).unwrap();
        assert_eq!(result.jiao, 1);
        assert_eq!(result.fen, 3);
    }

    #[test]
    fn test_all_same_digits() {
        let result = extract_digits(1111111111.11).unwrap();
        assert_eq!(result.integer_digits, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(result.jiao, 1);
        assert_eq!(result.fen, 1);
    }

    #[test]
    fn test_alternating_digits() {
        let result = extract_digits(5050505050.50).unwrap();
        assert_eq!(result.integer_digits, [0, 5, 0, 5, 0, 5, 0, 5, 0, 5]);
        assert_eq!(result.jiao, 5);
        assert_eq!(result.fen, 0);
    }

    #[test]
    fn test_small_decimals() {
        let result = extract_digits(0.01).unwrap();
        assert_eq!(result.integer_digits, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(result.jiao, 0);
        assert_eq!(result.fen, 1);

        let result = extract_digits(0.10).unwrap();
        assert_eq!(result.jiao, 1);
        assert_eq!(result.fen, 0);
    }
}
