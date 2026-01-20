// 3-b8: Change making algorithm - calculate minimum bills and coins
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用整数运算(分为单位)避免浮点精度问题
// 2. 使用结构体封装找零结果，提高代码可读性
// 3. 使用数组和迭代器简化输出逻辑，避免重复代码
// 4. 提取核心算法为纯函数，便于单元测试
// 5. 使用const定义面额，提高可维护性

use std::io::{self, Write};

/// 找零结果结构体
#[derive(Debug, PartialEq, Eq)]
struct ChangeResult {
    fifty_yuan: u32,      // 50元
    twenty_yuan: u32,     // 20元
    ten_yuan: u32,        // 10元
    five_yuan: u32,       // 5元
    one_yuan: u32,        // 1元
    five_jiao: u32,       // 5角
    one_jiao: u32,        // 1角
    five_fen: u32,        // 5分
    two_fen: u32,         // 2分
    one_fen: u32,         // 1分
}

impl ChangeResult {
    /// 计算总张数
    fn total_count(&self) -> u32 {
        self.fifty_yuan
            + self.twenty_yuan
            + self.ten_yuan
            + self.five_yuan
            + self.one_yuan
            + self.five_jiao
            + self.one_jiao
            + self.five_fen
            + self.two_fen
            + self.one_fen
    }

    /// 获取所有非零面额的迭代器
    /// 返回 (面额名称, 张数) 元组
    fn non_zero_denominations(&self) -> Vec<(&str, u32)> {
        let denominations = [
            ("50元", self.fifty_yuan),
            ("20元", self.twenty_yuan),
            ("10元", self.ten_yuan),
            ("5元 ", self.five_yuan),
            ("1元 ", self.one_yuan),
            ("5角 ", self.five_jiao),
            ("1角 ", self.one_jiao),
            ("5分 ", self.five_fen),
            ("2分 ", self.two_fen),
            ("1分 ", self.one_fen),
        ];

        denominations
            .iter()
            .filter(|(_, count)| *count > 0)
            .copied()
            .collect()
    }
}

/// 计算找零方案
///
/// # Arguments
/// * `amount` - 找零金额(元)
///
/// # Returns
/// * `Some(ChangeResult)` - 如果金额有效(非负)
/// * `None` - 如果金额无效(负数)
///
/// # Rust改进
/// 使用整数运算避免浮点精度问题:
/// - 将金额转换为分(乘以100)
/// - 使用整数除法和取模运算
/// - 避免了C++版本中的浮点误差累积
fn calculate_change(amount: f64) -> Option<ChangeResult> {
    if amount < 0.0 {
        return None;
    }

    // Rust改进: 转换为分(整数)避免浮点精度问题
    // 加0.5进行四舍五入，避免浮点误差
    let mut cents = (amount * 100.0 + 0.5) as u32;

    // 贪心算法: 从大到小依次计算每种面额的数量
    let fifty_yuan = cents / 5000;
    cents %= 5000;

    let twenty_yuan = cents / 2000;
    cents %= 2000;

    let ten_yuan = cents / 1000;
    cents %= 1000;

    let five_yuan = cents / 500;
    cents %= 500;

    let one_yuan = cents / 100;
    cents %= 100;

    let five_jiao = cents / 50;
    cents %= 50;

    let one_jiao = cents / 10;
    cents %= 10;

    let five_fen = cents / 5;
    cents %= 5;

    let two_fen = cents / 2;
    cents %= 2;

    let one_fen = cents; // 剩余的都是1分

    Some(ChangeResult {
        fifty_yuan,
        twenty_yuan,
        ten_yuan,
        five_yuan,
        one_yuan,
        five_jiao,
        one_jiao,
        five_fen,
        two_fen,
        one_fen,
    })
}

fn main() {
    print!("请输入找零值 : ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let amount: f64 = input.trim().parse().expect("请输入有效的数字");

    if let Some(result) = calculate_change(amount) {
        println!("共{}张找零，具体如下 : ", result.total_count());

        // Rust改进: 使用迭代器简化输出，避免重复的if语句
        for (denomination, count) in result.non_zero_denominations() {
            println!("{} : {}张", denomination, count);
        }
    } else {
        println!("金额不能为负数");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_amount() {
        let result = calculate_change(0.0).unwrap();
        assert_eq!(result.total_count(), 0);
    }

    #[test]
    fn test_single_denomination() {
        // 测试单一面额
        let result = calculate_change(50.0).unwrap();
        assert_eq!(result.fifty_yuan, 1);
        assert_eq!(result.total_count(), 1);

        let result = calculate_change(0.01).unwrap();
        assert_eq!(result.one_fen, 1);
        assert_eq!(result.total_count(), 1);
    }

    #[test]
    fn test_multiple_denominations() {
        // 测试 88.88元
        let result = calculate_change(88.88).unwrap();
        assert_eq!(result.fifty_yuan, 1);
        assert_eq!(result.twenty_yuan, 1);
        assert_eq!(result.ten_yuan, 1);
        assert_eq!(result.five_yuan, 1);
        assert_eq!(result.one_yuan, 3);
        assert_eq!(result.five_jiao, 1);
        assert_eq!(result.one_jiao, 3);
        assert_eq!(result.five_fen, 1);
        assert_eq!(result.two_fen, 1);
        assert_eq!(result.one_fen, 1);
        assert_eq!(result.total_count(), 14);
    }

    #[test]
    fn test_decimal_amounts() {
        // 测试小数金额
        let result = calculate_change(0.88).unwrap();
        assert_eq!(result.five_jiao, 1);
        assert_eq!(result.one_jiao, 3);
        assert_eq!(result.five_fen, 1);
        assert_eq!(result.two_fen, 1);
        assert_eq!(result.one_fen, 1);
        assert_eq!(result.total_count(), 7);
    }

    #[test]
    fn test_large_amount() {
        // 测试大额金额
        let result = calculate_change(999.99).unwrap();
        assert_eq!(result.fifty_yuan, 19);
        assert_eq!(result.twenty_yuan, 2);
        assert_eq!(result.ten_yuan, 0);
        assert_eq!(result.five_yuan, 1);
        assert_eq!(result.one_yuan, 4);
        assert_eq!(result.five_jiao, 1);
        assert_eq!(result.one_jiao, 4);
        assert_eq!(result.five_fen, 1);
        assert_eq!(result.two_fen, 2);
        assert_eq!(result.total_count(), 34);
    }

    #[test]
    fn test_boundary_values() {
        // 测试边界值
        let result = calculate_change(0.99).unwrap();
        assert_eq!(result.five_jiao, 1);
        assert_eq!(result.one_jiao, 4);
        assert_eq!(result.five_fen, 1);
        assert_eq!(result.two_fen, 2);
        assert_eq!(result.total_count(), 8);

        let result = calculate_change(1.00).unwrap();
        assert_eq!(result.one_yuan, 1);
        assert_eq!(result.total_count(), 1);
    }

    #[test]
    fn test_floating_point_precision() {
        // 测试浮点精度问题
        // 0.77 在二进制中无法精确表示，测试是否正确处理
        let result = calculate_change(0.77).unwrap();
        assert_eq!(result.five_jiao, 1);
        assert_eq!(result.one_jiao, 2);
        assert_eq!(result.five_fen, 1);
        assert_eq!(result.two_fen, 1);
        assert_eq!(result.total_count(), 5);
    }

    #[test]
    fn test_negative_amount() {
        // 测试负数金额
        assert_eq!(calculate_change(-1.0), None);
        assert_eq!(calculate_change(-0.01), None);
    }

    #[test]
    fn test_non_zero_denominations() {
        // 测试非零面额筛选
        let result = calculate_change(50.0).unwrap();
        let non_zero = result.non_zero_denominations();
        assert_eq!(non_zero.len(), 1);
        assert_eq!(non_zero[0], ("50元", 1));

        let result = calculate_change(88.88).unwrap();
        let non_zero = result.non_zero_denominations();
        assert_eq!(non_zero.len(), 10); // 所有面额都有
    }

    #[test]
    fn test_total_count() {
        // 测试总数计算
        let result = calculate_change(123.45).unwrap();
        let manual_count = result.fifty_yuan
            + result.twenty_yuan
            + result.ten_yuan
            + result.five_yuan
            + result.one_yuan
            + result.five_jiao
            + result.one_jiao
            + result.five_fen
            + result.two_fen
            + result.one_fen;
        assert_eq!(result.total_count(), manual_count);
    }
}
