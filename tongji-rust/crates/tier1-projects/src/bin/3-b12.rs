// 3-b12: Calendar printer - display monthly calendar
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 消除大量重复代码：C++中有24个几乎相同的switch case，Rust用一个函数替代
// 2. 使用const数组存储每月天数，避免硬编码
// 3. 提取闰年判断、天数计算、日历打印为独立函数，便于测试
// 4. 使用迭代器和format!宏简化输出格式化
// 5. 输入验证逻辑更清晰，使用loop + break模式
// 6. 类型安全：使用u32表示年月日，避免负数

use std::io::{self, Write};

/// 判断是否为闰年
///
/// # Arguments
/// * `year` - 年份
///
/// # Returns
/// * `true` - 闰年
/// * `false` - 平年
///
/// # Examples
/// ```
/// assert_eq!(is_leap_year(2000), true);  // 400的倍数
/// assert_eq!(is_leap_year(2004), true);  // 4的倍数但不是100的倍数
/// assert_eq!(is_leap_year(2100), false); // 100的倍数但不是400的倍数
/// assert_eq!(is_leap_year(2001), false); // 普通年份
/// ```
fn is_leap_year(year: u32) -> bool {
    // Rust改进: 使用布尔表达式直接返回，更简洁
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
fn days_in_month(year: u32, month: u32) -> u32 {
    // Rust改进: 使用const数组存储每月天数，避免重复的switch case
    const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    match month {
        2 if is_leap_year(year) => 29,
        _ => DAYS[(month - 1) as usize],
    }
}

/// 打印月历
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份
/// * `start_day` - 该月1日是星期几 (0-6表示星期日-星期六)
fn print_calendar(year: u32, month: u32, start_day: u32) {
    println!();
    println!("{}年{}月的月历为 : ", year, month);
    println!("星期日  星期一  星期二  星期三  星期四  星期五  星期六");

    // Rust改进: 使用迭代器和闭包简化输出逻辑
    let days = days_in_month(year, month);
    let mut current_col = start_day;

    // 打印起始空格
    // Rust改进: 使用repeat()方法生成重复字符串，比循环更简洁
    if start_day > 0 {
        print!("{}", "        ".repeat((start_day - 1) as usize));
        print!("    ");
    }

    // 打印日期
    for day in 1..=days {
        if current_col == 0 {
            print!("{:4}", day);
        } else {
            print!("{:8}", day);
        }

        current_col += 1;

        // 换行
        if current_col > 6 {
            println!();
            current_col = 0;
        }
    }

    // 如果最后一行没有换行，补充换行
    if current_col != 0 {
        println!();
    }
}

/// 读取并验证年份和月份输入
///
/// # Returns
/// * `(year, month)` - 有效的年份和月份
fn read_year_month() -> (u32, u32) {
    loop {
        print!("请输入年份(2000-2030)和月份(1-12) : ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Rust改进: 使用split_whitespace()和collect()解析多个输入
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("输入错误,请重新输入");
            continue;
        }

        // Rust改进: 使用and_then链式调用同时解析两个数字
        let result = parts[0]
            .parse::<u32>()
            .and_then(|y| parts[1].parse::<u32>().map(|m| (y, m)));

        match result {
            Ok((year, month)) if (2000..=2030).contains(&year) && (1..=12).contains(&month) => {
                return (year, month);
            }
            _ => {
                println!("输入错误,请重新输入");
            }
        }
    }
}

/// 读取并验证星期输入
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份
///
/// # Returns
/// * 该月1日的星期 (0-6)
fn read_start_day(year: u32, month: u32) -> u32 {
    loop {
        print!(
            "请输入{}年{}月1日的星期(0-6表示星期日-星期六) : ",
            year, month
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Rust改进: 使用filter()验证范围，代码更简洁
        if let Some(day) = input.trim().parse::<u32>().ok().filter(|&d| d <= 6) {
            return day;
        }

        println!("输入错误,请重新输入");
    }
}

fn main() {
    let (year, month) = read_year_month();
    let start_day = read_start_day(year, month);
    print_calendar(year, month, start_day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year_divisible_by_400() {
        // 测试能被400整除的年份
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
    }

    #[test]
    fn test_is_leap_year_divisible_by_4_not_100() {
        // 测试能被4整除但不能被100整除的年份
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2008));
        assert!(is_leap_year(2012));
        assert!(is_leap_year(2016));
        assert!(is_leap_year(2020));
        assert!(is_leap_year(2024));
    }

    #[test]
    fn test_is_leap_year_divisible_by_100_not_400() {
        // 测试能被100整除但不能被400整除的年份
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
        assert!(!is_leap_year(2200));
        assert!(!is_leap_year(2300));
    }

    #[test]
    fn test_is_leap_year_common_years() {
        // 测试普通年份
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2002));
        assert!(!is_leap_year(2003));
        assert!(!is_leap_year(2005));
        assert!(!is_leap_year(2019));
    }

    #[test]
    fn test_days_in_month_31_days() {
        // 测试31天的月份
        assert_eq!(days_in_month(2020, 1), 31); // 1月
        assert_eq!(days_in_month(2020, 3), 31); // 3月
        assert_eq!(days_in_month(2020, 5), 31); // 5月
        assert_eq!(days_in_month(2020, 7), 31); // 7月
        assert_eq!(days_in_month(2020, 8), 31); // 8月
        assert_eq!(days_in_month(2020, 10), 31); // 10月
        assert_eq!(days_in_month(2020, 12), 31); // 12月
    }

    #[test]
    fn test_days_in_month_30_days() {
        // 测试30天的月份
        assert_eq!(days_in_month(2020, 4), 30); // 4月
        assert_eq!(days_in_month(2020, 6), 30); // 6月
        assert_eq!(days_in_month(2020, 9), 30); // 9月
        assert_eq!(days_in_month(2020, 11), 30); // 11月
    }

    #[test]
    fn test_days_in_month_february_leap_year() {
        // 测试闰年2月
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(days_in_month(2004, 2), 29);
        assert_eq!(days_in_month(2020, 2), 29);
        assert_eq!(days_in_month(2024, 2), 29);
    }

    #[test]
    fn test_days_in_month_february_common_year() {
        // 测试平年2月
        assert_eq!(days_in_month(1900, 2), 28);
        assert_eq!(days_in_month(2001, 2), 28);
        assert_eq!(days_in_month(2019, 2), 28);
        assert_eq!(days_in_month(2100, 2), 28);
    }

    #[test]
    fn test_days_in_month_all_months_leap_year() {
        // 测试闰年所有月份
        let expected = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for month in 1..=12 {
            assert_eq!(days_in_month(2020, month), expected[(month - 1) as usize]);
        }
    }

    #[test]
    fn test_days_in_month_all_months_common_year() {
        // 测试平年所有月份
        let expected = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for month in 1..=12 {
            assert_eq!(days_in_month(2019, month), expected[(month - 1) as usize]);
        }
    }

    #[test]
    fn test_days_in_month_boundary_years() {
        // 测试边界年份
        assert_eq!(days_in_month(2000, 2), 29); // 边界闰年
        assert_eq!(days_in_month(2030, 2), 28); // 边界平年
    }
}
