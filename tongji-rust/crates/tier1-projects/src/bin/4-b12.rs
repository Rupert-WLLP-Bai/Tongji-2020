// 4-b12: Check if a number is a power of a base using recursion
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用bool类型替代int返回值(0/1)，更符合语义
// 2. 递归函数使用尾递归优化，编译器可能优化为循环
// 3. 添加边界条件检查，避免无效输入(base < 2, num < 1)
// 4. 使用Result处理输入错误，而不是panic
// 5. 提取核心逻辑为纯函数，便于单元测试

use std::io::{self, Write};

/// 检查num是否是base的幂
///
/// # Arguments
/// * `num` - 要检查的数字
/// * `base` - 基数(必须 >= 2)
///
/// # Returns
/// * `true` - 如果num是base的幂
/// * `false` - 如果num不是base的幂
///
/// # Examples
/// ```
/// assert_eq!(is_power(8, 2), true);  // 8 = 2^3
/// assert_eq!(is_power(9, 3), true);  // 9 = 3^2
/// assert_eq!(is_power(10, 2), false);
/// ```
fn is_power(num: i32, base: i32) -> bool {
    // Rust改进: 添加边界检查，避免无效输入
    if base < 2 {
        return false;
    }
    if num < 1 {
        return false;
    }

    // Rust改进: 使用match表达式，更清晰的逻辑分支
    match num {
        1 => true, // 任何数的0次幂都是1
        n if n % base == 0 => is_power(n / base, base), // 尾递归
        _ => false,
    }
}

/// 读取并验证用户输入
///
/// # Returns
/// * `Ok((num, base))` - 成功读取的数字和基数
/// * `Err(String)` - 错误信息
fn read_input() -> Result<(i32, i32), String> {
    print!("请输入整数num及基数base(2以上的正整数)");
    io::stdout().flush().unwrap();
    println!();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("读取输入失败: {}", e))?;

    // Rust改进: 使用迭代器和collect解析多个数字
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.len() != 2 {
        return Err("请输入两个整数".to_string());
    }

    let (num, base) = (numbers[0], numbers[1]);

    // 验证输入范围
    if base < 2 {
        return Err("基数必须是2以上的正整数".to_string());
    }

    Ok((num, base))
}

fn main() {
    match read_input() {
        Ok((num, base)) => {
            // Rust改进: 使用if表达式直接构造输出字符串
            let result = if is_power(num, base) {
                format!("{}是{}的幂", num, base)
            } else {
                format!("{}不是{}的幂", num, base)
            };
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_of_two() {
        // 测试2的幂
        assert_eq!(is_power(1, 2), true); // 2^0 = 1
        assert_eq!(is_power(2, 2), true); // 2^1 = 2
        assert_eq!(is_power(4, 2), true); // 2^2 = 4
        assert_eq!(is_power(8, 2), true); // 2^3 = 8
        assert_eq!(is_power(16, 2), true); // 2^4 = 16
        assert_eq!(is_power(1024, 2), true); // 2^10 = 1024
    }

    #[test]
    fn test_not_power_of_two() {
        // 测试不是2的幂
        assert_eq!(is_power(3, 2), false);
        assert_eq!(is_power(5, 2), false);
        assert_eq!(is_power(6, 2), false);
        assert_eq!(is_power(7, 2), false);
        assert_eq!(is_power(10, 2), false);
    }

    #[test]
    fn test_power_of_three() {
        // 测试3的幂
        assert_eq!(is_power(1, 3), true); // 3^0 = 1
        assert_eq!(is_power(3, 3), true); // 3^1 = 3
        assert_eq!(is_power(9, 3), true); // 3^2 = 9
        assert_eq!(is_power(27, 3), true); // 3^3 = 27
        assert_eq!(is_power(81, 3), true); // 3^4 = 81
    }

    #[test]
    fn test_not_power_of_three() {
        // 测试不是3的幂
        assert_eq!(is_power(2, 3), false);
        assert_eq!(is_power(4, 3), false);
        assert_eq!(is_power(10, 3), false);
        assert_eq!(is_power(26, 3), false);
    }

    #[test]
    fn test_power_of_larger_bases() {
        // 测试更大的基数
        assert_eq!(is_power(25, 5), true); // 5^2 = 25
        assert_eq!(is_power(125, 5), true); // 5^3 = 125
        assert_eq!(is_power(49, 7), true); // 7^2 = 49
        assert_eq!(is_power(100, 10), true); // 10^2 = 100
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        assert_eq!(is_power(1, 2), true); // 1是任何数的0次幂
        assert_eq!(is_power(1, 10), true);
        assert_eq!(is_power(1, 100), true);
    }

    #[test]
    fn test_invalid_base() {
        // 测试无效基数
        assert_eq!(is_power(8, 1), false); // base < 2
        assert_eq!(is_power(8, 0), false);
        assert_eq!(is_power(8, -1), false);
    }

    #[test]
    fn test_negative_and_zero_num() {
        // 测试负数和零
        assert_eq!(is_power(0, 2), false);
        assert_eq!(is_power(-1, 2), false);
        assert_eq!(is_power(-8, 2), false);
    }

    #[test]
    fn test_large_numbers() {
        // 测试大数
        assert_eq!(is_power(1048576, 2), true); // 2^20
        assert_eq!(is_power(1048577, 2), false); // 2^20 + 1
        assert_eq!(is_power(59049, 3), true); // 3^10
    }
}
