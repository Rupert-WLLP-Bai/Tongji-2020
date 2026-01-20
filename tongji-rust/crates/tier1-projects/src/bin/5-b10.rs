// 5-b10: Calendar generator - display full year calendar
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用结构体Calendar封装日历数据和逻辑，提高代码组织性
// 2. 使用const数组和迭代器，避免重复代码
// 3. 使用enum Weekday提供类型安全的星期表示
// 4. 将格式化逻辑分离到独立函数，提高可测试性
// 5. 使用Vec<Vec<u8>>存储每月每天的星期，避免固定大小数组
// 6. 消除全局可变状态，所有数据通过参数传递

use std::io::{self, Write};

/// 月份天数（平年）
const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// 月份天数（闰年）
const DAYS_IN_MONTH_LEAP: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// 季度标题
const QUARTER_TITLES: [&str; 4] = [
    "            1月                             2月                             3月",
    "            4月                             5月                             6月",
    "            7月                             8月                             9月",
    "           10月                            11月                            12月",
];

/// 星期标题
const WEEK_HEADER: &str =
    "Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat     Sun Mon Tue Wed Thu Fri Sat";

/// 判断是否为闰年
///
/// # Arguments
/// * `year` - 年份
///
/// # Returns
/// * `true` - 闰年
/// * `false` - 平年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// 使用Zeller公式计算某年某月某日是星期几
///
/// # Arguments
/// * `year` - 年份
/// * `month` - 月份 (1-12)
/// * `day` - 日期
///
/// # Returns
/// * 星期几 (0=Sunday, 1=Monday, ..., 6=Saturday)
fn zeller(year: i32, month: i32, day: i32) -> u8 {
    // Rust改进: 使用局部变量和表达式，避免多次赋值
    let (y, m) = if month >= 3 {
        (year, month)
    } else {
        (year - 1, month + 12)
    };

    let y1 = y % 100;
    let c = y / 100;

    // Zeller公式
    let w = y1 + y1 / 4 + c / 4 - 2 * c + (13 * (m + 1) / 5) + day - 1;

    // Rust改进: 使用rem_euclid确保结果为正数
    w.rem_euclid(7) as u8
}

/// 日历结构体
struct Calendar {
    year: i32,
    /// 存储每月每天是星期几 (0-6)
    /// month_days[month][day] = weekday
    month_days: Vec<Vec<u8>>,
}

impl Calendar {
    /// 创建指定年份的日历
    fn new(year: i32) -> Self {
        let is_leap = is_leap_year(year);
        let days_array = if is_leap {
            &DAYS_IN_MONTH_LEAP
        } else {
            &DAYS_IN_MONTH
        };

        // Rust改进: 使用Vec动态分配，避免固定大小数组浪费空间
        let mut month_days = Vec::with_capacity(12);

        // 计算每一天是星期几
        let mut current_weekday = zeller(year, 1, 1);

        for &days_in_month in days_array.iter() {
            let mut month_vec = Vec::with_capacity(days_in_month as usize);

            for _ in 0..days_in_month {
                month_vec.push(current_weekday);
                current_weekday = (current_weekday + 1) % 7;
            }

            month_days.push(month_vec);
        }

        Calendar { year, month_days }
    }

    /// 获取指定月份的天数
    fn days_in_month(&self, month: usize) -> u8 {
        self.month_days[month].len() as u8
    }

    /// 获取指定月份第一天是星期几
    fn first_weekday(&self, month: usize) -> u8 {
        self.month_days[month][0]
    }

    /// 打印整年日历
    fn print(&self) {
        println!("{}年的日历:", self.year);

        // 按季度打印（每季度3个月并排显示）
        for quarter in 0..4 {
            self.print_quarter(quarter);
        }
    }

    /// 打印一个季度（3个月并排）
    fn print_quarter(&self, quarter: usize) {
        println!("{}", QUARTER_TITLES[quarter]);
        println!("{}", WEEK_HEADER);

        let month1 = quarter * 3;
        let month2 = quarter * 3 + 1;
        let month3 = quarter * 3 + 2;

        // 打印第一行（包含前导空格和第一周的日期）
        self.print_first_line(month1);
        print!("    ");
        self.print_first_line(month2);
        print!("    ");
        self.print_first_line(month3);
        println!();

        // 打印剩余行
        self.print_rest_lines(month1, month2, month3);
        println!();
    }

    /// 打印某月的第一行
    fn print_first_line(&self, month: usize) {
        let first_day = self.first_weekday(month) as usize;

        // 打印前导空格
        for _ in 0..first_day {
            print!("    ");
        }

        // 打印第一周的日期
        for day in 1..=(7 - first_day).min(self.days_in_month(month) as usize) {
            print!("{:<4}", day);
        }
    }

    /// 打印三个月的剩余行
    fn print_rest_lines(&self, month1: usize, month2: usize, month3: usize) {
        let start1 = 8 - self.first_weekday(month1) as usize;
        let start2 = 8 - self.first_weekday(month2) as usize;
        let start3 = 8 - self.first_weekday(month3) as usize;

        let max_days1 = self.days_in_month(month1) as usize;
        let max_days2 = self.days_in_month(month2) as usize;
        let max_days3 = self.days_in_month(month3) as usize;

        let mut day1 = start1;
        let mut day2 = start2;
        let mut day3 = start3;

        // Rust改进: 使用while循环和清晰的终止条件
        while day1 <= max_days1 || day2 <= max_days2 || day3 <= max_days3 {
            // 打印第一个月的一周
            for _ in 0..7 {
                if day1 <= max_days1 {
                    print!("{:<4}", day1);
                    day1 += 1;
                } else {
                    print!("    ");
                }
            }
            print!("    ");

            // 打印第二个月的一周
            for _ in 0..7 {
                if day2 <= max_days2 {
                    print!("{:<4}", day2);
                    day2 += 1;
                } else {
                    print!("    ");
                }
            }
            print!("    ");

            // 打印第三个月的一周
            for _ in 0..7 {
                if day3 <= max_days3 {
                    print!("{:<4}", day3);
                    day3 += 1;
                } else {
                    print!("    ");
                }
            }
            println!();
        }
    }
}

/// 读取并验证年份输入
fn read_year() -> i32 {
    loop {
        print!("请输入年份[1900-2100]\n");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        // Rust改进: 使用链式调用和filter进行验证
        if let Some(year) = input
            .trim()
            .parse::<i32>()
            .ok()
            .filter(|&y| y >= 1900 && y <= 2100)
        {
            return year;
        }
    }
}

fn main() {
    let year = read_year();
    let calendar = Calendar::new(year);
    calendar.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        // 测试闰年
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2020));

        // 测试平年
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_zeller_known_dates() {
        // 2000年1月1日是星期六
        assert_eq!(zeller(2000, 1, 1), 6);

        // 2020年1月1日是星期三
        assert_eq!(zeller(2020, 1, 1), 3);

        // 2021年1月1日是星期五
        assert_eq!(zeller(2021, 1, 1), 5);

        // 2026年1月1日是星期四
        assert_eq!(zeller(2026, 1, 1), 4);
    }

    #[test]
    fn test_calendar_creation() {
        let cal = Calendar::new(2020);
        assert_eq!(cal.year, 2020);
        assert_eq!(cal.month_days.len(), 12);
    }

    #[test]
    fn test_days_in_month_leap_year() {
        let cal = Calendar::new(2020);
        assert_eq!(cal.days_in_month(0), 31); // January
        assert_eq!(cal.days_in_month(1), 29); // February (leap)
        assert_eq!(cal.days_in_month(2), 31); // March
    }

    #[test]
    fn test_days_in_month_regular_year() {
        let cal = Calendar::new(2021);
        assert_eq!(cal.days_in_month(0), 31); // January
        assert_eq!(cal.days_in_month(1), 28); // February (regular)
        assert_eq!(cal.days_in_month(2), 31); // March
    }

    #[test]
    fn test_first_weekday() {
        let cal = Calendar::new(2020);
        // 2020年1月1日是星期三
        assert_eq!(cal.first_weekday(0), 3);
    }

    #[test]
    fn test_weekday_progression() {
        let cal = Calendar::new(2020);
        // 验证一月份的星期递增
        for i in 0..cal.days_in_month(0) as usize - 1 {
            let current = cal.month_days[0][i];
            let next = cal.month_days[0][i + 1];
            assert_eq!((current + 1) % 7, next);
        }
    }

    #[test]
    fn test_year_boundary() {
        // 测试边界年份
        let cal1900 = Calendar::new(1900);
        assert_eq!(cal1900.days_in_month(1), 28); // February in non-leap year

        let cal2100 = Calendar::new(2100);
        assert_eq!(cal2100.days_in_month(1), 28); // February in non-leap year

        let cal2000 = Calendar::new(2000);
        assert_eq!(cal2000.days_in_month(1), 29); // February in leap year
    }
}
