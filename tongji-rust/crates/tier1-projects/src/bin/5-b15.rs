// 5-b15: Character classification - count character types in 3 input lines
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用char迭代器和is_ascii_*方法，避免手动字节比较
// 2. 使用结构体CharCounts封装计数，提供类型安全
// 3. 使用match表达式替代多个if-else，更清晰
// 4. 自动处理UTF-8，无需手动处理汉字编码
// 5. 提取分类逻辑为纯函数，便于测试

use std::io::{self, BufRead};

/// 字符分类计数结构
#[derive(Debug, Default, PartialEq)]
struct CharCounts {
    uppercase: usize,
    lowercase: usize,
    digits: usize,
    spaces: usize,
    others: usize,
}

impl CharCounts {
    /// 累加另一个计数结果
    fn add(&mut self, other: &CharCounts) {
        self.uppercase += other.uppercase;
        self.lowercase += other.lowercase;
        self.digits += other.digits;
        self.spaces += other.spaces;
        self.others += other.others;
    }
}

/// 分类单个字符
///
/// # Arguments
/// * `ch` - 要分类的字符
///
/// # Returns
/// * 字符类型的枚举值
///
/// # Rust改进
/// 使用char的内置方法is_ascii_uppercase等，比手动范围检查更清晰
#[derive(Debug, PartialEq)]
enum CharType {
    Uppercase,
    Lowercase,
    Digit,
    Space,
    Other,
}

fn classify_char(ch: char) -> CharType {
    // Rust改进: 使用match + char内置方法，比C++的if-else链更优雅
    match ch {
        c if c.is_ascii_uppercase() => CharType::Uppercase,
        c if c.is_ascii_lowercase() => CharType::Lowercase,
        c if c.is_ascii_digit() => CharType::Digit,
        ' ' => CharType::Space,
        _ => CharType::Other,
    }
}

/// 统计字符串中各类字符的数量
///
/// # Arguments
/// * `line` - 输入字符串
///
/// # Returns
/// * CharCounts结构，包含各类字符的计数
///
/// # Rust改进
/// 1. 使用chars()迭代器自动处理UTF-8，无需手动处理多字节字符
/// 2. 使用filter过滤换行符，比C++的if判断更函数式
/// 3. 使用fold累积计数，比手动循环更简洁
fn count_chars(line: &str) -> CharCounts {
    let mut counts = CharCounts::default();

    // Rust改进: chars()自动处理UTF-8，汉字等多字节字符被正确识别为单个char
    // 不需要C++中的手动字节检查和j++跳过
    for ch in line.chars() {
        // 过滤换行符
        if ch == '\n' {
            continue;
        }

        match classify_char(ch) {
            CharType::Uppercase => counts.uppercase += 1,
            CharType::Lowercase => counts.lowercase += 1,
            CharType::Digit => counts.digits += 1,
            CharType::Space => counts.spaces += 1,
            CharType::Other => counts.others += 1,
        }
    }

    counts
}

fn main() {
    let stdin = io::stdin();
    let mut total = CharCounts::default();

    // 读取3行输入
    for i in 1..=3 {
        println!("请输入第{}行", i);

        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        // Rust改进: 函数式编程，将统计逻辑提取为纯函数
        let counts = count_chars(&line);
        total.add(&counts);
    }

    // 输出结果
    println!("大写 : {}", total.uppercase);
    println!("小写 : {}", total.lowercase);
    println!("数字 : {}", total.digits);
    println!("空格 : {}", total.spaces);
    println!("其他 ：{}", total.others);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_char_uppercase() {
        assert_eq!(classify_char('A'), CharType::Uppercase);
        assert_eq!(classify_char('Z'), CharType::Uppercase);
        assert_eq!(classify_char('M'), CharType::Uppercase);
    }

    #[test]
    fn test_classify_char_lowercase() {
        assert_eq!(classify_char('a'), CharType::Lowercase);
        assert_eq!(classify_char('z'), CharType::Lowercase);
        assert_eq!(classify_char('m'), CharType::Lowercase);
    }

    #[test]
    fn test_classify_char_digit() {
        assert_eq!(classify_char('0'), CharType::Digit);
        assert_eq!(classify_char('9'), CharType::Digit);
        assert_eq!(classify_char('5'), CharType::Digit);
    }

    #[test]
    fn test_classify_char_space() {
        assert_eq!(classify_char(' '), CharType::Space);
    }

    #[test]
    fn test_classify_char_other() {
        assert_eq!(classify_char('!'), CharType::Other);
        assert_eq!(classify_char('@'), CharType::Other);
        assert_eq!(classify_char('中'), CharType::Other);
        assert_eq!(classify_char('文'), CharType::Other);
    }

    #[test]
    fn test_count_chars_empty() {
        let counts = count_chars("");
        assert_eq!(counts, CharCounts::default());
    }

    #[test]
    fn test_count_chars_only_newline() {
        let counts = count_chars("\n");
        assert_eq!(counts, CharCounts::default());
    }

    #[test]
    fn test_count_chars_mixed() {
        let counts = count_chars("Hello World 123!");
        assert_eq!(counts.uppercase, 2); // H, W
        assert_eq!(counts.lowercase, 8); // e,l,l,o,o,r,l,d
        assert_eq!(counts.digits, 3);    // 1,2,3
        assert_eq!(counts.spaces, 2);    // two spaces
        assert_eq!(counts.others, 1);    // !
    }

    #[test]
    fn test_count_chars_chinese() {
        // Rust改进: UTF-8自动处理，汉字被正确识别为单个字符
        let counts = count_chars("你好World");
        assert_eq!(counts.uppercase, 1); // W
        assert_eq!(counts.lowercase, 4); // o,r,l,d
        assert_eq!(counts.others, 2);    // 你,好
    }

    #[test]
    fn test_count_chars_all_uppercase() {
        let counts = count_chars("ABCDEF");
        assert_eq!(counts.uppercase, 6);
        assert_eq!(counts.lowercase, 0);
        assert_eq!(counts.digits, 0);
        assert_eq!(counts.spaces, 0);
        assert_eq!(counts.others, 0);
    }

    #[test]
    fn test_count_chars_all_digits() {
        let counts = count_chars("0123456789");
        assert_eq!(counts.uppercase, 0);
        assert_eq!(counts.lowercase, 0);
        assert_eq!(counts.digits, 10);
        assert_eq!(counts.spaces, 0);
        assert_eq!(counts.others, 0);
    }

    #[test]
    fn test_count_chars_special_symbols() {
        let counts = count_chars("!@#$%^&*()");
        assert_eq!(counts.uppercase, 0);
        assert_eq!(counts.lowercase, 0);
        assert_eq!(counts.digits, 0);
        assert_eq!(counts.spaces, 0);
        assert_eq!(counts.others, 10);
    }

    #[test]
    fn test_count_chars_with_newline() {
        let counts = count_chars("ABC\n");
        assert_eq!(counts.uppercase, 3);
        // 换行符被过滤，不计入others
    }

    #[test]
    fn test_char_counts_add() {
        let mut total = CharCounts {
            uppercase: 1,
            lowercase: 2,
            digits: 3,
            spaces: 4,
            others: 5,
        };

        let other = CharCounts {
            uppercase: 10,
            lowercase: 20,
            digits: 30,
            spaces: 40,
            others: 50,
        };

        total.add(&other);

        assert_eq!(total.uppercase, 11);
        assert_eq!(total.lowercase, 22);
        assert_eq!(total.digits, 33);
        assert_eq!(total.spaces, 44);
        assert_eq!(total.others, 55);
    }

    #[test]
    fn test_count_chars_multiple_spaces() {
        let counts = count_chars("a   b   c");
        assert_eq!(counts.lowercase, 3);
        assert_eq!(counts.spaces, 6);
    }
}
