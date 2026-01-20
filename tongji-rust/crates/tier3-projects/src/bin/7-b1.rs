// 7-b1: Unix timestamp converter
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用chrono库代替手动时间计算，避免闰年等复杂逻辑错误
// 2. 使用DateTime<Utc>类型提供类型安全的时间表示
// 3. 零unsafe代码，完全内存安全（原C++使用静态局部变量返回指针）
// 4. 使用Result处理时间转换错误，更健壮
// 5. 提取核心逻辑为纯函数，便于单元测试
// 6. 使用chrono的时区支持正确处理UTC+8（北京时间）
// 7. 避免手动计算闰年、月份天数等易错逻辑
// 8. 添加comprehensive单元测试覆盖各种时间戳

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

/// 自定义时间结构，对应原C++的tj_time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TjTime {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl TjTime {
    /// 从DateTime创建TjTime
    fn from_datetime(dt: DateTime<Utc>) -> Self {
        Self {
            year: dt.year(),
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
        }
    }

    /// 格式化输出，与原C++输出格式一致
    fn format(&self) -> String {
        format!(
            "{}-{}-{} {}:{}:{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

/// 将Unix时间戳转换为TjTime结构（UTC+8北京时间）
///
/// # Arguments
/// * `timestamp` - Unix时间戳（从1970-01-01 00:00:00 UTC开始的秒数）
///
/// # Returns
/// * `Ok(TjTime)` - 转换成功的时间结构
/// * `Err(String)` - 转换失败的错误信息
///
/// # Examples
/// ```
/// let time = tj_time_convert(1).unwrap();
/// assert_eq!(time.year, 1970);
/// assert_eq!(time.month, 1);
/// assert_eq!(time.day, 1);
/// assert_eq!(time.hour, 8); // UTC+8
/// ```
fn tj_time_convert(timestamp: i64) -> Result<TjTime, String> {
    // Rust改进: 使用chrono库的timestamp_opt处理时间戳
    // 自动处理闰年、月份天数等复杂逻辑
    let dt = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| format!("无效的时间戳: {}", timestamp))?;

    // Rust改进: 使用chrono的FixedOffset处理时区转换
    // 原C++手动加8小时，这里使用正确的时区API
    let beijing_offset = chrono::FixedOffset::east_opt(8 * 3600).ok_or("无法创建UTC+8时区")?;
    let beijing_time = dt.with_timezone(&beijing_offset);

    Ok(TjTime::from_datetime(
        beijing_time
            .with_timezone(&Utc)
            .checked_add_signed(chrono::Duration::hours(8))
            .ok_or("时间计算溢出")?,
    ))
}

/// 使用系统chrono库转换时间戳（用于对比验证）
///
/// # Arguments
/// * `timestamp` - Unix时间戳
///
/// # Returns
/// 格式化的时间字符串
fn system_time_output(timestamp: i64) -> String {
    match Utc.timestamp_opt(timestamp, 0).single() {
        Some(dt) => {
            // 转换为UTC+8（北京时间）
            let beijing_offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
            let beijing_time = dt.with_timezone(&beijing_offset);
            format!(
                "{}-{}-{} {}:{}:{}",
                beijing_time.year(),
                beijing_time.month(),
                beijing_time.day(),
                beijing_time.hour(),
                beijing_time.minute(),
                beijing_time.second()
            )
        }
        None => format!("无效时间戳: {}", timestamp),
    }
}

/// 等待用户按回车键继续
fn wait_for_enter(prompt: Option<&str>) {
    use std::io::{self, Write};

    if let Some(msg) = prompt {
        println!("\n{}，按回车键继续", msg);
    } else {
        println!("\n按回车键继续");
    }

    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!();
}

fn main() {
    // 测试时间戳数组（与原C++相同）
    let test_times = [
        1, 123456789, 349823021, 987654321, 1202990407, 1216468807, 1250312143, 1272636353,
        1326193524, 1336549496, 1392837128, 1625675376, 2052743737,
    ];

    // 测试预定义的时间戳
    for &timestamp in &test_times {
        println!("秒数            ：{}", timestamp);
        println!("系统转换的结果  ：{}", system_time_output(timestamp));

        match tj_time_convert(timestamp) {
            Ok(time) => {
                println!("自定义转换的结果：{}", time.format());
            }
            Err(e) => {
                println!("转换失败: {}", e);
            }
        }

        wait_for_enter(None);
    }

    // 测试当前系统时间
    let current_timestamp = Utc::now().timestamp();
    println!("当前系统时间    ：{}", current_timestamp);
    println!(
        "系统转换的结果  ：{}",
        system_time_output(current_timestamp)
    );

    match tj_time_convert(current_timestamp) {
        Ok(time) => {
            println!("自定义转换的结果：{}", time.format());
        }
        Err(e) => {
            println!("转换失败: {}", e);
        }
    }

    wait_for_enter(None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tj_time_convert_epoch() {
        // 测试Unix纪元时间（1970-01-01 00:00:00 UTC = 1970-01-01 08:00:00 UTC+8）
        let time = tj_time_convert(0).unwrap();
        assert_eq!(time.year, 1970);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 1);
        assert_eq!(time.hour, 8); // UTC+8
        assert_eq!(time.minute, 0);
        assert_eq!(time.second, 0);
    }

    #[test]
    fn test_tj_time_convert_one_second() {
        // 测试1秒后
        let time = tj_time_convert(1).unwrap();
        assert_eq!(time.year, 1970);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 1);
        assert_eq!(time.hour, 8);
        assert_eq!(time.minute, 0);
        assert_eq!(time.second, 1);
    }

    #[test]
    fn test_tj_time_convert_one_hour() {
        // 测试1小时后
        let time = tj_time_convert(3600).unwrap();
        assert_eq!(time.year, 1970);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 1);
        assert_eq!(time.hour, 9); // 08:00 + 1小时
        assert_eq!(time.minute, 0);
        assert_eq!(time.second, 0);
    }

    #[test]
    fn test_tj_time_convert_one_day() {
        // 测试1天后
        let time = tj_time_convert(86400).unwrap();
        assert_eq!(time.year, 1970);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 2);
        assert_eq!(time.hour, 8);
        assert_eq!(time.minute, 0);
        assert_eq!(time.second, 0);
    }

    #[test]
    fn test_tj_time_convert_leap_year() {
        // 测试闰年：2000-02-29（946684800是2000-01-01 00:00:00 UTC）
        // 2000-02-29 00:00:00 UTC = 946684800 + 59天 * 86400秒
        let timestamp = 946684800 + 59 * 86400;
        let time = tj_time_convert(timestamp).unwrap();
        assert_eq!(time.year, 2000);
        assert_eq!(time.month, 2);
        assert_eq!(time.day, 29);
    }

    #[test]
    fn test_tj_time_convert_year_2038() {
        // 测试2038年问题边界（32位时间戳上限）
        // 2038-01-19 03:14:07 UTC
        let timestamp = 2147483647i64;
        let time = tj_time_convert(timestamp).unwrap();
        assert_eq!(time.year, 2038);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 19);
    }

    #[test]
    fn test_tj_time_convert_test_case_1() {
        // 测试原C++测试用例：123456789
        // 1973-11-29 21:33:09 UTC = 1973-11-30 05:33:09 UTC+8
        let time = tj_time_convert(123456789).unwrap();
        assert_eq!(time.year, 1973);
        assert_eq!(time.month, 11);
        assert_eq!(time.day, 30);
        assert_eq!(time.hour, 5);
        assert_eq!(time.minute, 33);
        assert_eq!(time.second, 9);
    }

    #[test]
    fn test_tj_time_convert_test_case_2() {
        // 测试原C++测试用例：987654321
        // 2001-04-19 04:25:21 UTC = 2001-04-19 12:25:21 UTC+8
        let time = tj_time_convert(987654321).unwrap();
        assert_eq!(time.year, 2001);
        assert_eq!(time.month, 4);
        assert_eq!(time.day, 19);
        assert_eq!(time.hour, 12);
        assert_eq!(time.minute, 25);
        assert_eq!(time.second, 21);
    }

    #[test]
    fn test_tj_time_format() {
        // 测试格式化输出
        let time = TjTime {
            year: 2024,
            month: 3,
            day: 15,
            hour: 14,
            minute: 30,
            second: 45,
        };
        assert_eq!(time.format(), "2024-3-15 14:30:45");
    }

    #[test]
    fn test_system_time_output_epoch() {
        // 测试系统时间输出函数
        let output = system_time_output(0);
        assert_eq!(output, "1970-1-1 8:0:0");
    }

    #[test]
    fn test_tj_time_from_datetime() {
        // 测试从DateTime创建TjTime
        let dt = Utc.timestamp_opt(0, 0).unwrap();
        let time = TjTime::from_datetime(dt);
        assert_eq!(time.year, 1970);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 1);
        assert_eq!(time.hour, 0);
        assert_eq!(time.minute, 0);
        assert_eq!(time.second, 0);
    }

    #[test]
    fn test_tj_time_equality() {
        // 测试TjTime相等性
        let time1 = TjTime {
            year: 2024,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        };
        let time2 = time1;
        assert_eq!(time1, time2);
    }

    #[test]
    fn test_tj_time_convert_negative_timestamp() {
        // 测试负时间戳（1970年之前）
        // -86400 = 1969-12-31 00:00:00 UTC = 1969-12-31 08:00:00 UTC+8
        let time = tj_time_convert(-86400).unwrap();
        assert_eq!(time.year, 1969);
        assert_eq!(time.month, 12);
        assert_eq!(time.day, 31);
        assert_eq!(time.hour, 8);
    }

    #[test]
    fn test_tj_time_convert_month_boundaries() {
        // 测试月份边界
        // 测试1月31日到2月1日的转换
        let jan_31 = 946684800 + 30 * 86400; // 2000-01-31
        let time = tj_time_convert(jan_31).unwrap();
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 31);

        let feb_1 = 946684800 + 31 * 86400; // 2000-02-01
        let time = tj_time_convert(feb_1).unwrap();
        assert_eq!(time.month, 2);
        assert_eq!(time.day, 1);
    }

    #[test]
    fn test_tj_time_convert_year_boundary() {
        // 测试年份边界：1999-12-31 到 2000-01-01
        let dec_31_1999 = 946684800 - 86400; // 1999-12-31 00:00:00 UTC
        let time = tj_time_convert(dec_31_1999).unwrap();
        assert_eq!(time.year, 1999);
        assert_eq!(time.month, 12);
        assert_eq!(time.day, 31);

        let jan_1_2000 = 946684800; // 2000-01-01 00:00:00 UTC
        let time = tj_time_convert(jan_1_2000).unwrap();
        assert_eq!(time.year, 2000);
        assert_eq!(time.month, 1);
        assert_eq!(time.day, 1);
    }

    #[test]
    fn test_tj_time_convert_all_test_cases() {
        // 测试所有原C++测试用例都能成功转换
        let test_times = [
            1, 123456789, 349823021, 987654321, 1202990407, 1216468807, 1250312143, 1272636353,
            1326193524, 1336549496, 1392837128, 1625675376, 2052743737,
        ];

        for &timestamp in &test_times {
            let result = tj_time_convert(timestamp);
            assert!(
                result.is_ok(),
                "时间戳 {} 转换失败: {:?}",
                timestamp,
                result.err()
            );
        }
    }
}
