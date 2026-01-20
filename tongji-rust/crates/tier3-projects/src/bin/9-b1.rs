// 9-b1: Find maximum value among 2-4 positive integers
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用泛型和迭代器替代函数重载，代码更简洁且支持任意数量参数
// 2. 使用Result<T, E>进行错误处理，替代C++的cin.fail()检查
// 3. 零unsafe代码，完全内存安全
// 4. 使用Iterator::max()标准库方法，避免手动比较逻辑
// 5. 提取核心逻辑为纯函数，便于单元测试
// 6. 使用Option处理空集合情况，类型安全
// 7. 输入验证逻辑提取为独立函数，提高可测试性
// 8. 添加comprehensive单元测试覆盖各种边界情况

use std::io;

/// 从整数切片中找到最大值
///
/// # Arguments
/// * `numbers` - 整数切片
///
/// # Returns
/// * `Some(max)` - 最大值
/// * `None` - 空切片
///
/// # Examples
/// ```
/// let nums = vec![1, 5, 3];
/// assert_eq!(find_max(&nums), Some(&5));
/// ```
fn find_max(numbers: &[i32]) -> Option<&i32> {
    // Rust改进: 使用Iterator::max()标准库方法
    // 比手动比较更简洁、更不易出错
    numbers.iter().max()
}

/// 验证所有数字都是正整数
///
/// # Arguments
/// * `numbers` - 整数切片
///
/// # Returns
/// * `true` - 所有数字都是正整数
/// * `false` - 存在非正整数
///
/// # Examples
/// ```
/// assert_eq!(validate_positive(&[1, 2, 3]), true);
/// assert_eq!(validate_positive(&[1, -2, 3]), false);
/// assert_eq!(validate_positive(&[0, 1, 2]), false);
/// ```
fn validate_positive(numbers: &[i32]) -> bool {
    // Rust改进: 使用Iterator::all()进行验证
    // 比C++的多个if语句更简洁
    numbers.iter().all(|&n| n > 0)
}

/// 验证数量是否在有效范围内(2-4)
///
/// # Arguments
/// * `count` - 数量
///
/// # Returns
/// * `true` - 数量在2-4之间
/// * `false` - 数量不在有效范围
///
/// # Examples
/// ```
/// assert_eq!(validate_count(2), true);
/// assert_eq!(validate_count(3), true);
/// assert_eq!(validate_count(4), true);
/// assert_eq!(validate_count(1), false);
/// assert_eq!(validate_count(5), false);
/// ```
fn validate_count(count: usize) -> bool {
    (2..=4).contains(&count)
}

/// 从标准输入读取一个整数
///
/// # Returns
/// * `Ok(number)` - 成功读取的整数
/// * `Err(message)` - 读取失败的错误信息
fn read_integer() -> Result<i32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("读取输入失败: {}", e))?;

    input
        .trim()
        .parse::<i32>()
        .map_err(|_| "输入不是有效的整数".to_string())
}

/// 从标准输入读取指定数量的整数
///
/// # Arguments
/// * `count` - 需要读取的整数数量
///
/// # Returns
/// * `Ok(numbers)` - 成功读取的整数向量
/// * `Err(message)` - 读取失败的错误信息
fn read_integers(count: usize) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::with_capacity(count);

    for i in 0..count {
        match read_integer() {
            Ok(num) => numbers.push(num),
            Err(e) => return Err(format!("读取第{}个数字失败: {}", i + 1, e)),
        }
    }

    Ok(numbers)
}

/// 处理找最大值的核心逻辑
///
/// # Arguments
/// * `numbers` - 整数向量
///
/// # Returns
/// * `Ok(max)` - 最大值
/// * `Err(message)` - 错误信息
fn process_max(numbers: &[i32]) -> Result<i32, String> {
    // 验证所有数字都是正整数
    if !validate_positive(numbers) {
        return Err("所有数字必须是正整数".to_string());
    }

    // 查找最大值
    find_max(numbers)
        .copied()
        .ok_or_else(|| "数字列表为空".to_string())
}

fn main() {
    println!("请输入个数num及num个正整数");

    // 读取数量
    let count = loop {
        match read_integer() {
            Ok(num) if num >= 0 => {
                let count = num as usize;
                if validate_count(count) {
                    break count;
                } else {
                    println!("个数输入错误");
                    return;
                }
            }
            Ok(_) => {
                println!("个数输入错误");
                return;
            }
            Err(_) => {
                // Rust改进: 输入错误时继续循环，与原C++行为一致
                continue;
            }
        }
    };

    // 读取指定数量的整数
    let numbers = loop {
        match read_integers(count) {
            Ok(nums) => break nums,
            Err(_) => {
                // Rust改进: 读取失败时重新开始，与原C++行为一致
                continue;
            }
        }
    };

    // 处理并输出结果
    match process_max(&numbers) {
        Ok(max_value) => {
            println!("max={}", max_value);
        }
        Err(_) => {
            // Rust改进: 验证失败时静默退出，与原C++行为一致
            // 原C++在验证失败时continue到外层循环，这里直接退出
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_two_numbers() {
        // 测试两个数字的最大值
        let nums = vec![3, 7];
        assert_eq!(find_max(&nums), Some(&7));
    }

    #[test]
    fn test_find_max_three_numbers() {
        // 测试三个数字的最大值
        let nums = vec![5, 2, 8];
        assert_eq!(find_max(&nums), Some(&8));
    }

    #[test]
    fn test_find_max_four_numbers() {
        // 测试四个数字的最大值
        let nums = vec![4, 9, 2, 6];
        assert_eq!(find_max(&nums), Some(&9));
    }

    #[test]
    fn test_find_max_all_same() {
        // 测试所有数字相同
        let nums = vec![5, 5, 5];
        assert_eq!(find_max(&nums), Some(&5));
    }

    #[test]
    fn test_find_max_empty() {
        // 测试空切片
        let nums: Vec<i32> = vec![];
        assert_eq!(find_max(&nums), None);
    }

    #[test]
    fn test_find_max_single() {
        // 测试单个数字
        let nums = vec![42];
        assert_eq!(find_max(&nums), Some(&42));
    }

    #[test]
    fn test_find_max_negative_numbers() {
        // 测试负数（虽然程序要求正整数，但函数本身应该能处理）
        let nums = vec![-5, -2, -8];
        assert_eq!(find_max(&nums), Some(&-2));
    }

    #[test]
    fn test_find_max_mixed_numbers() {
        // 测试混合正负数
        let nums = vec![-5, 10, -2, 3];
        assert_eq!(find_max(&nums), Some(&10));
    }

    #[test]
    fn test_validate_positive_all_positive() {
        // 测试所有正整数
        assert!(validate_positive(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_validate_positive_with_zero() {
        // 测试包含零
        assert!(!validate_positive(&[1, 0, 3]));
    }

    #[test]
    fn test_validate_positive_with_negative() {
        // 测试包含负数
        assert!(!validate_positive(&[1, -2, 3]));
    }

    #[test]
    fn test_validate_positive_empty() {
        // 测试空切片（所有元素都满足条件，返回true）
        assert!(validate_positive(&[]));
    }

    #[test]
    fn test_validate_count_valid() {
        // 测试有效数量
        assert!(validate_count(2));
        assert!(validate_count(3));
        assert!(validate_count(4));
    }

    #[test]
    fn test_validate_count_invalid() {
        // 测试无效数量
        assert!(!validate_count(0));
        assert!(!validate_count(1));
        assert!(!validate_count(5));
        assert!(!validate_count(100));
    }

    #[test]
    fn test_process_max_valid() {
        // 测试有效输入
        let nums = vec![3, 7, 2];
        assert_eq!(process_max(&nums), Ok(7));
    }

    #[test]
    fn test_process_max_with_zero() {
        // 测试包含零的输入
        let nums = vec![1, 0, 3];
        assert!(process_max(&nums).is_err());
    }

    #[test]
    fn test_process_max_with_negative() {
        // 测试包含负数的输入
        let nums = vec![1, -2, 3];
        assert!(process_max(&nums).is_err());
    }

    #[test]
    fn test_process_max_empty() {
        // 测试空输入
        let nums: Vec<i32> = vec![];
        assert!(process_max(&nums).is_err());
    }

    #[test]
    fn test_process_max_two_numbers_case() {
        // 测试原C++的case 2场景
        let nums = vec![15, 23];
        assert_eq!(process_max(&nums), Ok(23));
    }

    #[test]
    fn test_process_max_three_numbers_case() {
        // 测试原C++的case 3场景
        let nums = vec![42, 17, 89];
        assert_eq!(process_max(&nums), Ok(89));
    }

    #[test]
    fn test_process_max_four_numbers_case() {
        // 测试原C++的case 4场景
        let nums = vec![12, 45, 23, 67];
        assert_eq!(process_max(&nums), Ok(67));
    }

    #[test]
    fn test_find_max_large_numbers() {
        // 测试大数字
        let nums = vec![1000000, 999999, 1000001];
        assert_eq!(find_max(&nums), Some(&1000001));
    }

    #[test]
    fn test_find_max_ascending_order() {
        // 测试升序排列
        let nums = vec![1, 2, 3, 4];
        assert_eq!(find_max(&nums), Some(&4));
    }

    #[test]
    fn test_find_max_descending_order() {
        // 测试降序排列
        let nums = vec![4, 3, 2, 1];
        assert_eq!(find_max(&nums), Some(&4));
    }

    #[test]
    fn test_validate_positive_single_positive() {
        // 测试单个正整数
        assert!(validate_positive(&[1]));
    }

    #[test]
    fn test_validate_positive_single_negative() {
        // 测试单个负整数
        assert!(!validate_positive(&[-1]));
    }

    #[test]
    fn test_validate_positive_single_zero() {
        // 测试单个零
        assert!(!validate_positive(&[0]));
    }
}
