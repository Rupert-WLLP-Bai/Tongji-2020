// 5-b3: Date validation and day-of-year calculation
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Result<T, E>类型进行错误处理，避免多层嵌套if-else
// 2. 使用const数组和match表达式，代码更简洁清晰
// 3. 提取验证和计算逻辑为独立函数，便于单元测试
// 4. 使用迭代器的sum()方法，避免手动循环累加
// 5. 使用自定义错误类型，提供更好的错误信息

use std::io;

/// 日期验证错误类型
#[derive(Debug, PartialEq)]
enum DateError {
    YearOutOfRange,
    MonthInvalid,
    DayInvalid,
}

impl std::fmt::Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateError::YearOutOfRange => write!(f, "输入错误-年份输入错误"),
            DateError::MonthInvalid => write!(f, "月份不正确"),
            DateError::DayInvalid => write!(f, "输入错误-日与月的关系非法"),
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
///
/// # Examples
/// ```
/// assert_eq!(is_leap_year(2000), true);  // 400的倍数
/// assert_eq!(is_leap_year(2004), true);  // 4的倍数但不是100的倍数
/// assert_eq!(is_leap_year(2100), false); // 100的倍数但不是400的倍数
/// assert_eq!(is_leap_year(2001), false); // 普通年份
/// ```
fn is_leap_year(year: i32) -> bool {
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
/// * 该月的天数
fn days_in_month(year: i32, month: u32) -> u32 {
    // Rust改进: 使用const数组和match表达式，比C++的if-else链更清晰
    const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    match month {
        2 if is_leap_year(year) => 29,
        _ => DAYS[(month - 1) as usize],
    }
}

/// 验证日期并计算是一年中的第几天
///
/// # Arguments
/// * `year` - 年份 (2000-2030)
/// * `month` - 月份 (1-12)
/// * `day` - 日期 (1-31)
///
/// # Returns
/// * `Ok(u32)` - 一年中的第几天
/// * `Err(DateError)` - 日期验证错误
fn validate_and_calculate_day_of_year(year: i32, month: u32, day: u32) -> Result<u32, DateError> {
    // 验证年份
    if !(2000..=2030).contains(&year) {
        return Err(DateError::YearOutOfRange);
    }

    // 验证月份
    if !(1..=12).contains(&month) {
        return Err(DateError::MonthInvalid);
    }

    // 验证日期
    let max_day = days_in_month(year, month);
    if !(1..=max_day).contains(&day) {
        return Err(DateError::DayInvalid);
    }

    // Rust改进: 使用迭代器的sum()方法计算前几个月的天数，避免手动循环
    let days_before_month: u32 = (1..month).map(|m| days_in_month(year, m)).sum();

    Ok(days_before_month + day)
}

fn main() {
    println!("请输入年,月,日 : ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // 解析输入
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        println!("输入格式错误，请输入三个数字");
        return;
    }

    let year = parts[0].parse::<i32>().unwrap_or(0);
    let month = parts[1].parse::<u32>().unwrap_or(0);
    let day = parts[2].parse::<u32>().unwrap_or(0);

    // Rust改进: 使用match处理Result，代码结构清晰
    match validate_and_calculate_day_of_year(year, month, day) {
        Ok(day_of_year) => {
            println!(
                "{}-{}-{}是{}年的第{}天",
                year, month, day, year, day_of_year
            );
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        // 测试闰年判断
        assert!(is_leap_year(2000)); // 400的倍数
        assert!(is_leap_year(2004)); // 4的倍数但不是100的倍数
        assert!(is_leap_year(2020));
        assert!(!is_leap_year(2100)); // 100的倍数但不是400的倍数
        assert!(!is_leap_year(2001)); // 普通年份
        assert!(!is_leap_year(2019));
    }

    #[test]
    fn test_days_in_month() {
        // 测试平年
        assert_eq!(days_in_month(2019, 1), 31);
        assert_eq!(days_in_month(2019, 2), 28);
        assert_eq!(days_in_month(2019, 4), 30);
        assert_eq!(days_in_month(2019, 12), 31);

        // 测试闰年
        assert_eq!(days_in_month(2020, 2), 29);
        assert_eq!(days_in_month(2000, 2), 29);
    }

    #[test]
    fn test_valid_dates() {
        // 测试有效日期
        assert_eq!(validate_and_calculate_day_of_year(2020, 1, 1), Ok(1));
        assert_eq!(validate_and_calculate_day_of_year(2020, 1, 31), Ok(31));
        assert_eq!(validate_and_calculate_day_of_year(2020, 2, 1), Ok(32));
        assert_eq!(validate_and_calculate_day_of_year(2020, 12, 31), Ok(366)); // 闰年
        assert_eq!(validate_and_calculate_day_of_year(2019, 12, 31), Ok(365)); // 平年
    }

    #[test]
    fn test_leap_year_february() {
        // 测试闰年2月
        assert_eq!(validate_and_calculate_day_of_year(2020, 2, 29), Ok(60));
        assert_eq!(validate_and_calculate_day_of_year(2020, 3, 1), Ok(61));

        // 测试平年2月29日无效
        assert_eq!(
            validate_and_calculate_day_of_year(2019, 2, 29),
            Err(DateError::DayInvalid)
        );
    }

    #[test]
    fn test_invalid_year() {
        // 测试年份超出范围
        assert_eq!(
            validate_and_calculate_day_of_year(1999, 1, 1),
            Err(DateError::YearOutOfRange)
        );
        assert_eq!(
            validate_and_calculate_day_of_year(2031, 1, 1),
            Err(DateError::YearOutOfRange)
        );
    }

    #[test]
    fn test_invalid_month() {
        // 测试月份无效
        assert_eq!(
            validate_and_calculate_day_of_year(2020, 0, 1),
            Err(DateError::MonthInvalid)
        );
        assert_eq!(
            validate_and_calculate_day_of_year(2020, 13, 1),
            Err(DateError::MonthInvalid)
        );
    }

    #[test]
    fn test_invalid_day() {
        // 测试日期无效
        assert_eq!(
            validate_and_calculate_day_of_year(2020, 1, 0),
            Err(DateError::DayInvalid)
        );
        assert_eq!(
            validate_and_calculate_day_of_year(2020, 1, 32),
            Err(DateError::DayInvalid)
        );
        assert_eq!(
            validate_and_calculate_day_of_year(2020, 4, 31),
            Err(DateError::DayInvalid)
        );
        assert_eq!(
            validate_and_calculate_day_of_year(2019, 2, 29),
            Err(DateError::DayInvalid)
        );
    }

    #[test]
    fn test_boundary_years() {
        // 测试边界年份
        assert_eq!(validate_and_calculate_day_of_year(2000, 1, 1), Ok(1));
        assert_eq!(validate_and_calculate_day_of_year(2030, 12, 31), Ok(365));
    }

    #[test]
    fn test_various_dates() {
        // 测试各种日期
        assert_eq!(validate_and_calculate_day_of_year(2020, 3, 15), Ok(75));
        assert_eq!(validate_and_calculate_day_of_year(2020, 6, 1), Ok(153));
        assert_eq!(validate_and_calculate_day_of_year(2020, 9, 30), Ok(274));
    }
}
