// 3-b10: Input validation - read integer between 0-100
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用loop + match + break返回值，避免未初始化变量
// 2. 使用Result的ok()和filter()链式调用，更简洁
// 3. 使用if let Some()模式匹配，更符合Rust习惯
// 4. 提取验证逻辑为独立函数，便于单元测试

use std::io::{self, Write};

/// 验证输入字符串是否为0-100之间的整数
///
/// # Arguments
/// * `input` - 输入字符串
///
/// # Returns
/// * `Some(i32)` - 如果输入有效且在范围内
/// * `None` - 如果输入无效或超出范围
fn validate_input(input: &str) -> Option<i32> {
    // Rust改进: 使用链式调用 parse().ok().filter() 更简洁
    // ok()将Result<T,E>转换为Option<T>
    // filter()检查范围，不符合则返回None
    input.trim().parse::<i32>().ok().filter(|&n| n >= 0 && n <= 100)
}

fn main() {
    // Rust改进: loop可以直接返回值，不需要先声明未初始化的变量
    let x = loop {
        print!("请输入x的值(0-100)：");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(num) = validate_input(&input) {
            break num; // Rust改进: loop可以break返回值
        }
        // 如果解析失败或不在范围内，继续循环
    };

    println!("x={}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input_boundary() {
        // 测试边界值
        assert_eq!(validate_input("0"), Some(0));
        assert_eq!(validate_input("100"), Some(100));
    }

    #[test]
    fn test_valid_input_middle() {
        // 测试中间值
        assert_eq!(validate_input("50"), Some(50));
        assert_eq!(validate_input("1"), Some(1));
        assert_eq!(validate_input("99"), Some(99));
    }

    #[test]
    fn test_invalid_input_out_of_range() {
        // 测试超出范围
        assert_eq!(validate_input("-1"), None);
        assert_eq!(validate_input("101"), None);
        assert_eq!(validate_input("-100"), None);
        assert_eq!(validate_input("1000"), None);
    }

    #[test]
    fn test_invalid_input_not_number() {
        // 测试非数字输入
        assert_eq!(validate_input("abc"), None);
        assert_eq!(validate_input("12.5"), None);
        assert_eq!(validate_input(""), None);
        assert_eq!(validate_input("  "), None);
    }

    #[test]
    fn test_input_with_whitespace() {
        // 测试带空格的输入
        assert_eq!(validate_input("  50  "), Some(50));
        assert_eq!(validate_input("\n100\n"), Some(100));
        assert_eq!(validate_input("\t0\t"), Some(0));
    }

    #[test]
    fn test_invalid_input_mixed() {
        // 测试混合无效输入
        assert_eq!(validate_input("50abc"), None);
        assert_eq!(validate_input("abc50"), None);
        assert_eq!(validate_input("5 0"), None);
    }
}

