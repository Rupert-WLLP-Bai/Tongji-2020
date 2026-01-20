// 4-b2: Calculate day of week using Zeller's congruence
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用枚举类型表示星期，类型安全且语义清晰
// 2. 使用Result类型处理日期验证，错误信息更明确
// 3. 提取日期验证逻辑为独立函数，便于测试
// 4. 使用const fn计算闰年，编译期优化
// 5. 避免while循环处理负数，使用rem_euclid确保正余数
// 6. 使用match表达式替代switch，更符合Rust习惯

use std::io::{self, Write};

/// 星期枚举
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekday {
    /// 从数字转换为星期（0=周日, 1=周一, ..., 6=周六）
    fn from_number(n: i32) -> Self {
        match n {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            6 => Weekday::Saturday,
            _ => unreachable!("Invalid weekday number: {}", n),
        }
    }

    /// 转换为中文字符串
    pub fn to_chinese(&self) -> &'static str {
        match self {
            Weekday::Sunday => "星期日",
            Weekday::Monday => "星期一",
            Weekday::Tuesday => "星期二",
            Weekday::Wednesday => "星期三",
            Weekday::Thursday => "星期四",
            Weekday::Friday => "星期五",
            Weekday::Saturday => "星期六",
        }
    }
}

/// 判断是否为闰年
///
/// # Arguments
/// * `year` - 年份
///
/// # Returns
/// * `true` - 闰年
/// * `false` - 平年
pub const fn is_leap_year(year: i32) -> bool {
    // Rust改进: 使用const fn，可在编译期计算
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// 获取指定月份的天数
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份（1-12）
///
/// # Returns
/// * 该月的天数
fn days_in_month(year: i32, month: i32) -> i32 {
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
        _ => 0, // 无效月份
    }
}

/// 验证日期是否合法
///
/// # Arguments
/// * `year` - 年份（1900-2099）
/// * `month` - 月份（1-12）
/// * `day` - 日期（1-31）
///
/// # Returns
/// * `Ok(())` - 日期合法
/// * `Err(String)` - 日期不合法，包含错误信息
pub fn validate_date(year: i32, month: i32, day: i32) -> Result<(), String> {
    // Rust改进: 使用Result类型，错误信息更明确
    if !(1900..=2099).contains(&year) {
        return Err(format!("年份必须在1900-2099之间，当前输入: {}", year));
    }

    if !(1..=12).contains(&month) {
        return Err(format!("月份必须在1-12之间，当前输入: {}", month));
    }

    let max_day = days_in_month(year, month);
    if day < 1 || day > max_day {
        return Err(format!(
            "{}年{}月的日期必须在1-{}之间，当前输入: {}",
            year, month, max_day, day
        ));
    }

    Ok(())
}

/// 使用蔡勒公式计算星期
///
/// Zeller's congruence:
/// w = (y + y/4 + c/4 - 2c + 26(m+1)/10 + d - 1) mod 7
///
/// 其中：
/// - y: 年份的后两位
/// - c: 世纪（年份的前两位）
/// - m: 月份（3-14，1月和2月看作上一年的13月和14月）
/// - d: 日期
/// - w: 星期（0=周日, 1=周一, ..., 6=周六）
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份（1-12）
/// * `day` - 日期
///
/// # Returns
/// * 星期枚举
pub fn zeller(year: i32, month: i32, day: i32) -> Weekday {
    // Rust改进: 使用元组解构，代码更简洁
    let (y, c, m) = if month >= 3 {
        // 3月到12月
        (year % 100, year / 100, month)
    } else {
        // 1月和2月看作上一年的13月和14月
        ((year - 1) % 100, (year - 1) / 100, month + 12)
    };

    // 蔡勒公式
    let w = y + y / 4 + c / 4 - 2 * c + (13 * (m + 1) / 5) + day - 1;

    // Rust改进: 使用rem_euclid确保结果为正数，避免while循环
    let weekday_num = w.rem_euclid(7);

    Weekday::from_number(weekday_num)
}

/// 读取并验证日期输入
///
/// # Returns
/// * `Some((year, month, day))` - 合法的日期
/// * `None` - 用户输入错误
fn read_date() -> Option<(i32, i32, i32)> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    // Rust改进: 使用迭代器和collect，代码更简洁
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    // Rust改进: 使用?操作符简化错误处理
    let year = parts[0].parse::<i32>().ok()?;
    let month = parts[1].parse::<i32>().ok()?;
    let day = parts[2].parse::<i32>().ok()?;

    Some((year, month, day))
}

fn main() {
    // Rust改进: 使用loop + break返回值，避免未初始化变量
    let (year, month, day) = loop {
        print!("输入年月日,范围是(1900.1.1-2099.12.31) : ");
        io::stdout().flush().unwrap();

        match read_date() {
            Some((y, m, d)) => {
                // 验证日期
                if let Err(err) = validate_date(y, m, d) {
                    println!("输入错误: {}", err);
                    continue;
                }
                break (y, m, d);
            }
            None => {
                println!("输入错误,请重新输入");
            }
        }
    };

    // 计算星期
    let weekday = zeller(year, month, day);
    println!("{}", weekday.to_chinese());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        // 测试闰年判断
        assert!(is_leap_year(2000)); // 400的倍数
        assert!(is_leap_year(2004)); // 4的倍数但不是100的倍数
        assert!(!is_leap_year(1900)); // 100的倍数但不是400的倍数
        assert!(!is_leap_year(2001)); // 普通年份
        assert!(is_leap_year(2020));
        assert!(!is_leap_year(2021));
    }

    #[test]
    fn test_days_in_month() {
        // 测试每月天数
        assert_eq!(days_in_month(2020, 1), 31);
        assert_eq!(days_in_month(2020, 2), 29); // 闰年2月
        assert_eq!(days_in_month(2021, 2), 28); // 平年2月
        assert_eq!(days_in_month(2020, 4), 30);
        assert_eq!(days_in_month(2020, 12), 31);
    }

    #[test]
    fn test_validate_date_valid() {
        // 测试合法日期
        assert!(validate_date(2020, 1, 1).is_ok());
        assert!(validate_date(2020, 2, 29).is_ok()); // 闰年2月29日
        assert!(validate_date(1900, 1, 1).is_ok());
        assert!(validate_date(2099, 12, 31).is_ok());
    }

    #[test]
    fn test_validate_date_invalid_year() {
        // 测试无效年份
        assert!(validate_date(1899, 1, 1).is_err());
        assert!(validate_date(2100, 1, 1).is_err());
    }

    #[test]
    fn test_validate_date_invalid_month() {
        // 测试无效月份
        assert!(validate_date(2020, 0, 1).is_err());
        assert!(validate_date(2020, 13, 1).is_err());
    }

    #[test]
    fn test_validate_date_invalid_day() {
        // 测试无效日期
        assert!(validate_date(2020, 1, 0).is_err());
        assert!(validate_date(2020, 1, 32).is_err());
        assert!(validate_date(2020, 2, 30).is_err());
        assert!(validate_date(2021, 2, 29).is_err()); // 平年没有2月29日
        assert!(validate_date(2020, 4, 31).is_err());
    }

    #[test]
    fn test_zeller_known_dates() {
        // 测试已知日期的星期
        // 2000年1月1日是星期六
        assert_eq!(zeller(2000, 1, 1), Weekday::Saturday);

        // 2020年1月1日是星期三
        assert_eq!(zeller(2020, 1, 1), Weekday::Wednesday);

        // 2021年1月1日是星期五
        assert_eq!(zeller(2021, 1, 1), Weekday::Friday);

        // 1900年1月1日是星期一
        assert_eq!(zeller(1900, 1, 1), Weekday::Monday);

        // 2099年12月31日是星期四
        assert_eq!(zeller(2099, 12, 31), Weekday::Thursday);
    }

    #[test]
    fn test_zeller_leap_year_feb29() {
        // 测试闰年2月29日
        // 2020年2月29日是星期六
        assert_eq!(zeller(2020, 2, 29), Weekday::Saturday);

        // 2000年2月29日是星期二
        assert_eq!(zeller(2000, 2, 29), Weekday::Tuesday);
    }

    #[test]
    fn test_zeller_all_weekdays() {
        // 测试所有星期（2020年3月1-7日）
        assert_eq!(zeller(2020, 3, 1), Weekday::Sunday);
        assert_eq!(zeller(2020, 3, 2), Weekday::Monday);
        assert_eq!(zeller(2020, 3, 3), Weekday::Tuesday);
        assert_eq!(zeller(2020, 3, 4), Weekday::Wednesday);
        assert_eq!(zeller(2020, 3, 5), Weekday::Thursday);
        assert_eq!(zeller(2020, 3, 6), Weekday::Friday);
        assert_eq!(zeller(2020, 3, 7), Weekday::Saturday);
    }

    #[test]
    fn test_weekday_to_chinese() {
        // 测试中文转换
        assert_eq!(Weekday::Sunday.to_chinese(), "星期日");
        assert_eq!(Weekday::Monday.to_chinese(), "星期一");
        assert_eq!(Weekday::Tuesday.to_chinese(), "星期二");
        assert_eq!(Weekday::Wednesday.to_chinese(), "星期三");
        assert_eq!(Weekday::Thursday.to_chinese(), "星期四");
        assert_eq!(Weekday::Friday.to_chinese(), "星期五");
        assert_eq!(Weekday::Saturday.to_chinese(), "星期六");
    }

    #[test]
    fn test_zeller_january_february() {
        // 测试1月和2月（作为上一年的13月和14月处理）
        // 2020年1月15日是星期三
        assert_eq!(zeller(2020, 1, 15), Weekday::Wednesday);

        // 2020年2月15日是星期六
        assert_eq!(zeller(2020, 2, 15), Weekday::Saturday);
    }
}
