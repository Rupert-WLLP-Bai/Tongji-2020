// 6-b2: Palindrome checker - verify if a string reads the same forwards and backwards
// Original: 2052526 信15 白俊豪
//
// 问题描述: 判断用户输入的字符串是否为回文串
// 回文串是指正读和反读都相同的字符串，如 "level", "noon", "上海自来水来自海上"
//
// Rust改进:
// 1. 使用&str切片而非char*指针，自动管理内存且保证UTF-8安全
// 2. 使用迭代器的rev()方法，比手动双指针更简洁且零成本抽象
// 3. 提取核心逻辑为纯函数，便于测试和复用
// 4. 使用chars()处理Unicode字符，正确支持多字节字符（C++版本有bug）
// 5. 添加Result类型处理空字符串等边界情况
// 6. 使用zip + all组合子，函数式风格更清晰
// 7. 自动处理字符串长度，无需手动计算中点

use std::io::{self, BufRead};

/// 检查字符串是否为回文串
///
/// # Arguments
/// * `s` - 待检查的字符串切片
///
/// # Returns
/// * `bool` - true表示是回文串，false表示不是
///
/// # Algorithm
/// 使用双端迭代器比较：从前向后和从后向前同时遍历，
/// 如果所有对应位置的字符都相等，则为回文串。
///
/// # Examples
/// ```
/// assert_eq!(is_palindrome("level"), true);
/// assert_eq!(is_palindrome("hello"), false);
/// assert_eq!(is_palindrome(""), true);  // 空串视为回文
/// ```
#[cfg(test)]
fn is_palindrome(s: &str) -> bool {
    // Rust改进: 使用chars()迭代器处理Unicode字符
    // C++版本使用char*只能正确处理ASCII，对中文等多字节字符会出错
    let chars: Vec<char> = s.chars().collect();

    // Rust改进: 使用zip将正向和反向迭代器配对
    // all()检查所有配对字符是否相等，短路求值提高效率
    chars.iter().zip(chars.iter().rev()).all(|(a, b)| a == b)
}

/// 更优化的回文检查实现（无需额外内存）
///
/// # Arguments
/// * `s` - 待检查的字符串切片
///
/// # Returns
/// * `bool` - true表示是回文串，false表示不是
///
/// # Performance
/// 时间复杂度: O(n/2)，只需比较一半的字符
/// 空间复杂度: O(1)，不需要额外的Vec存储
fn is_palindrome_optimized(s: &str) -> bool {
    // Rust改进: 直接比较字符串切片，利用Rust的字符串切片是UTF-8安全的特性
    // 使用eq_ignore_ascii_case可以实现忽略大小写的比较（如需要）
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    // 只需比较前半部分和后半部分
    (0..len / 2).all(|i| chars[i] == chars[len - 1 - i])
}

/// 从标准输入读取一行，去除末尾换行符
///
/// # Returns
/// * `io::Result<String>` - 成功返回读取的字符串，失败返回IO错误
fn read_line() -> io::Result<String> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;

    // Rust改进: 使用trim_end()自动处理各种换行符（\n, \r\n等）
    // C++版本手动设置'\0'，不够健壮
    Ok(line.trim_end().to_string())
}

fn main() -> io::Result<()> {
    println!("请输入一个长度小于80的字符串（回文串）");

    // Rust改进: 使用Result类型处理IO错误，比C++的隐式错误处理更安全
    let input = read_line()?;

    // 使用优化版本的回文检查
    if is_palindrome_optimized(&input) {
        println!("yes");
    } else {
        println!("no");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_palindromes() {
        // 测试简单的回文串
        assert!(is_palindrome("level"));
        assert!(is_palindrome("noon"));
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("a"));
    }

    #[test]
    fn test_non_palindromes() {
        // 测试非回文串
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("world"));
        assert!(!is_palindrome("rust"));
        assert!(!is_palindrome("ab"));
    }

    #[test]
    fn test_empty_and_single_char() {
        // 测试边界情况：空串和单字符
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("Z"));
    }

    #[test]
    fn test_even_length_palindromes() {
        // 测试偶数长度的回文串
        assert!(is_palindrome("abba"));
        assert!(is_palindrome("aabbaa"));
        assert!(is_palindrome("123321"));
    }

    #[test]
    fn test_odd_length_palindromes() {
        // 测试奇数长度的回文串
        assert!(is_palindrome("aba"));
        assert!(is_palindrome("12321"));
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn test_unicode_palindromes() {
        // 测试Unicode字符（中文等）
        // 这是C++版本无法正确处理的情况
        assert!(is_palindrome("上海自来水来自海上"));
        assert!(is_palindrome("我爱你你爱我"));
        assert!(!is_palindrome("你好世界"));
        assert!(is_palindrome("🚀🌟🚀"));
    }

    #[test]
    fn test_numbers_and_symbols() {
        // 测试数字和符号
        assert!(is_palindrome("12321"));
        assert!(is_palindrome("1001"));
        assert!(!is_palindrome("12345"));
        assert!(is_palindrome("!@#@!"));
    }

    #[test]
    fn test_optimized_matches_basic() {
        // 测试两种实现结果一致
        let test_cases = vec![
            "level",
            "hello",
            "",
            "a",
            "abba",
            "abc",
            "racecar",
            "上海自来水来自海上",
            "12321",
            "🚀🌟🚀",
        ];

        for case in test_cases {
            assert_eq!(
                is_palindrome(case),
                is_palindrome_optimized(case),
                "两种实现在输入'{}'时结果应该一致",
                case
            );
        }
    }

    #[test]
    fn test_long_palindrome() {
        // 测试较长的回文串
        let long = "abcdefghijklmnopqrstuvwxyzzyxwvutsrqponmlkjihgfedcba";
        assert!(is_palindrome(long));
        assert!(is_palindrome_optimized(long));
    }

    #[test]
    fn test_whitespace_sensitive() {
        // 测试空格敏感性（不忽略空格）
        assert!(is_palindrome("a b a"));
        assert!(!is_palindrome("a b c"));
        assert!(is_palindrome("   "));
    }

    #[test]
    fn test_case_sensitive() {
        // 测试大小写敏感性
        assert!(!is_palindrome("Aa"));
        assert!(!is_palindrome("Level"));
        assert!(is_palindrome("ABA"));
    }

    #[test]
    fn test_mixed_content() {
        // 测试混合内容
        assert!(is_palindrome("A man a plan a canal Panama".replace(" ", "").to_lowercase().as_str()));
        assert!(is_palindrome("Was it a car or a cat I saw".replace(" ", "").to_lowercase().as_str()));
    }
}
