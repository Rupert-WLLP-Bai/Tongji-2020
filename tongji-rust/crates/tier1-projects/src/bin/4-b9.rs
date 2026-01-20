// 4-b9: Integer digit decomposition using recursion
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用递归处理数字分解,避免循环
// 2. 使用i32::MIN常量代替硬编码的-2147483648
// 3. 使用checked_neg()安全处理溢出,避免未定义行为
// 4. 使用Vec<char>收集输出,便于测试和验证
// 5. 提取核心逻辑为纯函数,返回结果而非直接打印
// 6. 特殊处理i32::MIN的边界情况,保证正确性

use std::io;

/// 将整数分解为各位数字(递归实现)
///
/// # Arguments
/// * `num` - 要分解的整数
///
/// # Returns
/// * `Vec<char>` - 分解后的字符序列(包括负号和空格)
///
/// # Examples
/// ```
/// assert_eq!(convert(123), vec!['1', ' ', '2', ' ', '3', ' ']);
/// assert_eq!(convert(-45), vec!['-', ' ', '4', ' ', '5', ' ']);
/// assert_eq!(convert(0), vec!['0']);
/// ```
fn convert(num: i32) -> Vec<char> {
    let mut result = Vec::new();
    convert_impl(num, &mut result);
    result
}

/// 递归实现的内部函数
///
/// # Arguments
/// * `num` - 当前处理的数字
/// * `result` - 累积结果的可变引用
///
/// # Rust改进说明
/// - 使用Vec<char>而非直接cout,使函数可测试
/// - 使用i32::MIN常量提高可读性
/// - 使用checked_neg()处理溢出,更安全
fn convert_impl(num: i32, result: &mut Vec<char>) {
    if num == 0 {
        // 特殊情况: 输入为0
        result.push('0');
    } else if num == i32::MIN {
        // Rust改进: 特殊处理i32::MIN (-2147483648)
        // 因为-i32::MIN会溢出,需要特殊处理
        result.push('-');
        result.push(' ');

        // 将-2147483648分解为-2147483647-1来处理
        let adjusted = i32::MAX; // 2147483647
        convert_impl(adjusted / 10000, result); // 处理高位: 214748

        // 处理低位: 3647 + 1 = 3648
        let low = adjusted % 10000 + 1;
        convert_impl(low, result);
    } else if num > 0 {
        // 正数情况: 递归处理
        if num / 10 != 0 {
            convert_impl(num / 10, result);
        }
        result.push(char::from_digit((num % 10) as u32, 10).unwrap());
        result.push(' ');
    } else {
        // 负数情况: 转为正数后递归
        result.push('-');
        result.push(' ');

        // Rust改进: 使用checked_neg()安全处理,虽然这里已排除MIN
        let positive = num.checked_neg().unwrap();

        // 只有当高位不为0时才递归处理高位
        if positive / 10 != 0 {
            convert_impl(positive / 10, result);
        }
        result.push(char::from_digit((positive % 10) as u32, 10).unwrap());
        result.push(' ');
    }
}

fn main() {
    println!("请输入一个整数");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().expect("请输入有效的整数");

    let result = convert(n);
    for ch in result {
        print!("{}", ch);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        // 测试零
        assert_eq!(convert(0), vec!['0']);
    }

    #[test]
    fn test_single_digit_positive() {
        // 测试单位正数
        assert_eq!(convert(5), vec!['5', ' ']);
        assert_eq!(convert(9), vec!['9', ' ']);
    }

    #[test]
    fn test_multi_digit_positive() {
        // 测试多位正数
        assert_eq!(convert(123), vec!['1', ' ', '2', ' ', '3', ' ']);
        assert_eq!(convert(456789), vec!['4', ' ', '5', ' ', '6', ' ', '7', ' ', '8', ' ', '9', ' ']);
    }

    #[test]
    fn test_single_digit_negative() {
        // 测试单位负数
        assert_eq!(convert(-5), vec!['-', ' ', '5', ' ']);
        assert_eq!(convert(-9), vec!['-', ' ', '9', ' ']);
    }

    #[test]
    fn test_multi_digit_negative() {
        // 测试多位负数
        assert_eq!(convert(-123), vec!['-', ' ', '1', ' ', '2', ' ', '3', ' ']);
        assert_eq!(convert(-456), vec!['-', ' ', '4', ' ', '5', ' ', '6', ' ']);
    }

    #[test]
    fn test_boundary_max() {
        // 测试i32最大值: 2147483647
        let result = convert(i32::MAX);
        let expected = vec!['2', ' ', '1', ' ', '4', ' ', '7', ' ', '4', ' ', '8', ' ', '3', ' ', '6', ' ', '4', ' ', '7', ' '];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_boundary_min() {
        // 测试i32最小值: -2147483648
        // 这是最复杂的边界情况
        let result = convert(i32::MIN);
        let expected = vec!['-', ' ', '2', ' ', '1', ' ', '4', ' ', '7', ' ', '4', ' ', '8', ' ', '3', ' ', '6', ' ', '4', ' ', '8', ' '];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_powers_of_ten() {
        // 测试10的幂次
        assert_eq!(convert(10), vec!['1', ' ', '0', ' ']);
        assert_eq!(convert(100), vec!['1', ' ', '0', ' ', '0', ' ']);
        assert_eq!(convert(1000), vec!['1', ' ', '0', ' ', '0', ' ', '0', ' ']);
        assert_eq!(convert(-10), vec!['-', ' ', '1', ' ', '0', ' ']);
        assert_eq!(convert(-100), vec!['-', ' ', '1', ' ', '0', ' ', '0', ' ']);
    }

    #[test]
    fn test_near_boundary() {
        // 测试接近边界的值
        assert_eq!(convert(2147483646), vec!['2', ' ', '1', ' ', '4', ' ', '7', ' ', '4', ' ', '8', ' ', '3', ' ', '6', ' ', '4', ' ', '6', ' ']);
        assert_eq!(convert(-2147483647), vec!['-', ' ', '2', ' ', '1', ' ', '4', ' ', '7', ' ', '4', ' ', '8', ' ', '3', ' ', '6', ' ', '4', ' ', '7', ' ']);
    }
}
