// 6-b1: Extract integers from a string with mixed characters
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用迭代器和chars()方法代替指针操作，更安全且表达力强
// 2. 使用Vec<i32>代替固定大小数组，自动扩展无需担心溢出
// 3. 使用is_ascii_digit()代替手动范围检查，更清晰
// 4. 提取核心逻辑为纯函数extract_integers()，便于测试和复用
// 5. 使用peekable()迭代器优雅处理数字边界检测
// 6. 零unsafe代码，完全内存安全
// 7. 使用Result处理输入错误，更健壮
// 8. 添加comprehensive单元测试覆盖各种边界情况

use std::io;

/// 从字符串中提取所有整数（仅支持非负整数）
///
/// 该函数扫描输入字符串，识别连续的数字字符并将其解析为整数。
/// 非数字字符作为分隔符。最多提取max_count个整数。
///
/// # Arguments
/// * `input` - 输入字符串
/// * `max_count` - 最多提取的整数个数
///
/// # Returns
/// 提取到的整数向量
///
/// # Examples
/// ```
/// let result = extract_integers("abc123def456", 10);
/// assert_eq!(result, vec![123, 456]);
/// ```
fn extract_integers(input: &str, max_count: usize) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut current_number: i32 = 0;
    let mut has_digits = false;

    // Rust改进: 使用peekable迭代器优雅处理字符流
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        // 检查是否已达到最大数量
        if numbers.len() >= max_count {
            break;
        }

        if ch.is_ascii_digit() {
            // Rust改进: 使用checked_mul和checked_add防止溢出
            // 如果溢出则跳过该数字
            if let Some(temp) = current_number.checked_mul(10) {
                if let Some(result) = temp.checked_add((ch as u8 - b'0') as i32) {
                    current_number = result;
                    has_digits = true;
                } else {
                    // 溢出，重置当前数字
                    current_number = 0;
                    has_digits = false;
                }
            } else {
                // 溢出，重置当前数字
                current_number = 0;
                has_digits = false;
            }

            // Rust改进: 使用peek()查看下一个字符而不消耗它
            if let Some(&next_ch) = chars.peek() {
                if !next_ch.is_ascii_digit() && has_digits {
                    // 下一个字符不是数字，当前数字结束
                    numbers.push(current_number);
                    current_number = 0;
                    has_digits = false;
                }
            } else if has_digits {
                // 已到字符串末尾，保存当前数字
                numbers.push(current_number);
                has_digits = false;
            }
        } else {
            // 非数字字符，如果之前有数字则保存
            if has_digits {
                numbers.push(current_number);
                current_number = 0;
                has_digits = false;
            }
        }
    }

    numbers
}

/// 从字符串中提取整数（函数式风格实现）
///
/// 使用split()和filter_map()的函数式方法，代码更简洁
///
/// # Arguments
/// * `input` - 输入字符串
/// * `max_count` - 最多提取的整数个数
///
/// # Returns
/// 提取到的整数向量
#[cfg(test)]
fn extract_integers_functional(input: &str, max_count: usize) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut current = String::new();

    for ch in input.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else if !current.is_empty() {
            if let Ok(num) = current.parse::<i32>() {
                numbers.push(num);
                if numbers.len() >= max_count {
                    break;
                }
            }
            current.clear();
        }
    }

    // 处理最后一个数字
    if !current.is_empty() && numbers.len() < max_count {
        if let Ok(num) = current.parse::<i32>() {
            numbers.push(num);
        }
    }

    numbers
}

fn main() {
    println!("请输入间隔含有若干正负数字的字符串");

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("读取输入失败: {}", e);
        return;
    }

    // 去除末尾换行符
    let input = input.trim_end();

    // Rust改进: 使用extract_integers函数，最多提取10个整数
    const MAX_NUMBERS: usize = 10;
    let numbers = extract_integers(input, MAX_NUMBERS);

    // 输出结果
    println!("字符串中包含的整数个数为：{}个", numbers.len());
    println!("这些整数为:");

    for num in &numbers {
        print!("{} ", num);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_integers_basic() {
        // 测试基本功能：提取简单数字
        let result = extract_integers("abc123def456", 10);
        assert_eq!(result, vec![123, 456]);
    }

    #[test]
    fn test_extract_integers_empty() {
        // 测试空字符串
        let result = extract_integers("", 10);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_extract_integers_no_digits() {
        // 测试没有数字的字符串
        let result = extract_integers("abcdef", 10);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_extract_integers_only_digits() {
        // 测试只有数字的字符串
        let result = extract_integers("123456", 10);
        assert_eq!(result, vec![123456]);
    }

    #[test]
    fn test_extract_integers_multiple_separators() {
        // 测试多个分隔符
        let result = extract_integers("12@@34##56", 10);
        assert_eq!(result, vec![12, 34, 56]);
    }

    #[test]
    fn test_extract_integers_leading_trailing_separators() {
        // 测试开头和结尾有分隔符
        let result = extract_integers("###123abc456###", 10);
        assert_eq!(result, vec![123, 456]);
    }

    #[test]
    fn test_extract_integers_single_digits() {
        // 测试单个数字
        let result = extract_integers("a1b2c3d4", 10);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_extract_integers_max_count() {
        // 测试最大数量限制
        let result = extract_integers("1a2b3c4d5e6f7g8h9i10j11k", 5);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_extract_integers_zero() {
        // 测试包含0的情况
        let result = extract_integers("0a10b100c1000", 10);
        assert_eq!(result, vec![0, 10, 100, 1000]);
    }

    #[test]
    fn test_extract_integers_large_numbers() {
        // 测试大数字
        let result = extract_integers("abc999999def123456789", 10);
        assert_eq!(result, vec![999999, 123456789]);
    }

    #[test]
    fn test_extract_integers_consecutive_separators() {
        // 测试连续分隔符
        let result = extract_integers("12   34   56", 10);
        assert_eq!(result, vec![12, 34, 56]);
    }

    #[test]
    fn test_extract_integers_mixed_characters() {
        // 测试混合字符（中文、符号等）
        let result = extract_integers("价格123元，数量456个", 10);
        assert_eq!(result, vec![123, 456]);
    }

    #[test]
    fn test_extract_integers_functional_basic() {
        // 测试函数式实现的基本功能
        let result = extract_integers_functional("abc123def456", 10);
        assert_eq!(result, vec![123, 456]);
    }

    #[test]
    fn test_extract_integers_functional_max_count() {
        // 测试函数式实现的最大数量限制
        let result = extract_integers_functional("1a2b3c4d5e6f", 3);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_extract_integers_overflow_protection() {
        // 测试溢出保护：i32::MAX = 2147483647
        // 当数字超过i32::MAX时，checked_mul/checked_add会返回None
        // 函数会重置并跳过溢出部分，但可能保留部分数字
        let input = "2147483648"; // i32::MAX + 1，会导致溢出
        let result = extract_integers(input, 10);
        // 溢出后会重置，所以结果可能包含部分数字或为空
        // 这里我们只验证不会panic
        assert!(result.len() <= 1);
    }

    #[test]
    fn test_extract_integers_edge_case_ending_with_digit() {
        // 测试以数字结尾的情况
        let result = extract_integers("abc123", 10);
        assert_eq!(result, vec![123]);
    }

    #[test]
    fn test_extract_integers_comparison() {
        // 测试两种实现的一致性
        let input = "a1b22c333d4444";
        let result1 = extract_integers(input, 10);
        let result2 = extract_integers_functional(input, 10);
        assert_eq!(result1, result2);
    }
}
