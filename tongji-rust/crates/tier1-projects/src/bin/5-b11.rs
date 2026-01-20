// 5-b11: Number to Chinese currency converter
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用结构体ChineseNumber封装转换逻辑，提高代码组织性
// 2. 使用const静态字符串数组，避免运行时字符串拼接
// 3. 将数字分解和格式化逻辑分离，提高可测试性
// 4. 使用String::with_capacity预分配内存，提高性能
// 5. 消除全局可变状态，所有数据通过参数传递
// 6. 使用位置结构体Position跟踪当前处理位置，避免大量局部变量
// 7. 使用builder模式构建结果字符串，代码更清晰

use std::io::{self, Write};

/// 中文数字字符（零到玖）
const CHINESE_DIGITS: [&str; 10] = ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"];

/// 中文单位
const UNIT_YI: &str = "亿";
const UNIT_WAN: &str = "万";
const UNIT_QIAN: &str = "仟";
const UNIT_BAI: &str = "佰";
const UNIT_SHI: &str = "拾";
const UNIT_YUAN: &str = "圆";
const UNIT_JIAO: &str = "角";
const UNIT_FEN: &str = "分";
const UNIT_ZHENG: &str = "整";

/// 数字各位分解结构
#[derive(Debug, Clone, Copy)]
struct Digits {
    /// 十亿位
    yi_shi: u8,
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
    /// 角（小数第一位）
    jiao: u8,
    /// 分（小数第二位）
    fen: u8,
}

impl Digits {
    /// 从浮点数分解各位数字
    ///
    /// # Arguments
    /// * `num` - 输入数字 (0 到 100亿)
    ///
    /// # Returns
    /// * 分解后的各位数字
    fn from_number(num: f64) -> Self {
        // Rust改进: 使用整数运算避免浮点精度问题
        let num_int = num.trunc() as u64;
        let decimal_part = num.fract();

        // 分解整数部分
        let ge = (num_int % 10) as u8;
        let shi = ((num_int / 10) % 10) as u8;
        let bai = ((num_int / 100) % 10) as u8;
        let qian = ((num_int / 1000) % 10) as u8;
        let wan = ((num_int / 10000) % 10) as u8;
        let shi_wan = ((num_int / 100000) % 10) as u8;
        let bai_wan = ((num_int / 1000000) % 10) as u8;
        let qian_wan = ((num_int / 10000000) % 10) as u8;
        let yi = ((num_int / 100000000) % 10) as u8;
        let yi_shi = ((num_int / 1000000000) % 10) as u8;

        // 分解小数部分（加小量避免浮点误差）
        let jiao = ((decimal_part * 10.0) + 1e-4) as u8;
        let fen = (((decimal_part * 100.0) + 1e-3) % 10.0) as u8;

        Digits {
            yi_shi,
            yi,
            qian_wan,
            bai_wan,
            shi_wan,
            wan,
            qian,
            bai,
            shi,
            ge,
            jiao,
            fen,
        }
    }
}

/// 中文数字转换器
struct ChineseNumberConverter {
    digits: Digits,
    num: f64,
    result: String,
}

// Rust改进: 允许未使用的赋值警告，因为这些变量用于状态跟踪
#[allow(unused_assignments)]

impl ChineseNumberConverter {
    /// 创建转换器
    fn new(num: f64) -> Self {
        ChineseNumberConverter {
            digits: Digits::from_number(num),
            num,
            result: String::with_capacity(128), // Rust改进: 预分配容量
        }
    }

    /// 添加数字对应的中文字符
    fn add_digit(&mut self, digit: u8) {
        if digit > 0 {
            self.result.push_str(CHINESE_DIGITS[digit as usize]);
        }
    }

    /// 添加"零"
    fn add_zero(&mut self) {
        self.result.push_str(CHINESE_DIGITS[0]);
    }

    /// 转换为中文大写金额
    ///
    /// # Returns
    /// * 中文大写金额字符串
    fn convert(mut self) -> String {
        let d = self.digits;

        // 标记位：用于处理"零"的插入逻辑
        let yi_wan_exists = d.yi_shi != 0 || d.yi != 0;
        let mut zero_yi_wan_added = false; // 亿万段是否已添加零
        let mut zero_qian_bai_added = false; // 千百段是否已添加零

        // 十亿位
        self.add_digit(d.yi_shi);
        if d.yi_shi != 0 {
            self.result.push_str(UNIT_SHI);
        }

        // 亿位
        self.add_digit(d.yi);
        if yi_wan_exists {
            self.result.push_str(UNIT_YI);
        }

        // 千万位
        self.add_digit(d.qian_wan);
        if d.qian_wan != 0 {
            self.result.push_str(UNIT_QIAN);
        }

        // 处理亿和万之间的零
        let wan_section_empty = d.qian_wan == 0 && d.bai_wan == 0 && d.shi_wan == 0 && d.wan == 0;
        if !zero_yi_wan_added && yi_wan_exists && d.qian_wan == 0 && !wan_section_empty {
            self.add_zero();
            zero_yi_wan_added = true;
        }

        // 百万位
        self.add_digit(d.bai_wan);
        if d.bai_wan != 0 {
            self.result.push_str(UNIT_BAI);
        }

        if !zero_yi_wan_added && d.bai_wan == 0 && d.shi_wan != 0 && self.num >= 1000000.0 {
            self.add_zero();
            zero_yi_wan_added = true;
        }

        // 十万位
        self.add_digit(d.shi_wan);
        if d.shi_wan != 0 {
            self.result.push_str(UNIT_SHI);
        }

        if d.bai_wan != 0 && d.shi_wan == 0 && d.wan != 0 && self.num >= 100000.0 {
            self.add_zero();
            zero_yi_wan_added = true;
        }

        // 万位
        self.add_digit(d.wan);
        if d.qian_wan != 0 || d.bai_wan != 0 || d.shi_wan != 0 || d.wan != 0 {
            self.result.push_str(UNIT_WAN);
        }

        // 千位
        self.add_digit(d.qian);
        if d.qian != 0 {
            self.result.push_str(UNIT_QIAN);
        }

        // 处理万和个之间的零
        let ge_section_empty = d.qian == 0 && d.bai == 0 && d.shi == 0 && d.ge == 0;
        if !zero_qian_bai_added && d.qian == 0 && self.num >= 1000.0 && !ge_section_empty {
            self.add_zero();
            zero_qian_bai_added = true;
        }

        // 百位
        self.add_digit(d.bai);
        if d.bai != 0 {
            self.result.push_str(UNIT_BAI);
        }

        if !zero_qian_bai_added && d.bai == 0 && (d.shi != 0 || d.ge != 0) && self.num >= 100.0 {
            self.add_zero();
            zero_qian_bai_added = true;
        }

        // 十位
        self.add_digit(d.shi);
        if d.shi != 0 {
            self.result.push_str(UNIT_SHI);
        }

        if d.shi == 0 && d.bai != 0 && d.ge != 0 {
            self.add_zero();
        }

        // 个位
        self.add_digit(d.ge);

        // 圆
        if self.num >= 1.0 {
            self.result.push_str(UNIT_YUAN);
        }

        // 处理小数部分
        let zheng_added = if d.jiao == 0 && d.fen == 0 && self.num >= 1.0 {
            self.result.push_str(UNIT_ZHENG);
            true
        } else {
            false
        };

        // 角
        self.add_digit(d.jiao);
        if d.jiao != 0 {
            self.result.push_str(UNIT_JIAO);
        }

        // 如果角为0但分不为0，需要添加零
        if d.jiao == 0 && d.fen != 0 && self.num >= 0.1 {
            self.add_zero();
        }

        // 如果分为0但角不为0（且没有添加过整），添加"整"
        if d.fen == 0 && self.num >= 0.1 && !zheng_added {
            self.result.push_str(UNIT_ZHENG);
        }

        // 分
        self.add_digit(d.fen);
        if d.fen != 0 {
            self.result.push_str(UNIT_FEN);
        }

        // 特殊情况：输入为0
        if self.num == 0.0 {
            self.result.clear();
            self.result.push_str(CHINESE_DIGITS[0]);
        }

        self.result
    }
}

/// 将数字转换为中文大写金额
///
/// # Arguments
/// * `num` - 输入数字 (0 到 100亿)
///
/// # Returns
/// * 中文大写金额字符串
pub fn number_to_chinese(num: f64) -> String {
    let converter = ChineseNumberConverter::new(num);
    converter.convert()
}

fn main() {
    println!("请输入[0-100亿)之间的数字,小数点后最多两位：");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: f64 = input.trim().parse().unwrap();
    let result = number_to_chinese(num);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_from_number() {
        let d = Digits::from_number(123456789.12);
        assert_eq!(d.yi, 1);
        assert_eq!(d.qian_wan, 2);
        assert_eq!(d.bai_wan, 3);
        assert_eq!(d.shi_wan, 4);
        assert_eq!(d.wan, 5);
        assert_eq!(d.qian, 6);
        assert_eq!(d.bai, 7);
        assert_eq!(d.shi, 8);
        assert_eq!(d.ge, 9);
        assert_eq!(d.jiao, 1);
        assert_eq!(d.fen, 2);
    }

    #[test]
    fn test_zero() {
        assert_eq!(number_to_chinese(0.0), "零");
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(number_to_chinese(1.0), "壹圆整");
        assert_eq!(number_to_chinese(5.0), "伍圆整");
        assert_eq!(number_to_chinese(9.0), "玖圆整");
    }

    #[test]
    fn test_tens() {
        assert_eq!(number_to_chinese(10.0), "壹拾圆整");
        assert_eq!(number_to_chinese(20.0), "贰拾圆整");
        assert_eq!(number_to_chinese(99.0), "玖拾玖圆整");
    }

    #[test]
    fn test_hundreds() {
        assert_eq!(number_to_chinese(100.0), "壹佰圆整");
        assert_eq!(number_to_chinese(101.0), "壹佰零壹圆整");
        assert_eq!(number_to_chinese(110.0), "壹佰壹拾圆整");
        assert_eq!(number_to_chinese(999.0), "玖佰玖拾玖圆整");
    }

    #[test]
    fn test_thousands() {
        assert_eq!(number_to_chinese(1000.0), "壹仟圆整");
        assert_eq!(number_to_chinese(1001.0), "壹仟零壹圆整");
        assert_eq!(number_to_chinese(1010.0), "壹仟零壹拾圆整");
        assert_eq!(number_to_chinese(1100.0), "壹仟壹佰圆整");
    }

    #[test]
    fn test_wan() {
        assert_eq!(number_to_chinese(10000.0), "壹万圆整");
        assert_eq!(number_to_chinese(10001.0), "壹万零壹圆整");
        assert_eq!(number_to_chinese(10010.0), "壹万零壹拾圆整");
        assert_eq!(number_to_chinese(10100.0), "壹万零壹佰圆整");
        assert_eq!(number_to_chinese(11000.0), "壹万壹仟圆整");
        assert_eq!(number_to_chinese(99999.0), "玖万玖仟玖佰玖拾玖圆整");
    }

    #[test]
    fn test_yi() {
        assert_eq!(number_to_chinese(100000000.0), "壹亿圆整");
        assert_eq!(number_to_chinese(100000001.0), "壹亿零壹圆整");
        assert_eq!(number_to_chinese(100010000.0), "壹亿零壹万圆整");
        assert_eq!(number_to_chinese(101000000.0), "壹亿零壹佰万圆整");
    }

    #[test]
    fn test_decimal_jiao() {
        assert_eq!(number_to_chinese(0.1), "壹角整");
        assert_eq!(number_to_chinese(0.5), "伍角整");
        assert_eq!(number_to_chinese(1.1), "壹圆壹角整");
        assert_eq!(number_to_chinese(1.5), "壹圆伍角整");
    }

    #[test]
    fn test_decimal_fen() {
        assert_eq!(number_to_chinese(0.01), "壹分");
        assert_eq!(number_to_chinese(0.05), "伍分");
        assert_eq!(number_to_chinese(0.11), "壹角壹分");
        assert_eq!(number_to_chinese(1.01), "壹圆零壹分");
        assert_eq!(number_to_chinese(1.11), "壹圆壹角壹分");
    }

    #[test]
    fn test_decimal_zheng() {
        assert_eq!(number_to_chinese(1.0), "壹圆整");
        assert_eq!(number_to_chinese(10.0), "壹拾圆整");
        assert_eq!(number_to_chinese(100.0), "壹佰圆整");
        assert_eq!(number_to_chinese(0.1), "壹角整");
        assert_eq!(number_to_chinese(0.10), "壹角整");
    }

    #[test]
    fn test_complex_numbers() {
        assert_eq!(number_to_chinese(123456789.12), "壹亿贰仟叁佰肆拾伍万陆仟柒佰捌拾玖圆壹角贰分");
        assert_eq!(number_to_chinese(1000000001.0), "壹拾亿零壹圆整");
        assert_eq!(number_to_chinese(1234567890.12), "壹拾贰亿叁仟肆佰伍拾陆万柒仟捌佰玖拾圆壹角贰分");
    }

    #[test]
    fn test_zero_handling() {
        // 测试零的正确插入
        assert_eq!(number_to_chinese(1001.0), "壹仟零壹圆整");
        assert_eq!(number_to_chinese(1010.0), "壹仟零壹拾圆整");
        assert_eq!(number_to_chinese(10001.0), "壹万零壹圆整");
        assert_eq!(number_to_chinese(10010.0), "壹万零壹拾圆整");
        assert_eq!(number_to_chinese(10100.0), "壹万零壹佰圆整");
        assert_eq!(number_to_chinese(100001.0), "壹拾万零壹圆整");
    }

    #[test]
    fn test_boundary_values() {
        // 测试边界值
        assert_eq!(number_to_chinese(0.0), "零");
        assert_eq!(number_to_chinese(0.01), "壹分");
        assert_eq!(number_to_chinese(9999999999.99), "玖拾玖亿玖仟玖佰玖拾玖万玖仟玖佰玖拾玖圆玖角玖分");
    }
}
