// 6-b3: Binary string to decimal conversion
// Original: 2052526 信15 白俊豪
//
// 问题描述: 读取一个由0/1组成的字符串（长度不超过32），将其作为二进制数转换为十进制
//
// Rust改进:
// 1. 使用u32::from_str_radix()标准库函数，避免手动计算幂次
// 2. 使用Result类型处理解析错误，比C++的隐式错误处理更安全
// 3. 提取验证和转换逻辑为纯函数，便于测试和复用
// 4. 使用chars().all()验证输入，比C风格字符串遍历更简洁
// 5. 避免使用pow()浮点运算，直接用位运算或标准库整数解析
// 6. 添加边界检查：32位二进制串对应u32范围，防止溢出
// 7. 使用迭代器fold实现手动转换，展示函数式编程优势

use std::io::{self, Write};

/// 验证字符串是否为有效的二进制串
///
/// # Arguments
/// * `s` - 待验证的字符串
///
/// # Returns
/// * `true` - 字符串只包含'0'和'1'
/// * `false` - 字符串包含其他字符
///
/// # Examples
/// ```
/// assert!(is_valid_binary("101010"));
/// assert!(is_valid_binary("0"));
/// assert!(!is_valid_binary("102"));
/// assert!(!is_valid_binary("abc"));
/// ```
fn is_valid_binary(s: &str) -> bool {
    // Rust改进: 使用chars().all()迭代器方法，比C风格循环更简洁
    !s.is_empty() && s.chars().all(|c| c == '0' || c == '1')
}

/// 使用标准库将二进制字符串转换为十进制数
///
/// # Arguments
/// * `binary_str` - 二进制字符串（只包含'0'和'1'）
///
/// # Returns
/// * `Ok(u32)` - 转换成功，返回十进制值
/// * `Err(String)` - 转换失败，返回错误信息
///
/// # Examples
/// ```
/// assert_eq!(binary_to_decimal_stdlib("101").unwrap(), 5);
/// assert_eq!(binary_to_decimal_stdlib("1111").unwrap(), 15);
/// assert_eq!(binary_to_decimal_stdlib("0").unwrap(), 0);
/// ```
fn binary_to_decimal_stdlib(binary_str: &str) -> Result<u32, String> {
    // 验证输入
    if !is_valid_binary(binary_str) {
        return Err("输入包含非0/1字符".to_string());
    }

    // 检查长度限制（32位）
    if binary_str.len() > 32 {
        return Err("二进制串长度超过32位".to_string());
    }

    // Rust改进: 使用标准库u32::from_str_radix()，安全且高效
    // 比手动计算pow(2, n)更可靠，避免浮点精度问题
    u32::from_str_radix(binary_str, 2)
        .map_err(|e| format!("解析失败: {}", e))
}

/// 手动实现二进制到十进制的转换（用于教学和测试）
///
/// # Arguments
/// * `binary_str` - 二进制字符串
///
/// # Returns
/// * `Ok(u32)` - 转换成功
/// * `Err(String)` - 转换失败
///
/// # Algorithm
/// 从左到右遍历每个字符，累加值 = 累加值 * 2 + 当前位
/// 例如: "101" -> ((0*2+1)*2+0)*2+1 = 5
#[cfg(test)]
fn binary_to_decimal_manual(binary_str: &str) -> Result<u32, String> {
    if !is_valid_binary(binary_str) {
        return Err("输入包含非0/1字符".to_string());
    }

    if binary_str.len() > 32 {
        return Err("二进制串长度超过32位".to_string());
    }

    // Rust改进: 使用fold迭代器实现累加，比C++的循环更函数式
    // fold从初始值0开始，对每个字符执行: acc * 2 + digit
    let result = binary_str
        .chars()
        .try_fold(0u32, |acc, c| {
            let digit = c.to_digit(10).ok_or("无效字符")?;
            acc.checked_mul(2)
                .and_then(|v| v.checked_add(digit))
                .ok_or("数值溢出")
        })?;

    Ok(result)
}

/// 使用位运算实现二进制到十进制转换
///
/// # Arguments
/// * `binary_str` - 二进制字符串
///
/// # Returns
/// * `Ok(u32)` - 转换成功
/// * `Err(String)` - 转换失败
///
/// # Algorithm
/// 使用左移运算符<<代替乘以2，使用|代替加法
#[cfg(test)]
fn binary_to_decimal_bitwise(binary_str: &str) -> Result<u32, String> {
    if !is_valid_binary(binary_str) {
        return Err("输入包含非0/1字符".to_string());
    }

    if binary_str.len() > 32 {
        return Err("二进制串长度超过32位".to_string());
    }

    // Rust改进: 使用位运算，性能最优
    let result = binary_str
        .chars()
        .try_fold(0u32, |acc, c| {
            let bit = if c == '1' { 1 } else { 0 };
            acc.checked_shl(1)
                .and_then(|v| v.checked_add(bit))
                .ok_or("数值溢出")
        })?;

    Ok(result)
}

fn main() {
    println!("请输入一个0/1组成的字符串，长度不超过32");

    // Rust改进: 使用String而非固定大小字符数组，更灵活
    let mut input = String::new();
    io::stdout().flush().unwrap();

    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("读取输入失败: {}", e);
        return;
    }

    let binary_str = input.trim();

    // Rust改进: 使用match表达式处理Result，清晰表达成功/失败分支
    match binary_to_decimal_stdlib(binary_str) {
        Ok(decimal) => println!("{}", decimal),
        Err(e) => eprintln!("错误: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_binary() {
        // 测试有效的二进制串
        assert!(is_valid_binary("0"));
        assert!(is_valid_binary("1"));
        assert!(is_valid_binary("101010"));
        assert!(is_valid_binary("11111111"));

        // 测试无效的二进制串
        assert!(!is_valid_binary(""));
        assert!(!is_valid_binary("102"));
        assert!(!is_valid_binary("abc"));
        assert!(!is_valid_binary("1 0 1"));
        assert!(!is_valid_binary("1.0"));
    }

    #[test]
    fn test_binary_to_decimal_basic() {
        // 测试基本转换
        assert_eq!(binary_to_decimal_stdlib("0").unwrap(), 0);
        assert_eq!(binary_to_decimal_stdlib("1").unwrap(), 1);
        assert_eq!(binary_to_decimal_stdlib("10").unwrap(), 2);
        assert_eq!(binary_to_decimal_stdlib("11").unwrap(), 3);
        assert_eq!(binary_to_decimal_stdlib("100").unwrap(), 4);
        assert_eq!(binary_to_decimal_stdlib("101").unwrap(), 5);
        assert_eq!(binary_to_decimal_stdlib("110").unwrap(), 6);
        assert_eq!(binary_to_decimal_stdlib("111").unwrap(), 7);
    }

    #[test]
    fn test_binary_to_decimal_powers_of_two() {
        // 测试2的幂次
        assert_eq!(binary_to_decimal_stdlib("1").unwrap(), 1);
        assert_eq!(binary_to_decimal_stdlib("10").unwrap(), 2);
        assert_eq!(binary_to_decimal_stdlib("100").unwrap(), 4);
        assert_eq!(binary_to_decimal_stdlib("1000").unwrap(), 8);
        assert_eq!(binary_to_decimal_stdlib("10000").unwrap(), 16);
        assert_eq!(binary_to_decimal_stdlib("100000").unwrap(), 32);
        assert_eq!(binary_to_decimal_stdlib("1000000").unwrap(), 64);
        assert_eq!(binary_to_decimal_stdlib("10000000").unwrap(), 128);
    }

    #[test]
    fn test_binary_to_decimal_all_ones() {
        // 测试全1的情况
        assert_eq!(binary_to_decimal_stdlib("1").unwrap(), 1);
        assert_eq!(binary_to_decimal_stdlib("11").unwrap(), 3);
        assert_eq!(binary_to_decimal_stdlib("111").unwrap(), 7);
        assert_eq!(binary_to_decimal_stdlib("1111").unwrap(), 15);
        assert_eq!(binary_to_decimal_stdlib("11111").unwrap(), 31);
        assert_eq!(binary_to_decimal_stdlib("111111").unwrap(), 63);
        assert_eq!(binary_to_decimal_stdlib("1111111").unwrap(), 127);
        assert_eq!(binary_to_decimal_stdlib("11111111").unwrap(), 255);
    }

    #[test]
    fn test_binary_to_decimal_max_32bit() {
        // 测试32位最大值
        let max_32bit = "11111111111111111111111111111111"; // 2^32 - 1
        assert_eq!(binary_to_decimal_stdlib(max_32bit).unwrap(), u32::MAX);
    }

    #[test]
    fn test_binary_to_decimal_leading_zeros() {
        // 测试前导零
        assert_eq!(binary_to_decimal_stdlib("0001").unwrap(), 1);
        assert_eq!(binary_to_decimal_stdlib("00101").unwrap(), 5);
        assert_eq!(binary_to_decimal_stdlib("000000001").unwrap(), 1);
    }

    #[test]
    fn test_binary_to_decimal_invalid_input() {
        // 测试无效输入
        assert!(binary_to_decimal_stdlib("").is_err());
        assert!(binary_to_decimal_stdlib("102").is_err());
        assert!(binary_to_decimal_stdlib("abc").is_err());
        assert!(binary_to_decimal_stdlib("1 0 1").is_err());
        assert!(binary_to_decimal_stdlib("1.0").is_err());
    }

    #[test]
    fn test_binary_to_decimal_too_long() {
        // 测试超过32位的输入
        let too_long = "111111111111111111111111111111111"; // 33位
        assert!(binary_to_decimal_stdlib(too_long).is_err());
    }

    #[test]
    fn test_all_implementations_match() {
        // 测试三种实现方式结果一致
        let test_cases = vec![
            "0", "1", "10", "11", "101", "1010", "11111111",
            "10101010", "11110000", "100000001",
        ];

        for binary_str in test_cases {
            let stdlib_result = binary_to_decimal_stdlib(binary_str).unwrap();
            let manual_result = binary_to_decimal_manual(binary_str).unwrap();
            let bitwise_result = binary_to_decimal_bitwise(binary_str).unwrap();

            assert_eq!(
                stdlib_result, manual_result,
                "stdlib和manual实现在'{}'时结果不一致",
                binary_str
            );
            assert_eq!(
                stdlib_result, bitwise_result,
                "stdlib和bitwise实现在'{}'时结果不一致",
                binary_str
            );
        }
    }

    #[test]
    fn test_manual_implementation_correctness() {
        // 专门测试手动实现的正确性
        assert_eq!(binary_to_decimal_manual("101").unwrap(), 5);
        assert_eq!(binary_to_decimal_manual("1111").unwrap(), 15);
        assert_eq!(binary_to_decimal_manual("10000").unwrap(), 16);
        assert_eq!(binary_to_decimal_manual("11111111").unwrap(), 255);
    }

    #[test]
    fn test_bitwise_implementation_correctness() {
        // 专门测试位运算实现的正确性
        assert_eq!(binary_to_decimal_bitwise("101").unwrap(), 5);
        assert_eq!(binary_to_decimal_bitwise("1111").unwrap(), 15);
        assert_eq!(binary_to_decimal_bitwise("10000").unwrap(), 16);
        assert_eq!(binary_to_decimal_bitwise("11111111").unwrap(), 255);
    }

    #[test]
    fn test_overflow_detection() {
        // 测试溢出检测（虽然32位内不会溢出，但测试边界）
        let max_valid = "11111111111111111111111111111111"; // 32位全1
        assert!(binary_to_decimal_stdlib(max_valid).is_ok());

        let too_long = "111111111111111111111111111111111"; // 33位
        assert!(binary_to_decimal_stdlib(too_long).is_err());
    }

    #[test]
    fn test_specific_examples() {
        // 测试一些具体的例子
        assert_eq!(binary_to_decimal_stdlib("1010").unwrap(), 10);
        assert_eq!(binary_to_decimal_stdlib("10100").unwrap(), 20);
        assert_eq!(binary_to_decimal_stdlib("11001").unwrap(), 25);
        assert_eq!(binary_to_decimal_stdlib("110010").unwrap(), 50);
        assert_eq!(binary_to_decimal_stdlib("1100100").unwrap(), 100);
        assert_eq!(binary_to_decimal_stdlib("11111111").unwrap(), 255);
        assert_eq!(binary_to_decimal_stdlib("100000000").unwrap(), 256);
    }
}
