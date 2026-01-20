// 3-b6: Calculate day of year from date
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用const数组存储每月天数，避免重复的switch-case代码
// 2. 提取闰年判断、日期验证、天数计算为独立函数，便于测试和复用
// 3. 使用Result类型进行错误处理，而不是在每个分支打印错误
// 4. 使用迭代器的sum()方法计算累计天数，更简洁且零成本抽象
// 5. 类型安全：使用u32表示年月日，避免负数输入

use std::io::{self, Write};

/// 每月天数（非闰年）
const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
/// assert_eq!(is_leap_year(1900), false); // 100的倍数但不是400的倍数
/// assert_eq!(is_leap_year(2001), false); // 不是4的倍数
/// ```
fn is_leap_year(year: u32) -> bool {
    // Rust改进: 使用布尔表达式直接返回，更简洁
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// 获取指定月份的天数
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份 (1-12)
///
/// # Returns
/// * `Some(u32)` - 该月的天数
/// * `None` - 月份无效
fn days_in_month(year: u32, month: u32) -> Option<u32> {
    // Rust改进: 使用match表达式和数组索引，避免重复代码
    match month {
        1..=12 => {
            let mut days = DAYS_IN_MONTH[(month - 1) as usize];
            // 闰年2月有29天
            if month == 2 && is_leap_year(year) {
                days += 1;
            }
            Some(days)
        }
        _ => None,
    }
}

/// 验证日期是否合法
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份
/// * `day` - 日期
///
/// # Returns
/// * `Ok(())` - 日期合法
/// * `Err(&str)` - 日期非法，包含错误信息
fn validate_date(year: u32, month: u32, day: u32) -> Result<(), &'static str> {
    // 验证月份
    if !(1..=12).contains(&month) {
        return Err("月份不正确");
    }

    // 验证日期
    let max_days = days_in_month(year, month).unwrap(); // month已验证，unwrap安全
    if day < 1 || day > max_days {
        return Err("输入错误-日与月的关系非法");
    }

    Ok(())
}

/// 计算给定日期是该年的第几天
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份 (1-12)
/// * `day` - 日期
///
/// # Returns
/// * `Ok(u32)` - 该年的第几天
/// * `Err(&str)` - 日期非法
///
/// # Examples
/// ```
/// assert_eq!(day_of_year(2020, 1, 1), Ok(1));
/// assert_eq!(day_of_year(2020, 2, 29), Ok(60));  // 闰年
/// assert_eq!(day_of_year(2021, 2, 29), Err("输入错误-日与月的关系非法")); // 平年
/// assert_eq!(day_of_year(2020, 12, 31), Ok(366)); // 闰年最后一天
/// ```
fn day_of_year(year: u32, month: u32, day: u32) -> Result<u32, &'static str> {
    // 先验证日期
    validate_date(year, month, day)?;

    // Rust改进: 使用迭代器和sum()计算前几个月的天数总和
    // 这是零成本抽象，编译后性能与手写循环相同
    let days_before_month: u32 = (1..month)
        .map(|m| days_in_month(year, m).unwrap()) // month已验证，unwrap安全
        .sum();

    Ok(days_before_month + day)
}

fn main() {
    println!("请输入年,月,日 : ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Rust改进: 使用迭代器和collect解析多个数字
    let numbers: Vec<u32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // 检查是否输入了3个数字
    if numbers.len() != 3 {
        println!("输入格式错误，请输入三个数字");
        return;
    }

    let (year, month, day) = (numbers[0], numbers[1], numbers[2]);

    // Rust改进: 使用match处理Result，优雅地处理成功和错误情况
    match day_of_year(year, month, day) {
        Ok(order) => {
            println!("{}-{}-{}是{}年的第{}天", year, month, day, year, order);
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        // 400的倍数是闰年
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));

        // 4的倍数但不是100的倍数是闰年
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2020));
        assert!(is_leap_year(2024));

        // 100的倍数但不是400的倍数不是闰年
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));

        // 不是4的倍数不是闰年
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2019));
    }

    #[test]
    fn test_days_in_month_normal_year() {
        let year = 2019; // 平年
        assert_eq!(days_in_month(year, 1), Some(31));
        assert_eq!(days_in_month(year, 2), Some(28)); // 平年2月28天
        assert_eq!(days_in_month(year, 3), Some(31));
        assert_eq!(days_in_month(year, 4), Some(30));
        assert_eq!(days_in_month(year, 5), Some(31));
        assert_eq!(days_in_month(year, 6), Some(30));
        assert_eq!(days_in_month(year, 7), Some(31));
        assert_eq!(days_in_month(year, 8), Some(31));
        assert_eq!(days_in_month(year, 9), Some(30));
        assert_eq!(days_in_month(year, 10), Some(31));
        assert_eq!(days_in_month(year, 11), Some(30));
        assert_eq!(days_in_month(year, 12), Some(31));
    }

    #[test]
    fn test_days_in_month_leap_year() {
        let year = 2020; // 闰年
        assert_eq!(days_in_month(year, 2), Some(29)); // 闰年2月29天
        assert_eq!(days_in_month(year, 1), Some(31)); // 其他月份不变
    }

    #[test]
    fn test_days_in_month_invalid() {
        assert_eq!(days_in_month(2020, 0), None);
        assert_eq!(days_in_month(2020, 13), None);
        assert_eq!(days_in_month(2020, 100), None);
    }

    #[test]
    fn test_validate_date_valid() {
        assert!(validate_date(2020, 1, 1).is_ok());
        assert!(validate_date(2020, 12, 31).is_ok());
        assert!(validate_date(2020, 2, 29).is_ok()); // 闰年2月29日
        assert!(validate_date(2020, 6, 15).is_ok());
    }

    #[test]
    fn test_validate_date_invalid_month() {
        assert_eq!(validate_date(2020, 0, 1), Err("月份不正确"));
        assert_eq!(validate_date(2020, 13, 1), Err("月份不正确"));
    }

    #[test]
    fn test_validate_date_invalid_day() {
        assert_eq!(
            validate_date(2020, 1, 0),
            Err("输入错误-日与月的关系非法")
        );
        assert_eq!(
            validate_date(2020, 1, 32),
            Err("输入错误-日与月的关系非法")
        );
        assert_eq!(
            validate_date(2019, 2, 29),
            Err("输入错误-日与月的关系非法")
        ); // 平年2月没有29日
        assert_eq!(
            validate_date(2020, 4, 31),
            Err("输入错误-日与月的关系非法")
        ); // 4月只有30天
    }

    #[test]
    fn test_day_of_year_first_day() {
        assert_eq!(day_of_year(2020, 1, 1), Ok(1));
        assert_eq!(day_of_year(2019, 1, 1), Ok(1));
    }

    #[test]
    fn test_day_of_year_last_day() {
        assert_eq!(day_of_year(2020, 12, 31), Ok(366)); // 闰年
        assert_eq!(day_of_year(2019, 12, 31), Ok(365)); // 平年
    }

    #[test]
    fn test_day_of_year_leap_year() {
        assert_eq!(day_of_year(2020, 2, 29), Ok(60)); // 闰年2月29日
        assert_eq!(day_of_year(2020, 3, 1), Ok(61)); // 闰年3月1日
    }

    #[test]
    fn test_day_of_year_normal_year() {
        assert_eq!(day_of_year(2019, 2, 28), Ok(59)); // 平年2月28日
        assert_eq!(day_of_year(2019, 3, 1), Ok(60)); // 平年3月1日
    }

    #[test]
    fn test_day_of_year_various_dates() {
        // 测试各种日期
        assert_eq!(day_of_year(2020, 1, 31), Ok(31));
        assert_eq!(day_of_year(2020, 2, 1), Ok(32));
        assert_eq!(day_of_year(2020, 6, 15), Ok(167));
        assert_eq!(day_of_year(2019, 6, 15), Ok(166)); // 平年少一天
    }

    #[test]
    fn test_day_of_year_invalid() {
        // 测试无效日期
        assert!(day_of_year(2020, 0, 1).is_err());
        assert!(day_of_year(2020, 13, 1).is_err());
        assert!(day_of_year(2020, 1, 0).is_err());
        assert!(day_of_year(2020, 1, 32).is_err());
        assert!(day_of_year(2019, 2, 29).is_err()); // 平年没有2月29日
    }

    #[test]
    fn test_day_of_year_edge_cases() {
        // 边界情况测试
        assert_eq!(day_of_year(2000, 2, 29), Ok(60)); // 2000年是闰年
        assert!(day_of_year(1900, 2, 29).is_err()); // 1900年不是闰年
        assert_eq!(day_of_year(2000, 12, 31), Ok(366));
        assert_eq!(day_of_year(1900, 12, 31), Ok(365));
    }
}
