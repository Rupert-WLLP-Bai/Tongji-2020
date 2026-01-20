// 3-b3: Extract digits from integer (1-30000)
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用数组存储各位数字，避免多个临时变量
// 2. 使用迭代器和enumerate()优雅地处理位数提取
// 3. 提取核心逻辑为纯函数，便于单元测试
// 4. 使用const数组定义位名称，类型安全且易维护
// 5. 添加输入验证，确保数字在有效范围内

use std::io::{self, Write};

/// 提取整数的各位数字（个位、十位、百位、千位、万位）
///
/// # Arguments
/// * `num` - 输入整数，应在1-30000范围内
///
/// # Returns
/// * 长度为5的数组，依次为[个位, 十位, 百位, 千位, 万位]
///
/// # Examples
/// ```
/// let digits = extract_digits(12345);
/// assert_eq!(digits, [5, 4, 3, 2, 1]);
/// ```
fn extract_digits(num: i32) -> [i32; 5] {
    // Rust改进: 使用数组而非多个变量，更简洁
    let mut digits = [0; 5];
    let mut n = num;

    // Rust改进: 使用for循环和索引，比C++的重复代码更优雅
    for i in 0..5 {
        digits[i] = n % 10;
        n /= 10;
    }

    digits
}

/// 验证输入是否为1-30000之间的整数
///
/// # Arguments
/// * `input` - 输入字符串
///
/// # Returns
/// * `Some(i32)` - 如果输入有效且在范围内
/// * `None` - 如果输入无效或超出范围
fn validate_input(input: &str) -> Option<i32> {
    // Rust改进: 链式调用parse().ok().filter()更简洁
    input
        .trim()
        .parse::<i32>()
        .ok()
        .filter(|&n| n >= 1 && n <= 30000)
}

fn main() {
    // Rust改进: 使用const数组定义位名称，类型安全
    const PLACE_NAMES: [&str; 5] = ["个位", "十位", "百位", "千位", "万位"];

    // Rust改进: loop可以直接返回值，避免未初始化变量
    let num = loop {
        print!("请输入一个[1..30000]间的整数：");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(n) = validate_input(&input) {
            break n;
        }
        println!("输入无效，请输入1到30000之间的整数");
    };

    let digits = extract_digits(num);

    // Rust改进: 使用迭代器反向遍历，配合enumerate()优雅输出
    // rev()反转迭代器，从万位到个位输出
    for (i, &digit) in digits.iter().enumerate().rev() {
        println!("{} : {}", PLACE_NAMES[i], digit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digits_single_digit() {
        // 测试个位数
        assert_eq!(extract_digits(5), [5, 0, 0, 0, 0]);
        assert_eq!(extract_digits(9), [9, 0, 0, 0, 0]);
    }

    #[test]
    fn test_extract_digits_two_digits() {
        // 测试两位数
        assert_eq!(extract_digits(12), [2, 1, 0, 0, 0]);
        assert_eq!(extract_digits(99), [9, 9, 0, 0, 0]);
    }

    #[test]
    fn test_extract_digits_three_digits() {
        // 测试三位数
        assert_eq!(extract_digits(123), [3, 2, 1, 0, 0]);
        assert_eq!(extract_digits(999), [9, 9, 9, 0, 0]);
    }

    #[test]
    fn test_extract_digits_four_digits() {
        // 测试四位数
        assert_eq!(extract_digits(1234), [4, 3, 2, 1, 0]);
        assert_eq!(extract_digits(9999), [9, 9, 9, 9, 0]);
    }

    #[test]
    fn test_extract_digits_five_digits() {
        // 测试五位数
        assert_eq!(extract_digits(12345), [5, 4, 3, 2, 1]);
        assert_eq!(extract_digits(30000), [0, 0, 0, 0, 3]);
        assert_eq!(extract_digits(10001), [1, 0, 0, 0, 1]);
    }

    #[test]
    fn test_extract_digits_boundary() {
        // 测试边界值
        assert_eq!(extract_digits(1), [1, 0, 0, 0, 0]);
        assert_eq!(extract_digits(30000), [0, 0, 0, 0, 3]);
    }

    #[test]
    fn test_extract_digits_with_zeros() {
        // 测试包含0的数字
        assert_eq!(extract_digits(10), [0, 1, 0, 0, 0]);
        assert_eq!(extract_digits(100), [0, 0, 1, 0, 0]);
        assert_eq!(extract_digits(1000), [0, 0, 0, 1, 0]);
        assert_eq!(extract_digits(10000), [0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_validate_input_valid() {
        // 测试有效输入
        assert_eq!(validate_input("1"), Some(1));
        assert_eq!(validate_input("30000"), Some(30000));
        assert_eq!(validate_input("15000"), Some(15000));
        assert_eq!(validate_input("  12345  "), Some(12345));
    }

    #[test]
    fn test_validate_input_out_of_range() {
        // 测试超出范围
        assert_eq!(validate_input("0"), None);
        assert_eq!(validate_input("-1"), None);
        assert_eq!(validate_input("30001"), None);
        assert_eq!(validate_input("100000"), None);
    }

    #[test]
    fn test_validate_input_invalid() {
        // 测试无效输入
        assert_eq!(validate_input("abc"), None);
        assert_eq!(validate_input("12.5"), None);
        assert_eq!(validate_input(""), None);
        assert_eq!(validate_input("  "), None);
        assert_eq!(validate_input("123abc"), None);
    }

    #[test]
    fn test_extract_digits_comprehensive() {
        // 综合测试：验证提取的数字可以重建原数字
        for num in [1, 99, 123, 1234, 12345, 30000] {
            let digits = extract_digits(num);
            let reconstructed =
                digits[0] + digits[1] * 10 + digits[2] * 100 + digits[3] * 1000 + digits[4] * 10000;
            assert_eq!(reconstructed, num, "Failed to reconstruct {}", num);
        }
    }
}
