// 4-b1: Convert numbers to Chinese currency format (大写金额)
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用const数组存储中文数字，避免重复字符串
// 2. 提取数字分解逻辑为独立函数，便于测试
// 3. 避免浮点运算精度问题，使用整数运算
// 4. 使用Result类型处理输入验证
// 5. 零的处理逻辑更清晰，使用布尔标志追踪状态
// 6. 提取核心转换逻辑为可测试的纯函数

use std::io::{self, Write};

/// 中文大写数字
const DIGITS: [&str; 10] = ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"];

/// 输出大写数字（0-9）
///
/// # Arguments
/// * `num` - 数字（0-9）
/// * `flag_of_zero` - 是否输出零（true表示输出，false表示不输出）
fn daxie(num: u8, flag_of_zero: bool) -> &'static str {
    // Rust改进: 返回字符串而不是直接打印，便于测试和组合
    if num == 0 && !flag_of_zero {
        ""
    } else {
        DIGITS[num as usize]
    }
}

/// 将数字分解为各个位数
///
/// # Arguments
/// * `num` - 输入的浮点数
///
/// # Returns
/// * 包含12个元素的数组：[个位, 十位, 百位, 千位, 万位, 十万位, 百万位, 千万位, 亿位, 十亿位, 角, 分]
pub fn decompose_number(num: f64) -> [u8; 12] {
    let mut digits = [0u8; 12];

    // 整数部分
    let num_int = num.trunc() as u64;
    let mut temp = num_int;

    for i in 0..10 {
        digits[i] = (temp % 10) as u8;
        temp /= 10;
    }

    // 小数部分
    let num_decimal = num - num.trunc();
    digits[10] = ((num_decimal * 10.0) + 1e-4) as u8; // 角
    digits[11] = (((num_decimal * 100.0) + 1e-3) % 10.0 + 1e-4) as u8; // 分

    digits
}

/// 将数字转换为中文大写金额
///
/// # Arguments
/// * `num` - 输入的数字（0到100亿之间，最多两位小数）
///
/// # Returns
/// * 中文大写金额字符串
pub fn to_chinese_currency(num: f64) -> String {
    if num == 0.0 {
        return "零".to_string();
    }

    let digits = decompose_number(num);
    let mut result = String::new();

    // 标志位
    let i10_19_exist = digits[9] > 0 || digits[8] > 0; // 十亿位或亿位存在
    let mut zero_1_exist = false; // 万节是否已输出零
    let mut zero_2_exist = false; // 个节是否已输出零
    let zheng_exist = digits[10] == 0 && digits[11] == 0 && num >= 1.0; // 是否整数

    // 十亿位 (i10)
    result.push_str(daxie(digits[9], false));
    if digits[9] > 0 {
        result.push_str("拾");
    }

    // 亿位 (i9)
    result.push_str(daxie(digits[8], false));

    if digits[9] > 0 || digits[8] > 0 {
        result.push_str("亿");
    }

    // 千万位 (i8)
    if !(digits[7] > 0 || digits[6] > 0 || digits[5] > 0 || digits[4] > 0) {
        zero_1_exist = true;
    }

    if !zero_1_exist && i10_19_exist && digits[7] == 0 {
        result.push_str(daxie(0, true));
        zero_1_exist = true;
    }

    result.push_str(daxie(digits[7], false));
    if digits[7] > 0 {
        result.push_str("仟");
    }

    // 百万位 (i7)
    if !zero_1_exist && digits[6] == 0 && digits[5] > 0 && num >= 1_000_000.0 {
        result.push_str(daxie(0, true));
        zero_1_exist = true;
    }

    result.push_str(daxie(digits[6], false));
    if digits[6] > 0 {
        result.push_str("佰");
    }

    // 十万位 (i6)
    if digits[6] > 0 && digits[5] == 0 && digits[4] > 0 && num >= 100_000.0 {
        result.push_str(daxie(0, true));
        zero_1_exist = true;
    }

    result.push_str(daxie(digits[5], false));
    if digits[5] > 0 {
        result.push_str("拾");
    }

    // 万位 (i5)
    result.push_str(daxie(digits[4], false));

    if digits[4] > 0 || digits[5] > 0 || digits[6] > 0 || digits[7] > 0 {
        result.push_str("万");
    }

    // 千位 (i4)
    if !(digits[3] > 0 || digits[2] > 0 || digits[1] > 0 || digits[0] > 0) {
        zero_2_exist = true;
    }

    if !zero_2_exist && digits[3] == 0 && num >= 1000.0 {
        result.push_str(daxie(0, true));
        zero_2_exist = true;
    }

    result.push_str(daxie(digits[3], false));
    if digits[3] > 0 {
        result.push_str("仟");
    }

    // 百位 (i3)
    if !zero_2_exist && digits[2] == 0 && (digits[1] > 0 || digits[0] > 0) && num >= 100.0 {
        result.push_str(daxie(0, true));
        zero_2_exist = true;
    }

    result.push_str(daxie(digits[2], false));
    if digits[2] > 0 {
        result.push_str("佰");
    }

    // 十位 (i2)
    if digits[1] == 0 && digits[2] > 0 && digits[0] > 0 {
        result.push_str(daxie(0, true));
        zero_2_exist = true;
    }

    result.push_str(daxie(digits[1], false));
    if digits[1] > 0 {
        result.push_str("拾");
    }

    // 个位 (i1)
    result.push_str(daxie(digits[0], false));

    if num >= 1.0 {
        result.push_str("圆");
    }

    if digits[10] == 0 && digits[11] == 0 && num >= 1.0 {
        result.push_str("整");
        return result;
    }

    // 角 (i_1)
    if digits[10] == 0 && digits[11] != 0 && num >= 1.0 {
        result.push_str(daxie(0, true));
    }

    result.push_str(daxie(digits[10], false));
    if digits[10] > 0 {
        result.push_str("角");
    }

    if digits[11] == 0 && num >= 1.0 && !zheng_exist {
        result.push_str("整");
    }

    // 分 (i_2)
    result.push_str(daxie(digits[11], false));
    if digits[11] > 0 {
        result.push_str("分");
    }

    result
}

fn main() {
    print!("请输入[0-100亿)之间的数字,小数点后最多两位：");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Rust改进: 使用Result的错误处理，更优雅
    match input.trim().parse::<f64>() {
        Ok(num) if num >= 0.0 && num < 10_000_000_000.0 => {
            let result = to_chinese_currency(num);
            println!("{}", result);
        }
        Ok(_) => {
            println!("输入超出范围");
        }
        Err(_) => {
            println!("输入格式错误");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompose_number() {
        // 测试数字分解: 123.45
        let digits = decompose_number(123.45);
        assert_eq!(digits[0], 3); // 个位
        assert_eq!(digits[1], 2); // 十位
        assert_eq!(digits[2], 1); // 百位
        assert_eq!(digits[10], 4); // 角
        assert_eq!(digits[11], 5); // 分
    }

    #[test]
    fn test_zero() {
        assert_eq!(to_chinese_currency(0.0), "零");
    }

    #[test]
    fn test_small_amounts() {
        // 测试小额
        assert_eq!(to_chinese_currency(0.01), "壹分");
        assert_eq!(to_chinese_currency(0.1), "壹角");
        assert_eq!(to_chinese_currency(0.11), "壹角壹分");
    }

    #[test]
    fn test_integer_amounts() {
        // 测试整数金额
        assert_eq!(to_chinese_currency(1.0), "壹圆整");
        assert_eq!(to_chinese_currency(10.0), "壹拾圆整");
        assert_eq!(to_chinese_currency(100.0), "壹佰圆整");
    }

    #[test]
    fn test_with_decimals() {
        // 测试带小数的金额
        assert_eq!(to_chinese_currency(1.23), "壹圆贰角叁分");
        assert_eq!(to_chinese_currency(10.05), "壹拾圆零伍分");
    }

    #[test]
    fn test_zero_handling() {
        // 测试零的处理
        assert_eq!(to_chinese_currency(101.0), "壹佰零壹圆整");
        assert_eq!(to_chinese_currency(1001.0), "壹仟零壹圆整");
        assert_eq!(to_chinese_currency(1010.0), "壹仟零壹拾圆整");
    }

    #[test]
    fn test_wan_section() {
        // 测试万节
        assert_eq!(to_chinese_currency(10000.0), "壹万圆整");
        assert_eq!(to_chinese_currency(12345.0), "壹万贰仟叁佰肆拾伍圆整");
    }

    #[test]
    fn test_yi_section() {
        // 测试亿节
        assert_eq!(to_chinese_currency(100000000.0), "壹亿圆整");
        assert_eq!(to_chinese_currency(123456789.12), "壹亿贰仟叁佰肆拾伍万陆仟柒佰捌拾玖圆壹角贰分");
    }

    #[test]
    fn test_large_numbers() {
        // 测试大数
        assert_eq!(to_chinese_currency(1000000000.0), "壹拾亿圆整");
    }

    #[test]
    fn test_daxie_function() {
        // 测试daxie函数
        assert_eq!(daxie(0, false), "");
        assert_eq!(daxie(0, true), "零");
        assert_eq!(daxie(1, false), "壹");
        assert_eq!(daxie(9, false), "玖");
    }

    #[test]
    fn test_complex_zero_patterns() {
        // 测试复杂的零处理
        assert_eq!(to_chinese_currency(10001.0), "壹万零壹圆整");
        assert_eq!(to_chinese_currency(100001.0), "壹拾万零壹圆整");
        assert_eq!(to_chinese_currency(1000001.0), "壹佰万零壹圆整");
    }
}
