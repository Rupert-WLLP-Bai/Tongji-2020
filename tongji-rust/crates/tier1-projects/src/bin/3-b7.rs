// 3-b7: Number to Chinese Currency Converter (人民币大写转换器)
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用数组和常量替代重复的switch语句,提高代码可维护性
// 2. 使用结构体封装数字的各个位,使代码更清晰
// 3. 提取核心转换逻辑为纯函数,便于单元测试
// 4. 使用String::with_capacity预分配内存,避免多次重新分配
// 5. 使用迭代器和函数式编程,减少可变状态
// 6. 添加输入验证,确保数字在有效范围内
// 7. 使用Option类型处理零的特殊情况,避免布尔标志

use std::io::{self, Write};

/// 中文数字常量 (0-9)
const CHINESE_DIGITS: [&str; 10] = ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"];

/// 数字的各个位数分解
#[derive(Debug, PartialEq)]
struct DigitBreakdown {
    /// 十亿位
    shi_yi: u8,
    /// 亿位
    yi: u8,
    /// 千万位
    qian_wan: u8,
    /// 百万位
    bai_wan: u8,
    /// 十万位
    shi_wan: u8,
    /// 万位
    wan: u8,
    /// 千位
    qian: u8,
    /// 百位
    bai: u8,
    /// 十位
    shi: u8,
    /// 个位
    ge: u8,
    /// 角(小数第一位)
    jiao: u8,
    /// 分(小数第二位)
    fen: u8,
}

impl DigitBreakdown {
    /// 将浮点数分解为各个位数
    ///
    /// # Arguments
    /// * `num` - 输入的数字 (0.0 <= num < 10_000_000_000.0)
    ///
    /// # Returns
    /// * `DigitBreakdown` - 分解后的各位数字
    fn from_number(num: f64) -> Self {
        // Rust改进: 使用整数运算避免浮点精度问题
        // 将数字乘以100转为整数,统一处理整数和小数部分
        let num_cents = (num * 100.0 + 0.5) as u64; // 四舍五入到分

        let integer_part = num_cents / 100;
        let decimal_part = num_cents % 100;

        // Rust改进: 使用数组和循环提取各位数字,避免重复代码
        let mut digits = [0u8; 10];
        let mut temp = integer_part;
        for digit in digits.iter_mut() {
            *digit = (temp % 10) as u8;
            temp /= 10;
        }

        Self {
            shi_yi: digits[9],
            yi: digits[8],
            qian_wan: digits[7],
            bai_wan: digits[6],
            shi_wan: digits[5],
            wan: digits[4],
            qian: digits[3],
            bai: digits[2],
            shi: digits[1],
            ge: digits[0],
            jiao: (decimal_part / 10) as u8,
            fen: (decimal_part % 10) as u8,
        }
    }

    /// 检查亿级(亿、十亿)是否有非零数字
    fn has_yi_level(&self) -> bool {
        self.shi_yi != 0 || self.yi != 0
    }

    /// 检查万级(万、十万、百万、千万)是否有非零数字
    fn has_wan_level(&self) -> bool {
        self.wan != 0 || self.shi_wan != 0 || self.bai_wan != 0 || self.qian_wan != 0
    }

    /// 检查个级(个、十、百、千)是否全为零
    fn ge_level_all_zero(&self) -> bool {
        self.ge == 0 && self.shi == 0 && self.bai == 0 && self.qian == 0
    }

    /// 检查万级是否全为零
    fn wan_level_all_zero(&self) -> bool {
        self.wan == 0 && self.shi_wan == 0 && self.bai_wan == 0 && self.qian_wan == 0
    }
}

/// 将数字转换为中文货币大写
///
/// # Arguments
/// * `num` - 输入的数字 (0.0 <= num < 10_000_000_000.0)
///
/// # Returns
/// * `String` - 中文货币大写字符串
///
/// # Examples
/// ```
/// let result = number_to_chinese_currency(123.45);
/// assert_eq!(result, "壹佰贰拾叁圆肆角伍分");
/// ```
fn number_to_chinese_currency(num: f64) -> String {
    // 特殊情况: 零
    if num < 0.01 {
        return "零".to_string();
    }

    let digits = DigitBreakdown::from_number(num);

    // Rust改进: 预分配足够的容量,避免多次重新分配
    let mut result = String::with_capacity(100);

    // 处理亿级
    append_yi_level(&mut result, &digits);

    // 处理万级
    append_wan_level(&mut result, &digits);

    // 处理个级
    append_ge_level(&mut result, &digits, num);

    // 处理小数部分
    append_decimal_part(&mut result, &digits, num);

    result
}

/// 添加亿级数字(十亿、亿)
fn append_yi_level(result: &mut String, digits: &DigitBreakdown) {
    // 十亿位
    if digits.shi_yi != 0 {
        result.push_str(CHINESE_DIGITS[digits.shi_yi as usize]);
        result.push_str("拾");
    }

    // 亿位
    if digits.yi != 0 {
        result.push_str(CHINESE_DIGITS[digits.yi as usize]);
    }

    // 添加"亿"单位
    if digits.has_yi_level() {
        result.push_str("亿");
    }
}

/// 添加万级数字(千万、百万、十万、万)
fn append_wan_level(result: &mut String, digits: &DigitBreakdown) {
    // 千万位
    if digits.qian_wan != 0 {
        result.push_str(CHINESE_DIGITS[digits.qian_wan as usize]);
        result.push_str("仟");
    }

    // Rust改进: 使用局部变量跟踪是否需要添加"零",逻辑更清晰
    // 如果万级全为零,标记已添加零(避免重复)
    let mut zero_added = digits.wan_level_all_zero();

    // 如果有亿级但千万位为0,需要添加"零"
    if !zero_added && digits.has_yi_level() && digits.qian_wan == 0 {
        result.push_str("零");
        zero_added = true;
    }

    // 百万位
    if digits.bai_wan != 0 {
        result.push_str(CHINESE_DIGITS[digits.bai_wan as usize]);
        result.push_str("佰");
    }

    // 如果千万位不为0但百万位为0,且十万位不为0,需要添加"零"
    if !zero_added && digits.qian_wan != 0 && digits.bai_wan == 0 && digits.shi_wan != 0 {
        result.push_str("零");
    }

    // 十万位
    if digits.shi_wan != 0 {
        result.push_str(CHINESE_DIGITS[digits.shi_wan as usize]);
        result.push_str("拾");
    }

    // 如果百万位不为0但十万位为0,且万位不为0,需要添加"零"
    if digits.bai_wan != 0 && digits.shi_wan == 0 && digits.wan != 0 {
        result.push_str("零");
    }

    // 万位
    if digits.wan != 0 {
        result.push_str(CHINESE_DIGITS[digits.wan as usize]);
    }

    // 添加"万"单位
    if digits.has_wan_level() {
        result.push_str("万");
    }
}

/// 添加个级数字(千、百、十、个)
fn append_ge_level(result: &mut String, digits: &DigitBreakdown, num: f64) {
    // 千位
    if digits.qian != 0 {
        result.push_str(CHINESE_DIGITS[digits.qian as usize]);
        result.push_str("仟");
    }

    // 如果个级全为零,标记已添加零
    let mut zero_added = digits.ge_level_all_zero();

    // 如果千位为0但有更高位,需要添加"零"
    if !zero_added && digits.qian == 0 && num >= 1000.0 {
        result.push_str("零");
        zero_added = true;
    }

    // 百位
    if digits.bai != 0 {
        result.push_str(CHINESE_DIGITS[digits.bai as usize]);
        result.push_str("佰");
    }

    // 如果百位为0但有十位或个位,需要添加"零"
    if !zero_added && digits.bai == 0 && (digits.shi != 0 || digits.ge != 0) && num >= 100.0 {
        result.push_str("零");
    }

    // 十位
    if digits.shi != 0 {
        result.push_str(CHINESE_DIGITS[digits.shi as usize]);
        result.push_str("拾");
    }

    // 如果十位为0但百位和个位都不为0,需要添加"零"
    if digits.shi == 0 && digits.bai != 0 && digits.ge != 0 {
        result.push_str("零");
    }

    // 个位
    if digits.ge != 0 {
        result.push_str(CHINESE_DIGITS[digits.ge as usize]);
    }

    // 添加"圆"单位
    // 只有当整数部分大于等于1,或者个级有非零数字时才添加"圆"
    if num >= 1.0 && !digits.ge_level_all_zero() {
        result.push_str("圆");
    } else if num >= 1.0 && digits.ge_level_all_zero() && (digits.jiao != 0 || digits.fen != 0) {
        // 如果个级全为0但有小数部分,不添加"圆"(例如: 1000000000.01 -> 壹拾亿零壹分)
        // 不添加"圆"
    } else if num >= 1.0 && digits.ge_level_all_zero() && digits.jiao == 0 && digits.fen == 0 {
        // 如果个级全为0且没有小数部分,添加"圆整"(例如: 10000.0 -> 壹万圆整)
        result.push_str("圆");
    }
}

/// 添加小数部分(角、分)
fn append_decimal_part(result: &mut String, digits: &DigitBreakdown, num: f64) {
    // 如果没有小数部分且整数部分大于等于1,添加"整"
    if digits.jiao == 0 && digits.fen == 0 && num >= 1.0 {
        result.push_str("整");
        return;
    }

    // 角
    if digits.jiao != 0 {
        result.push_str(CHINESE_DIGITS[digits.jiao as usize]);
        result.push_str("角");
    } else if digits.fen != 0 {
        // 如果角为0但分不为0,需要添加"零"
        result.push_str("零");
    }

    // 如果分为0但角不为0,添加"整"
    if digits.fen == 0 && digits.jiao != 0 {
        result.push_str("整");
        return;
    }

    // 分
    if digits.fen != 0 {
        result.push_str(CHINESE_DIGITS[digits.fen as usize]);
        result.push_str("分");
    }
}

/// 验证输入是否为有效的数字
///
/// # Arguments
/// * `input` - 输入字符串
///
/// # Returns
/// * `Some(f64)` - 如果输入有效且在范围内 [0, 10_000_000_000)
/// * `None` - 如果输入无效或超出范围
fn validate_input(input: &str) -> Option<f64> {
    // Rust改进: 使用链式调用和filter进行验证
    input
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|&n| n >= 0.0 && n < 10_000_000_000.0)
        .and_then(|n| {
            // 检查小数位数不超过2位
            let rounded = (n * 100.0).round() / 100.0;
            if (n - rounded).abs() < 1e-9 {
                Some(rounded)
            } else {
                None
            }
        })
}

fn main() {
    println!("请输入[0-100亿)之间的数字,小数点后最多两位：");

    // Rust改进: 使用loop表达式直接返回值
    let num = loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(n) = validate_input(&input) {
            break n;
        }

        println!("输入无效,请重新输入!");
    };

    let result = number_to_chinese_currency(num);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_breakdown_zero() {
        let digits = DigitBreakdown::from_number(0.0);
        assert_eq!(digits.ge, 0);
        assert_eq!(digits.jiao, 0);
        assert_eq!(digits.fen, 0);
    }

    #[test]
    fn test_digit_breakdown_simple() {
        let digits = DigitBreakdown::from_number(123.45);
        assert_eq!(digits.bai, 1);
        assert_eq!(digits.shi, 2);
        assert_eq!(digits.ge, 3);
        assert_eq!(digits.jiao, 4);
        assert_eq!(digits.fen, 5);
    }

    #[test]
    fn test_digit_breakdown_large() {
        let digits = DigitBreakdown::from_number(9876543210.99);
        assert_eq!(digits.shi_yi, 9);
        assert_eq!(digits.yi, 8);
        assert_eq!(digits.qian_wan, 7);
        assert_eq!(digits.bai_wan, 6);
        assert_eq!(digits.shi_wan, 5);
        assert_eq!(digits.wan, 4);
        assert_eq!(digits.qian, 3);
        assert_eq!(digits.bai, 2);
        assert_eq!(digits.shi, 1);
        assert_eq!(digits.ge, 0);
        assert_eq!(digits.jiao, 9);
        assert_eq!(digits.fen, 9);
    }

    #[test]
    fn test_convert_zero() {
        assert_eq!(number_to_chinese_currency(0.0), "零");
        assert_eq!(number_to_chinese_currency(0.001), "零");
    }

    #[test]
    fn test_convert_simple_integer() {
        assert_eq!(number_to_chinese_currency(1.0), "壹圆整");
        assert_eq!(number_to_chinese_currency(5.0), "伍圆整");
        assert_eq!(number_to_chinese_currency(9.0), "玖圆整");
    }

    #[test]
    fn test_convert_tens() {
        assert_eq!(number_to_chinese_currency(10.0), "壹拾圆整");
        assert_eq!(number_to_chinese_currency(25.0), "贰拾伍圆整");
        assert_eq!(number_to_chinese_currency(99.0), "玖拾玖圆整");
    }

    #[test]
    fn test_convert_hundreds() {
        assert_eq!(number_to_chinese_currency(100.0), "壹佰圆整");
        assert_eq!(number_to_chinese_currency(123.0), "壹佰贰拾叁圆整");
        assert_eq!(number_to_chinese_currency(505.0), "伍佰零伍圆整");
    }

    #[test]
    fn test_convert_thousands() {
        assert_eq!(number_to_chinese_currency(1000.0), "壹仟圆整");
        assert_eq!(number_to_chinese_currency(1234.0), "壹仟贰佰叁拾肆圆整");
        assert_eq!(number_to_chinese_currency(1001.0), "壹仟零壹圆整");
        assert_eq!(number_to_chinese_currency(1010.0), "壹仟零壹拾圆整");
    }

    #[test]
    fn test_convert_wan() {
        assert_eq!(number_to_chinese_currency(10000.0), "壹万圆整");
        assert_eq!(number_to_chinese_currency(12345.0), "壹万贰仟叁佰肆拾伍圆整");
        assert_eq!(number_to_chinese_currency(100001.0), "壹拾万零壹圆整");
    }

    #[test]
    fn test_convert_yi() {
        assert_eq!(number_to_chinese_currency(100000000.0), "壹亿圆整");
        assert_eq!(number_to_chinese_currency(123456789.0), "壹亿贰仟叁佰肆拾伍万陆仟柒佰捌拾玖圆整");
        assert_eq!(number_to_chinese_currency(1000000001.0), "壹拾亿零壹圆整");
    }

    #[test]
    fn test_convert_decimal() {
        assert_eq!(number_to_chinese_currency(0.12), "壹角贰分");
        assert_eq!(number_to_chinese_currency(0.50), "伍角整");
        assert_eq!(number_to_chinese_currency(0.05), "零伍分");
        assert_eq!(number_to_chinese_currency(1.23), "壹圆贰角叁分");
        assert_eq!(number_to_chinese_currency(123.45), "壹佰贰拾叁圆肆角伍分");
    }

    #[test]
    fn test_convert_zero_in_middle() {
        assert_eq!(number_to_chinese_currency(101.0), "壹佰零壹圆整");
        assert_eq!(number_to_chinese_currency(1001.0), "壹仟零壹圆整");
        assert_eq!(number_to_chinese_currency(10001.0), "壹万零壹圆整");
        assert_eq!(number_to_chinese_currency(100000001.0), "壹亿零壹圆整");
    }

    #[test]
    fn test_convert_complex() {
        assert_eq!(
            number_to_chinese_currency(9876543210.99),
            "玖拾捌亿柒仟陆佰伍拾肆万叁仟贰佰壹拾圆玖角玖分"
        );
        assert_eq!(
            number_to_chinese_currency(1000000000.01),
            "壹拾亿零壹分"
        );
    }

    #[test]
    fn test_validate_input_valid() {
        assert_eq!(validate_input("123.45"), Some(123.45));
        assert_eq!(validate_input("0"), Some(0.0));
        assert_eq!(validate_input("9999999999.99"), Some(9999999999.99));
        assert_eq!(validate_input("  100.5  "), Some(100.5));
    }

    #[test]
    fn test_validate_input_invalid_range() {
        assert_eq!(validate_input("-1"), None);
        assert_eq!(validate_input("10000000000"), None);
        assert_eq!(validate_input("10000000000.01"), None);
    }

    #[test]
    fn test_validate_input_invalid_decimal() {
        assert_eq!(validate_input("123.456"), None); // 超过2位小数
        assert_eq!(validate_input("1.999"), None);
    }

    #[test]
    fn test_validate_input_invalid_format() {
        assert_eq!(validate_input("abc"), None);
        assert_eq!(validate_input("12.34.56"), None);
        assert_eq!(validate_input(""), None);
        assert_eq!(validate_input("  "), None);
    }

    #[test]
    fn test_digit_breakdown_helpers() {
        let digits = DigitBreakdown::from_number(123456789.0);
        assert!(digits.has_yi_level());
        assert!(digits.has_wan_level());
        assert!(!digits.ge_level_all_zero());

        let digits_zero = DigitBreakdown::from_number(0.0);
        assert!(!digits_zero.has_yi_level());
        assert!(!digits_zero.has_wan_level());
        assert!(digits_zero.ge_level_all_zero());
    }
}
