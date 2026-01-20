// 5-b12: String manipulation library (tj_str functions)
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用&str和String类型，自动处理UTF-8编码和内存安全
// 2. 使用标准库的字符串方法，避免手动索引和越界风险
// 3. 使用Option<usize>返回查找结果，比C++的0/-1更清晰
// 4. 使用迭代器和函数式编程，代码更简洁高效
// 5. 所有函数都是纯函数，无副作用（除了需要修改的函数）
// 6. 完整的单元测试覆盖所有边界情况

#![allow(dead_code)] // 允许未使用的函数，因为这是一个库

/// 计算字符串长度
///
/// # Arguments
/// * `s` - 输入字符串
///
/// # Returns
/// * `usize` - 字符串长度
///
/// # Rust改进
/// Rust的&str自带len()方法，但这里为了演示实现原理
fn tj_strlen(s: &str) -> usize {
    s.len()
}

/// 字符串拼接
///
/// # Arguments
/// * `s1` - 目标字符串
/// * `s2` - 源字符串
///
/// # Returns
/// * `String` - 拼接后的字符串
///
/// # Rust改进
/// 返回新String而不是修改参数，避免内存安全问题
fn tj_strcat(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

/// 字符串拼接（限制长度）
///
/// # Arguments
/// * `s1` - 目标字符串
/// * `s2` - 源字符串
/// * `len` - 最大拼接长度
///
/// # Returns
/// * `String` - 拼接后的字符串
///
/// # Rust改进
/// 使用切片操作，自动处理边界情况
fn tj_strncat(s1: &str, s2: &str, len: usize) -> String {
    let append_len = len.min(s2.len());
    format!("{}{}", s1, &s2[..append_len])
}

/// 字符串复制
///
/// # Arguments
/// * `s` - 源字符串
///
/// # Returns
/// * `String` - 复制的字符串
///
/// # Rust改进
/// Rust的String自动管理内存，无需手动复制
fn tj_strcpy(s: &str) -> String {
    s.to_string()
}

/// 字符串复制（限制长度）
///
/// # Arguments
/// * `s` - 源字符串
/// * `len` - 最大复制长度
///
/// # Returns
/// * `String` - 复制的字符串
///
/// # Rust改进
/// 使用切片操作，自动处理边界
fn tj_strncpy(s: &str, len: usize) -> String {
    let copy_len = len.min(s.len());
    s[..copy_len].to_string()
}

/// 字符串比较
///
/// # Arguments
/// * `s1` - 第一个字符串
/// * `s2` - 第二个字符串
///
/// # Returns
/// * `i32` - 比较结果：负数表示s1<s2，0表示相等，正数表示s1>s2
///
/// # Rust改进
/// 使用Ordering枚举更清晰，但这里为了兼容C接口返回i32
fn tj_strcmp(s1: &str, s2: &str) -> i32 {
    match s1.cmp(s2) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// 字符串比较（忽略大小写）
///
/// # Arguments
/// * `s1` - 第一个字符串
/// * `s2` - 第二个字符串
///
/// # Returns
/// * `i32` - 比较结果
///
/// # Rust改进
/// 使用to_lowercase()方法，自动处理Unicode
fn tj_strcasecmp(s1: &str, s2: &str) -> i32 {
    let s1_lower = s1.to_lowercase();
    let s2_lower = s2.to_lowercase();

    match s1_lower.cmp(&s2_lower) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// 字符串比较（限制长度）
///
/// # Arguments
/// * `s1` - 第一个字符串
/// * `s2` - 第二个字符串
/// * `len` - 比较长度
///
/// # Returns
/// * `i32` - 比较结果
///
/// # Rust改进
/// 使用切片操作，避免越界
fn tj_strncmp(s1: &str, s2: &str, len: usize) -> i32 {
    let len1 = len.min(s1.len());
    let len2 = len.min(s2.len());
    let cmp_len = len1.min(len2);

    match s1[..cmp_len].cmp(&s2[..cmp_len]) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => {
            if len1 < len2 {
                -1
            } else if len1 > len2 {
                1
            } else {
                0
            }
        }
        std::cmp::Ordering::Greater => 1,
    }
}

/// 字符串比较（忽略大小写，限制长度）
///
/// # Arguments
/// * `s1` - 第一个字符串
/// * `s2` - 第二个字符串
/// * `len` - 比较长度
///
/// # Returns
/// * `i32` - 比较结果
///
/// # Rust改进
/// 结合切片和to_lowercase()
fn tj_strcasencmp(s1: &str, s2: &str, len: usize) -> i32 {
    let len1 = len.min(s1.len());
    let len2 = len.min(s2.len());
    let cmp_len = len1.min(len2);

    let s1_lower = s1[..cmp_len].to_lowercase();
    let s2_lower = s2[..cmp_len].to_lowercase();

    match s1_lower.cmp(&s2_lower) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => {
            if len1 < len2 {
                -1
            } else if len1 > len2 {
                1
            } else {
                0
            }
        }
        std::cmp::Ordering::Greater => 1,
    }
}

/// 字符串转大写
///
/// # Arguments
/// * `s` - 输入字符串
///
/// # Returns
/// * `String` - 转换后的字符串
///
/// # Rust改进
/// 使用to_uppercase()方法，自动处理Unicode
fn tj_strupr(s: &str) -> String {
    s.to_uppercase()
}

/// 字符串转小写
///
/// # Arguments
/// * `s` - 输入字符串
///
/// # Returns
/// * `String` - 转换后的字符串
///
/// # Rust改进
/// 使用to_lowercase()方法，自动处理Unicode
fn tj_strlwr(s: &str) -> String {
    s.to_lowercase()
}

/// 查找字符首次出现的位置
///
/// # Arguments
/// * `s` - 输入字符串
/// * `ch` - 要查找的字符
///
/// # Returns
/// * `Option<usize>` - 字符位置（从0开始），未找到返回None
///
/// # Rust改进
/// 使用Option<usize>比C的返回0/-1更清晰
fn tj_strchr(s: &str, ch: char) -> Option<usize> {
    s.find(ch)
}

/// 查找子串首次出现的位置
///
/// # Arguments
/// * `s` - 输入字符串
/// * `substr` - 要查找的子串
///
/// # Returns
/// * `Option<usize>` - 子串位置（从0开始），未找到返回None
///
/// # Rust改进
/// 使用find()方法，内部使用高效的字符串匹配算法
fn tj_strstr(s: &str, substr: &str) -> Option<usize> {
    s.find(substr)
}

/// 查找字符最后一次出现的位置
///
/// # Arguments
/// * `s` - 输入字符串
/// * `ch` - 要查找的字符
///
/// # Returns
/// * `Option<usize>` - 字符位置（从0开始），未找到返回None
///
/// # Rust改进
/// 使用rfind()方法，从右向左查找
fn tj_strrchr(s: &str, ch: char) -> Option<usize> {
    s.rfind(ch)
}

/// 查找子串最后一次出现的位置
///
/// # Arguments
/// * `s` - 输入字符串
/// * `substr` - 要查找的子串
///
/// # Returns
/// * `Option<usize>` - 子串位置（从0开始），未找到返回None
///
/// # Rust改进
/// 使用rfind()方法，从右向左查找
fn tj_strrstr(s: &str, substr: &str) -> Option<usize> {
    s.rfind(substr)
}

/// 字符串反转
///
/// # Arguments
/// * `s` - 输入字符串
///
/// # Returns
/// * `String` - 反转后的字符串
///
/// # Rust改进
/// 使用chars().rev()迭代器，自动处理UTF-8字符边界
fn tj_strrev(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    // 演示所有函数的使用
    println!("=== tj_strlen 测试 ===");
    println!("长度: {}", tj_strlen("hello"));

    println!("\n=== tj_strcat 测试 ===");
    println!("拼接: {}", tj_strcat("hello", " world"));

    println!("\n=== tj_strncat 测试 ===");
    println!("限制拼接: {}", tj_strncat("hello", " world", 3));

    println!("\n=== tj_strcmp 测试 ===");
    println!("比较: {}", tj_strcmp("abc", "abd"));

    println!("\n=== tj_strcasecmp 测试 ===");
    println!("忽略大小写比较: {}", tj_strcasecmp("Hello", "hello"));

    println!("\n=== tj_strupr/tj_strlwr 测试 ===");
    println!("转大写: {}", tj_strupr("hello"));
    println!("转小写: {}", tj_strlwr("HELLO"));

    println!("\n=== tj_strchr 测试 ===");
    println!("查找字符: {:?}", tj_strchr("hello", 'l'));

    println!("\n=== tj_strstr 测试 ===");
    println!("查找子串: {:?}", tj_strstr("hello world", "world"));

    println!("\n=== tj_strrev 测试 ===");
    println!("反转: {}", tj_strrev("hello"));
}

#[cfg(test)]
mod tests {
    use super::*;

    // tj_strlen 测试
    #[test]
    fn test_strlen() {
        assert_eq!(tj_strlen("abcdefghijklmnopqrstuvwxyz"), 26);
        assert_eq!(tj_strlen(""), 0);
        assert_eq!(tj_strlen("hello"), 5);
    }

    // tj_strcat 测试
    #[test]
    fn test_strcat() {
        assert_eq!(tj_strcat("abc", "def"), "abcdef");
        assert_eq!(tj_strcat("", "hello"), "hello");
        assert_eq!(tj_strcat("hello", ""), "hello");
    }

    // tj_strncat 测试
    #[test]
    fn test_strncat() {
        assert_eq!(tj_strncat("abc", "defgh", 3), "abcdef");
        assert_eq!(tj_strncat("abc", "de", 10), "abcde");
        assert_eq!(tj_strncat("abc", "defgh", 0), "abc");
    }

    // tj_strcpy 测试
    #[test]
    fn test_strcpy() {
        assert_eq!(tj_strcpy("hello"), "hello");
        assert_eq!(tj_strcpy(""), "");
    }

    // tj_strncpy 测试
    #[test]
    fn test_strncpy() {
        assert_eq!(tj_strncpy("hello", 3), "hel");
        assert_eq!(tj_strncpy("hello", 10), "hello");
        assert_eq!(tj_strncpy("hello", 0), "");
    }

    // tj_strcmp 测试
    #[test]
    fn test_strcmp() {
        assert_eq!(tj_strcmp("abc", "abc"), 0);
        assert!(tj_strcmp("abc", "abd") < 0);
        assert!(tj_strcmp("abd", "abc") > 0);
        assert!(tj_strcmp("hello", "hell") > 0);
        assert!(tj_strcmp("hell", "hello") < 0);
    }

    // tj_strcasecmp 测试
    #[test]
    fn test_strcasecmp() {
        assert_eq!(tj_strcasecmp("hello", "HELLO"), 0);
        assert_eq!(tj_strcasecmp("HeLLo", "hEllO"), 0);
        assert!(tj_strcasecmp("abc", "ABD") < 0);
        assert!(tj_strcasecmp("ABD", "abc") > 0);
    }

    // tj_strncmp 测试
    #[test]
    fn test_strncmp() {
        assert_eq!(tj_strncmp("horse", "house", 2), 0);
        assert!(tj_strncmp("horse", "house", 3) < 0);
        assert_eq!(tj_strncmp("hello", "hello", 5), 0);
        assert_eq!(tj_strncmp("hello", "hello", 10), 0);
        assert!(tj_strncmp("hell", "hello", 10) < 0);
    }

    // tj_strcasencmp 测试
    #[test]
    fn test_strcasencmp() {
        assert_eq!(tj_strcasencmp("HeLLo", "hello", 5), 0);
        assert_eq!(tj_strcasencmp("HORSE", "house", 2), 0);
        assert!(tj_strcasencmp("HORSE", "house", 3) < 0);
        assert_eq!(tj_strcasencmp("HeLLo", "hello", 10), 0);
    }

    // tj_strupr 测试
    #[test]
    fn test_strupr() {
        assert_eq!(tj_strupr("hello"), "HELLO");
        assert_eq!(tj_strupr("HeLLo"), "HELLO");
        assert_eq!(tj_strupr("123abc"), "123ABC");
        assert_eq!(tj_strupr(""), "");
    }

    // tj_strlwr 测试
    #[test]
    fn test_strlwr() {
        assert_eq!(tj_strlwr("HELLO"), "hello");
        assert_eq!(tj_strlwr("HeLLo"), "hello");
        assert_eq!(tj_strlwr("123ABC"), "123abc");
        assert_eq!(tj_strlwr(""), "");
    }

    // tj_strchr 测试
    #[test]
    fn test_strchr() {
        assert_eq!(tj_strchr("This is a pencil.", 'T'), Some(0));
        assert_eq!(tj_strchr("This is a pencil.", 'i'), Some(2));
        assert_eq!(tj_strchr("This is a pencil.", ' '), Some(4));
        assert_eq!(tj_strchr("This is a pencil.", 'x'), None);
    }

    // tj_strstr 测试
    #[test]
    fn test_strstr() {
        assert_eq!(tj_strstr("This is a pencil.", "T"), Some(0));
        assert_eq!(tj_strstr("This is a pencil.", "is"), Some(2));
        assert_eq!(tj_strstr("This is a pencil.", "pencil"), Some(10));
        assert_eq!(tj_strstr("This is a pencil.", "Pencil"), None);
        assert_eq!(tj_strstr("aabbbceddbbbceeeff", "bb"), Some(2));
        assert_eq!(tj_strstr("aabbbceddbbbceeeff", "bbb"), Some(2));
        assert_eq!(tj_strstr("aabbbceddbbbceeeff", "bbbb"), None);
    }

    // tj_strrchr 测试
    #[test]
    fn test_strrchr() {
        assert_eq!(tj_strrchr("This is a pencil.", 'T'), Some(0));
        assert_eq!(tj_strrchr("This is a pencil.", 'i'), Some(14));
        assert_eq!(tj_strrchr("This is a pencil.", ' '), Some(9));
        assert_eq!(tj_strrchr("This is a pencil.", 'x'), None);
    }

    // tj_strrstr 测试
    #[test]
    fn test_strrstr() {
        assert_eq!(tj_strrstr("This is a pencil.", "T"), Some(0));
        assert_eq!(tj_strrstr("This is a pencil.", "is"), Some(5));
        assert_eq!(tj_strrstr("This is a pencil.", "pencil"), Some(10));
        assert_eq!(tj_strrstr("This is a pencil.", "Pencil"), None);
        assert_eq!(tj_strrstr("aabbbceddbbbceeeff", "bb"), Some(10));
        assert_eq!(tj_strrstr("aabbbceddbbbceeeff", "bbb"), Some(9));
        assert_eq!(tj_strrstr("aabbbceddbbbceeeff", "ce"), Some(12));
    }

    // tj_strrev 测试
    #[test]
    fn test_strrev() {
        assert_eq!(tj_strrev("This is a pencil."), ".licnep a si sihT");
        assert_eq!(tj_strrev("aabbbceddbbbceeeff"), "ffeeecbbbddecbbbaa");
        assert_eq!(tj_strrev(""), "");
        assert_eq!(tj_strrev("a"), "a");
        assert_eq!(tj_strrev("hello"), "olleh");
    }

    // 边界情况测试
    #[test]
    fn test_empty_strings() {
        assert_eq!(tj_strlen(""), 0);
        assert_eq!(tj_strcat("", ""), "");
        assert_eq!(tj_strcmp("", ""), 0);
        assert_eq!(tj_strrev(""), "");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(tj_strlen("a"), 1);
        assert_eq!(tj_strrev("a"), "a");
        assert_eq!(tj_strchr("a", 'a'), Some(0));
    }

    #[test]
    fn test_special_chars() {
        assert_eq!(tj_strupr("123*#@"), "123*#@");
        assert_eq!(tj_strlwr("123*#@"), "123*#@");
        assert_eq!(tj_strchr("hello*world", '*'), Some(5));
    }
}
