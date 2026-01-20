// 9-b2: Find minimum of 2-4 positive integers
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用泛型和切片代替C++的默认参数，避免魔法值-1的sentinel模式
// 2. 使用Iterator::min()提供零成本抽象，编译器优化为最优代码
// 3. 使用Result<T, E>处理输入验证，类型安全且可组合
// 4. 提取验证逻辑为纯函数，消除重复代码（原C++有大量重复的输入验证）
// 5. 使用parse()的Result自动处理类型转换错误，无需手动清理输入流
// 6. 零unsafe代码，完全内存安全
// 7. 函数式编程风格：使用迭代器链式操作代替命令式循环
// 8. 添加comprehensive单元测试覆盖边界情况

use std::io::{self, Write};

/// 从切片中找到最小值
///
/// # Arguments
/// * `numbers` - 整数切片，至少包含一个元素
///
/// # Returns
/// * `Some(i32)` - 切片中的最小值
/// * `None` - 切片为空
///
/// # Examples
/// ```
/// assert_eq!(find_min(&[3, 1, 4]), Some(1));
/// assert_eq!(find_min(&[5, 2]), Some(2));
/// assert_eq!(find_min(&[]), None);
/// ```
///
/// # Rust改进
/// 使用Iterator::min()代替手动比较，编译器会优化为最优机器码
/// 原C++使用默认参数-1作为sentinel值，容易出错且不类型安全
fn find_min(numbers: &[i32]) -> Option<i32> {
    numbers.iter().copied().min()
}

/// 验证数字个数是否在有效范围内
///
/// # Arguments
/// * `count` - 要验证的数字个数
///
/// # Returns
/// * `Ok(usize)` - 有效的数字个数
/// * `Err(String)` - 错误信息
///
/// # Rust改进
/// 使用Result类型提供可组合的错误处理，避免C++的多层if嵌套
fn validate_count(count: i32) -> Result<usize, String> {
    if (2..=4).contains(&count) {
        Ok(count as usize)
    } else {
        Err(format!("个数必须在2-4之间，实际输入: {}", count))
    }
}

/// 验证所有数字都是正整数
///
/// # Arguments
/// * `numbers` - 要验证的数字切片
///
/// # Returns
/// * `Ok(())` - 所有数字都是正整数
/// * `Err(String)` - 包含非正整数的错误信息
///
/// # Rust改进
/// 使用Iterator::all()进行声明式验证，比C++的命令式检查更清晰
fn validate_positive(numbers: &[i32]) -> Result<(), String> {
    if numbers.iter().all(|&n| n > 0) {
        Ok(())
    } else {
        Err("所有数字必须是正整数".to_string())
    }
}

/// 读取一行输入并解析为整数
///
/// # Returns
/// * `Ok(i32)` - 成功解析的整数
/// * `Err(String)` - 解析失败的错误信息
///
/// # Rust改进
/// 使用parse()的Result自动处理类型转换，无需手动清理输入流
/// 原C++使用cin.fail()、cin.clear()、cin.ignore()的繁琐模式
fn read_integer() -> Result<i32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("读取输入失败: {}", e))?;

    input
        .trim()
        .parse::<i32>()
        .map_err(|e| format!("解析整数失败: {}", e))
}

/// 读取指定数量的正整数
///
/// # Arguments
/// * `count` - 要读取的整数个数
///
/// # Returns
/// * `Ok(Vec<i32>)` - 成功读取的整数向量
/// * `Err(String)` - 读取或验证失败的错误信息
///
/// # Rust改进
/// 使用Vec动态分配，避免C++的固定变量a,b,c,d
/// 使用函数式编程消除重复的输入验证代码
fn read_positive_integers(count: usize) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::with_capacity(count);

    for i in 0..count {
        match read_integer() {
            Ok(n) => numbers.push(n),
            Err(e) => return Err(format!("读取第{}个数字失败: {}", i + 1, e)),
        }
    }

    validate_positive(&numbers)?;
    Ok(numbers)
}

/// 主程序逻辑
///
/// # Returns
/// * `Ok(())` - 程序正常执行
/// * `Err(String)` - 执行过程中的错误
///
/// # Rust改进
/// 将main逻辑提取为可测试的函数，使用Result进行错误传播
fn run() -> Result<(), String> {
    println!("请输入个数num及num个正整数");

    // 读取并验证数字个数
    let count = loop {
        match read_integer() {
            Ok(n) => match validate_count(n) {
                Ok(count) => break count,
                Err(e) => {
                    println!("个数输入错误");
                    return Err(e);
                }
            },
            Err(_) => {
                // 输入错误，继续循环（与原C++行为一致）
                continue;
            }
        }
    };

    // 读取指定数量的正整数
    let numbers = loop {
        match read_positive_integers(count) {
            Ok(nums) => break nums,
            Err(_) => {
                // 输入错误，重新读取（与原C++行为一致）
                continue;
            }
        }
    };

    // 计算并输出最小值
    match find_min(&numbers) {
        Some(min_value) => {
            print!("min={}", min_value);
            io::stdout().flush().unwrap();
            Ok(())
        }
        None => Err("无法计算最小值：数组为空".to_string()),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_two_numbers() {
        // 测试两个数字的最小值
        assert_eq!(find_min(&[5, 3]), Some(3));
        assert_eq!(find_min(&[1, 10]), Some(1));
        assert_eq!(find_min(&[7, 7]), Some(7));
    }

    #[test]
    fn test_find_min_three_numbers() {
        // 测试三个数字的最小值
        assert_eq!(find_min(&[5, 3, 8]), Some(3));
        assert_eq!(find_min(&[10, 1, 5]), Some(1));
        assert_eq!(find_min(&[2, 2, 2]), Some(2));
    }

    #[test]
    fn test_find_min_four_numbers() {
        // 测试四个数字的最小值
        assert_eq!(find_min(&[5, 3, 8, 1]), Some(1));
        assert_eq!(find_min(&[10, 20, 5, 15]), Some(5));
        assert_eq!(find_min(&[100, 50, 75, 25]), Some(25));
    }

    #[test]
    fn test_find_min_empty_slice() {
        // 测试空切片
        assert_eq!(find_min(&[]), None);
    }

    #[test]
    fn test_find_min_single_element() {
        // 测试单个元素
        assert_eq!(find_min(&[42]), Some(42));
    }

    #[test]
    fn test_find_min_negative_numbers() {
        // 测试负数（虽然程序要求正整数，但函数本身应该能处理）
        assert_eq!(find_min(&[-5, -3, -8]), Some(-8));
        assert_eq!(find_min(&[-1, 0, 1]), Some(-1));
    }

    #[test]
    fn test_find_min_large_numbers() {
        // 测试大数字
        assert_eq!(find_min(&[i32::MAX, i32::MAX - 1]), Some(i32::MAX - 1));
        assert_eq!(find_min(&[1000000, 999999, 1000001]), Some(999999));
    }

    #[test]
    fn test_validate_count_valid() {
        // 测试有效的数字个数
        assert_eq!(validate_count(2), Ok(2));
        assert_eq!(validate_count(3), Ok(3));
        assert_eq!(validate_count(4), Ok(4));
    }

    #[test]
    fn test_validate_count_invalid() {
        // 测试无效的数字个数
        assert!(validate_count(1).is_err());
        assert!(validate_count(5).is_err());
        assert!(validate_count(0).is_err());
        assert!(validate_count(-1).is_err());
        assert!(validate_count(100).is_err());
    }

    #[test]
    fn test_validate_positive_all_positive() {
        // 测试所有正整数
        assert!(validate_positive(&[1, 2, 3]).is_ok());
        assert!(validate_positive(&[100, 200]).is_ok());
        assert!(validate_positive(&[1]).is_ok());
    }

    #[test]
    fn test_validate_positive_with_zero() {
        // 测试包含零的情况
        assert!(validate_positive(&[0, 1, 2]).is_err());
        assert!(validate_positive(&[1, 0]).is_err());
        assert!(validate_positive(&[0]).is_err());
    }

    #[test]
    fn test_validate_positive_with_negative() {
        // 测试包含负数的情况
        assert!(validate_positive(&[-1, 2, 3]).is_err());
        assert!(validate_positive(&[1, -5]).is_err());
        assert!(validate_positive(&[-10, -20]).is_err());
    }

    #[test]
    fn test_validate_positive_empty() {
        // 测试空切片（技术上所有元素都是正数）
        assert!(validate_positive(&[]).is_ok());
    }

    #[test]
    fn test_find_min_matches_cpp_behavior() {
        // 测试与原C++行为一致的场景
        // 原C++: min(a, b) where a=5, b=3
        assert_eq!(find_min(&[5, 3]), Some(3));

        // 原C++: min(a, b, c) where a=10, b=5, c=8
        assert_eq!(find_min(&[10, 5, 8]), Some(5));

        // 原C++: min(a, b, c, d) where a=7, b=3, c=9, d=2
        assert_eq!(find_min(&[7, 3, 9, 2]), Some(2));
    }

    #[test]
    fn test_find_min_boundary_values() {
        // 测试边界值
        assert_eq!(find_min(&[i32::MIN, i32::MAX]), Some(i32::MIN));
        assert_eq!(find_min(&[i32::MAX, i32::MIN]), Some(i32::MIN));
        assert_eq!(find_min(&[0, 1, 2, 3]), Some(0));
    }

    #[test]
    fn test_find_min_all_same() {
        // 测试所有数字相同
        assert_eq!(find_min(&[5, 5]), Some(5));
        assert_eq!(find_min(&[10, 10, 10]), Some(10));
        assert_eq!(find_min(&[1, 1, 1, 1]), Some(1));
    }

    #[test]
    fn test_find_min_descending_order() {
        // 测试降序排列
        assert_eq!(find_min(&[10, 5]), Some(5));
        assert_eq!(find_min(&[100, 50, 25]), Some(25));
        assert_eq!(find_min(&[1000, 500, 250, 125]), Some(125));
    }

    #[test]
    fn test_find_min_ascending_order() {
        // 测试升序排列
        assert_eq!(find_min(&[1, 5]), Some(1));
        assert_eq!(find_min(&[10, 20, 30]), Some(10));
        assert_eq!(find_min(&[5, 10, 15, 20]), Some(5));
    }

    #[test]
    fn test_validate_count_boundary() {
        // 测试边界值
        assert!(validate_count(1).is_err()); // 小于最小值
        assert_eq!(validate_count(2), Ok(2)); // 最小有效值
        assert_eq!(validate_count(4), Ok(4)); // 最大有效值
        assert!(validate_count(5).is_err()); // 大于最大值
    }
}
