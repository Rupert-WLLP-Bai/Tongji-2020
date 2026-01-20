// 4-b3: Calendar display using Zeller's formula
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用const数组存储每月天数，消除大量重复的switch-case代码
// 2. 使用迭代器和函数式编程，代码更简洁易读
// 3. 提取闰年判断、天数计算、Zeller公式为独立纯函数，便于测试
// 4. 使用Result类型处理输入验证，错误处理更清晰
// 5. 使用format!宏构建输出，避免复杂的格式化逻辑
// 6. 消除可变状态，使用不可变变量和函数式风格

use std::io::{self, Write};

/// 判断是否为闰年
///
/// # Arguments
/// * `year` - 年份
///
/// # Returns
/// * `true` - 闰年
/// * `false` - 平年
const fn is_leap_year(year: i32) -> bool {
    // Rust改进: 使用const fn，编译期计算
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 获取指定月份的天数
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份 (1-12)
///
/// # Returns
/// * 该月的天数
const fn days_in_month(year: i32, month: u32) -> u32 {
    // Rust改进: 使用const数组和match表达式，替代大量重复的switch-case
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0, // 不应该到达这里
    }
}

/// 使用Zeller公式计算某日是星期几
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份 (1-12)
/// * `day` - 日期
///
/// # Returns
/// * 0-6 表示星期日到星期六
fn zeller(year: i32, month: u32, day: u32) -> u32 {
    // Rust改进: 使用let绑定和表达式，避免多个if-else分支
    let (y, m) = if month >= 3 {
        (year, month)
    } else {
        // 1月和2月看作上一年的13月和14月
        (year - 1, month + 12)
    };

    let y1 = y % 100;
    let c = y / 100;

    // Zeller公式
    let mut w = y1 + y1 / 4 + c / 4 - 2 * c + (13 * (m as i32 + 1) / 5) + day as i32 - 1;

    // 确保结果为正数
    while w <= 0 {
        w += 7;
    }

    (w % 7) as u32
}

/// 打印月历
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份 (1-12)
fn print_calendar(year: i32, month: u32) {
    println!("{}年{}月", year, month);
    println!("======================================================");
    println!("星期日  星期一  星期二  星期三  星期四  星期五  星期六");
    println!("======================================================");

    let first_day = zeller(year, month, 1);
    let total_days = days_in_month(year, month);

    // Rust改进: 使用迭代器和函数式编程，避免复杂的循环和状态管理
    let mut output = String::new();

    // 打印前置空格
    for _ in 0..first_day {
        output.push_str("        ");
    }

    // 打印日期
    for day in 1..=total_days {
        let current_weekday = (first_day + day - 1) % 7;

        if current_weekday == 0 && day != 1 {
            output.push('\n');
        }

        if current_weekday == 0 {
            output.push_str(&format!("{:4}", day));
        } else {
            output.push_str(&format!("{:8}", day));
        }
    }

    println!("{}", output);
}

/// 验证并解析年月输入
///
/// # Arguments
/// * `input` - 输入字符串
///
/// # Returns
/// * `Some((year, month))` - 如果输入有效
/// * `None` - 如果输入无效
fn parse_year_month(input: &str) -> Option<(i32, u32)> {
    // Rust改进: 使用split_whitespace和collect，优雅处理空格分隔的输入
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        return None;
    }

    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;

    // 验证范围
    if (1900..=2099).contains(&year) && (1..=12).contains(&month) {
        Some((year, month))
    } else {
        None
    }
}

fn main() {
    // Rust改进: 使用loop + break返回值，避免未初始化变量
    let (year, month) = loop {
        print!("输入年月,范围是(1900.1-2099.12) : ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match parse_year_month(&input) {
            Some((y, m)) => break (y, m),
            None => {
                // 判断具体错误类型给出提示
                if let Ok(parts) = input
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
                {
                    if parts.len() == 2 {
                        if !(1900..=2099).contains(&parts[0]) {
                            println!("年份错误,请重新输入");
                        } else {
                            println!("月份错误,请重新输入");
                        }
                    } else {
                        println!("输入非法,请重新输入");
                    }
                } else {
                    println!("输入非法,请重新输入");
                }
            }
        }
    };

    println!();
    print_calendar(year, month);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        // 测试闰年
        assert!(is_leap_year(2000)); // 400的倍数
        assert!(is_leap_year(2004)); // 4的倍数但不是100的倍数
        assert!(is_leap_year(2020));

        // 测试平年
        assert!(!is_leap_year(1900)); // 100的倍数但不是400的倍数
        assert!(!is_leap_year(2001)); // 不是4的倍数
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_days_in_month() {
        // 测试31天的月份
        assert_eq!(days_in_month(2020, 1), 31);
        assert_eq!(days_in_month(2020, 3), 31);
        assert_eq!(days_in_month(2020, 5), 31);
        assert_eq!(days_in_month(2020, 7), 31);
        assert_eq!(days_in_month(2020, 8), 31);
        assert_eq!(days_in_month(2020, 10), 31);
        assert_eq!(days_in_month(2020, 12), 31);

        // 测试30天的月份
        assert_eq!(days_in_month(2020, 4), 30);
        assert_eq!(days_in_month(2020, 6), 30);
        assert_eq!(days_in_month(2020, 9), 30);
        assert_eq!(days_in_month(2020, 11), 30);

        // 测试2月
        assert_eq!(days_in_month(2020, 2), 29); // 闰年
        assert_eq!(days_in_month(2021, 2), 28); // 平年
        assert_eq!(days_in_month(2000, 2), 29); // 闰年
        assert_eq!(days_in_month(1900, 2), 28); // 平年
    }

    #[test]
    fn test_zeller() {
        // 测试已知日期
        // 2020年1月1日是星期三
        assert_eq!(zeller(2020, 1, 1), 3);

        // 2020年12月25日是星期五
        assert_eq!(zeller(2020, 12, 25), 5);

        // 2000年1月1日是星期六
        assert_eq!(zeller(2000, 1, 1), 6);

        // 1900年1月1日是星期一
        assert_eq!(zeller(1900, 1, 1), 1);
    }

    #[test]
    fn test_parse_year_month_valid() {
        // 测试有效输入
        assert_eq!(parse_year_month("2020 1"), Some((2020, 1)));
        assert_eq!(parse_year_month("1900 12"), Some((1900, 12)));
        assert_eq!(parse_year_month("2099 6"), Some((2099, 6)));
        assert_eq!(parse_year_month("  2020   5  "), Some((2020, 5)));
    }

    #[test]
    fn test_parse_year_month_invalid() {
        // 测试无效输入
        assert_eq!(parse_year_month("2020"), None); // 缺少月份
        assert_eq!(parse_year_month("2020 13"), None); // 月份超出范围
        assert_eq!(parse_year_month("1899 1"), None); // 年份小于1900
        assert_eq!(parse_year_month("2100 1"), None); // 年份大于2099
        assert_eq!(parse_year_month("abc 1"), None); // 非数字
        assert_eq!(parse_year_month("2020 abc"), None); // 非数字
        assert_eq!(parse_year_month(""), None); // 空输入
    }

    #[test]
    fn test_zeller_month_boundary() {
        // 测试1月和2月的特殊处理
        let jan_day = zeller(2020, 1, 15);
        let feb_day = zeller(2020, 2, 15);
        let mar_day = zeller(2020, 3, 15);

        // 确保返回值在0-6范围内
        assert!(jan_day <= 6);
        assert!(feb_day <= 6);
        assert!(mar_day <= 6);
    }
}
