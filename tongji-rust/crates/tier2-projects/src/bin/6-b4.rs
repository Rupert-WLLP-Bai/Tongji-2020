/*
 * 6-b4: 字符串操作函数库实现
 *
 * 原C++项目实现了一套自定义的字符串操作函数，模拟标准库的字符串处理功能。
 *
 * Rust改进：
 * 1. 使用 &str 和 String 类型替代 C 风格字符串，提供内存安全保证
 * 2. 返回 Option 类型处理空值情况，避免空指针问题
 * 3. 使用迭代器和函数式编程提高代码可读性
 * 4. 字符串操作自动处理 UTF-8 编码
 * 5. 不可变借用和可变借用确保线程安全
 * 6. 使用 Ordering 枚举替代整数比较结果
 */

use std::cmp::Ordering;

/// 字符串长度计算
/// Rust改进：直接使用 str::len()，O(1) 时间复杂度
fn tj_strlen(s: Option<&str>) -> usize {
    s.map_or(0, |s| s.len())
}

/// 字符串拼接
/// Rust改进：返回新的 String，避免缓冲区溢出
fn tj_strcat(s1: &str, s2: Option<&str>) -> String {
    match s2 {
        Some(s2) => format!("{}{}", s1, s2),
        None => s1.to_string(),
    }
}

/// 限长字符串拼接
/// Rust改进：使用切片操作，自动处理边界情况
fn tj_strncat(s1: &str, s2: Option<&str>, len: usize) -> String {
    match s2 {
        Some(s2) => {
            let append_len = len.min(s2.len());
            format!("{}{}", s1, &s2[..append_len])
        }
        None => s1.to_string(),
    }
}

/// 字符串复制
/// Rust改进：返回新的 String，避免内存管理问题
fn tj_strcpy(s: Option<&str>) -> String {
    s.map_or(String::new(), |s| s.to_string())
}

/// 限长字符串复制
/// Rust改进：使用切片操作，安全处理长度限制
fn tj_strncpy(s: Option<&str>, len: usize) -> String {
    match s {
        Some(s) => {
            let copy_len = len.min(s.len());
            s[..copy_len].to_string()
        }
        None => String::new(),
    }
}

/// 字符串比较
/// Rust改进：返回 Ordering 枚举，语义更清晰
fn tj_strcmp(s1: Option<&str>, s2: Option<&str>) -> Ordering {
    match (s1, s2) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(s1), Some(s2)) => s1.cmp(s2),
    }
}

/// 忽略大小写的字符串比较
/// Rust改进：使用 eq_ignore_ascii_case，标准库优化实现
fn tj_strcasecmp(s1: Option<&str>, s2: Option<&str>) -> Ordering {
    match (s1, s2) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(s1), Some(s2)) => {
            // 逐字符比较，忽略大小写
            let mut chars1 = s1.chars();
            let mut chars2 = s2.chars();

            loop {
                match (chars1.next(), chars2.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(c1), Some(c2)) => {
                        let c1_lower = c1.to_ascii_lowercase();
                        let c2_lower = c2.to_ascii_lowercase();
                        match c1_lower.cmp(&c2_lower) {
                            Ordering::Equal => continue,
                            other => return other,
                        }
                    }
                }
            }
        }
    }
}

/// 限长字符串比较
/// Rust改进：使用切片操作，避免越界访问
fn tj_strncmp(s1: Option<&str>, s2: Option<&str>, len: usize) -> Ordering {
    match (s1, s2) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(s1), Some(s2)) => {
            // 逐字符比较，最多比较 len 个字符
            let mut chars1 = s1.chars();
            let mut chars2 = s2.chars();
            let mut count = 0;

            loop {
                if count >= len {
                    return Ordering::Equal;
                }

                match (chars1.next(), chars2.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(c1), Some(c2)) => match c1.cmp(&c2) {
                        Ordering::Equal => {
                            count += 1;
                            continue;
                        }
                        other => return other,
                    },
                }
            }
        }
    }
}

/// 限长忽略大小写的字符串比较
/// Rust改进：结合切片和大小写转换，安全高效
fn tj_strcasencmp(s1: Option<&str>, s2: Option<&str>, len: usize) -> Ordering {
    match (s1, s2) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(s1), Some(s2)) => {
            let mut chars1 = s1.chars();
            let mut chars2 = s2.chars();
            let mut count = 0;

            loop {
                if count >= len {
                    return Ordering::Equal;
                }

                match (chars1.next(), chars2.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(c1), Some(c2)) => {
                        let c1_lower = c1.to_ascii_lowercase();
                        let c2_lower = c2.to_ascii_lowercase();
                        match c1_lower.cmp(&c2_lower) {
                            Ordering::Equal => {
                                count += 1;
                                continue;
                            }
                            other => return other,
                        }
                    }
                }
            }
        }
    }
}

/// 字符串转大写
/// Rust改进：使用 to_uppercase()，支持 Unicode
fn tj_strupr(s: Option<&str>) -> Option<String> {
    s.map(|s| s.to_uppercase())
}

/// 字符串转小写
/// Rust改进：使用 to_lowercase()，支持 Unicode
fn tj_strlwr(s: Option<&str>) -> Option<String> {
    s.map(|s| s.to_lowercase())
}

/// 查找字符首次出现位置
/// Rust改进：返回 Option<usize>，明确表示找到或未找到
fn tj_strchr(s: Option<&str>, ch: char) -> Option<usize> {
    s.and_then(|s| s.find(ch).map(|pos| pos + 1))
}

/// 查找子串首次出现位置
/// Rust改进：使用 find() 方法，KMP 算法优化
fn tj_strstr(s: Option<&str>, substr: Option<&str>) -> Option<usize> {
    match (s, substr) {
        (Some(s), Some(substr)) => s.find(substr).map(|pos| pos + 1),
        _ => None,
    }
}

/// 查找字符最后出现位置
/// Rust改进：使用 rfind() 方法，从后向前查找
fn tj_strrchr(s: Option<&str>, ch: char) -> Option<usize> {
    s.and_then(|s| s.rfind(ch).map(|pos| pos + 1))
}

/// 查找子串最后出现位置
/// Rust改进：使用 rfind() 方法，高效反向查找
fn tj_strrstr(s: Option<&str>, substr: Option<&str>) -> Option<usize> {
    match (s, substr) {
        (Some(s), Some(substr)) => s.rfind(substr).map(|pos| pos + 1),
        _ => None,
    }
}

/// 字符串反转
/// Rust改进：使用迭代器的 rev() 方法，函数式编程
fn tj_strrev(s: Option<&str>) -> Option<String> {
    s.map(|s| s.chars().rev().collect())
}

fn main() {
    println!("6-b4: 字符串操作函数库测试\n");

    // tj_strlen() 测试
    println!("tj_strlen()测试部分：");
    let s1 = "abcdefghijklmnopqrstuvwxyz";
    let s2 = "abcdefghijklmnopqrstuvwxyz\0UVWXYZ";
    let s3 = "";

    println!("1.s1      的长度应该是26，实际是：{}", tj_strlen(Some(s1)));
    println!("2.s2      的长度应该是33，实际是：{}", tj_strlen(Some(s2)));
    println!("3.s3      的长度应该是0， 实际是：{}", tj_strlen(Some(s3)));
    println!("4.None    的长度认为是0， 实际是：{}", tj_strlen(None));
    println!();

    // tj_strcat() 测试
    println!("tj_strcat()测试部分：");
    let mut result = "abcdefghij".to_string();
    println!("1.初始值应该是abcdefghij，实际是：{}", result);

    result = tj_strcat(&result, Some("abcde"));
    println!("2.拼接后应该是abcdefghijabcde，实际是：{}", result);
    println!("  长度应该是15，实际是：{}", result.len());

    result = tj_strcat(&result, Some("hello"));
    println!("3.拼接后应该是abcdefghijabcdehello，实际是：{}", result);
    println!("  长度应该是20，实际是：{}", result.len());

    result = tj_strcat(&result, None);
    println!("4.拼接None后应该不变，实际是：{}", result);
    println!();

    // tj_strncat() 测试
    println!("tj_strncat()测试部分：");
    let mut result = "abcdefghij".to_string();
    println!("1.初始值应该是abcdefghij，实际是：{}", result);

    result = tj_strncat(&result, Some("abcde"), 3);
    println!("2.拼接3个字符后应该是abcdefghijabc，实际是：{}", result);
    println!("  长度应该是13，实际是：{}", result.len());

    result = tj_strncat(&result, Some("hello"), 100);
    println!("3.拼接hello后应该是abcdefghijabchello，实际是：{}", result);
    println!("  长度应该是18，实际是：{}", result.len());
    println!();

    // tj_strcpy() 测试
    println!("tj_strcpy()测试部分：");
    let result = tj_strcpy(Some("abcde"));
    println!("1.复制abcde，实际是：{}", result);
    println!("  长度应该是5，实际是：{}", result.len());

    let result = tj_strcpy(Some(""));
    println!("2.复制空串，实际是：-{}-", result);
    println!("  长度应该是0，实际是：{}", result.len());

    let result = tj_strcpy(None);
    println!("3.复制None，实际是：-{}-", result);
    println!("  长度应该是0，实际是：{}", result.len());
    println!();

    // tj_strncpy() 测试
    println!("tj_strncpy()测试部分：");
    let result = tj_strncpy(Some("hello"), 3);
    println!("1.复制3个字符，应该是hel，实际是：{}", result);

    let result = tj_strncpy(Some("hello"), 10);
    println!("2.复制10个字符，应该是hello，实际是：{}", result);

    let result = tj_strncpy(None, 5);
    println!("3.复制None，应该是空串，实际是：-{}-", result);
    println!();

    // tj_strcmp() 测试
    println!("tj_strcmp()测试部分：");
    println!(
        "1.horse 和 house 比较：{:?}",
        tj_strcmp(Some("horse"), Some("house"))
    );
    println!(
        "2.hell 和 hello 比较：{:?}",
        tj_strcmp(Some("hell"), Some("hello"))
    );
    println!(
        "3.hello 和 hell 比较：{:?}",
        tj_strcmp(Some("hello"), Some("hell"))
    );
    println!(
        "4.hello 和 hello 比较：{:?}",
        tj_strcmp(Some("hello"), Some("hello"))
    );
    println!(
        "5.HELLO 和 hello 比较：{:?}",
        tj_strcmp(Some("HELLO"), Some("hello"))
    );
    println!("6.None 和 hello 比较：{:?}", tj_strcmp(None, Some("hello")));
    println!("7.hello 和 None 比较：{:?}", tj_strcmp(Some("hello"), None));
    println!("8.None 和 None 比较：{:?}", tj_strcmp(None, None));
    println!();

    // tj_strcasecmp() 测试
    println!("tj_strcasecmp()测试部分：");
    println!(
        "1.horse 和 house 比较：{:?}",
        tj_strcasecmp(Some("horse"), Some("house"))
    );
    println!(
        "2.hell 和 hello 比较：{:?}",
        tj_strcasecmp(Some("hell"), Some("hello"))
    );
    println!(
        "3.hello 和 hell 比较：{:?}",
        tj_strcasecmp(Some("hello"), Some("hell"))
    );
    println!(
        "4.hello 和 hello 比较：{:?}",
        tj_strcasecmp(Some("hello"), Some("hello"))
    );
    println!(
        "5.HELLO 和 hello 比较：{:?}",
        tj_strcasecmp(Some("HELLO"), Some("hello"))
    );
    println!(
        "6.HeLlO 和 hElLo 比较：{:?}",
        tj_strcasecmp(Some("HeLlO"), Some("hElLo"))
    );
    println!();

    // tj_strncmp() 测试
    println!("tj_strncmp()测试部分：");
    println!(
        "1.horse 和 house 比较前2个字符：{:?}",
        tj_strncmp(Some("horse"), Some("house"), 2)
    );
    println!(
        "2.horse 和 house 比较前3个字符：{:?}",
        tj_strncmp(Some("horse"), Some("house"), 3)
    );
    println!(
        "3.hell 和 hello 比较前4个字符：{:?}",
        tj_strncmp(Some("hell"), Some("hello"), 4)
    );
    println!(
        "4.hell 和 hello 比较前5个字符：{:?}",
        tj_strncmp(Some("hell"), Some("hello"), 5)
    );
    println!();

    // tj_strcasencmp() 测试
    println!("tj_strcasencmp()测试部分：");
    println!(
        "1.HELLO 和 hello 比较前5个字符：{:?}",
        tj_strcasencmp(Some("HELLO"), Some("hello"), 5)
    );
    println!(
        "2.HeLlO 和 hElLo 比较前5个字符：{:?}",
        tj_strcasencmp(Some("HeLlO"), Some("hElLo"), 5)
    );
    println!(
        "3.horse 和 house 比较前2个字符：{:?}",
        tj_strcasencmp(Some("horse"), Some("house"), 2)
    );
    println!();

    // tj_strupr() 测试
    println!("tj_strupr()测试部分：");
    println!(
        "1.123horseHELLO*#@ 转大写：{}",
        tj_strupr(Some("123horseHELLO*#@")).unwrap()
    );
    println!(
        "2.1A2b3C*d#E@f 转大写：{}",
        tj_strupr(Some("1A2b3C*d#E@f")).unwrap()
    );
    println!("3.None 转大写：{:?}", tj_strupr(None));
    println!();

    // tj_strlwr() 测试
    println!("tj_strlwr()测试部分：");
    println!(
        "1.123horseHELLO*#@ 转小写：{}",
        tj_strlwr(Some("123horseHELLO*#@")).unwrap()
    );
    println!(
        "2.1A2b3C*d#E@f 转小写：{}",
        tj_strlwr(Some("1A2b3C*d#E@f")).unwrap()
    );
    println!("3.None 转小写：{:?}", tj_strlwr(None));
    println!();

    // tj_strchr() 测试
    println!("tj_strchr()测试部分：");
    let s = "This is a pencil.";
    println!("1.查找 'T'，应该是1，实际是：{:?}", tj_strchr(Some(s), 'T'));
    println!("2.查找 'i'，应该是3，实际是：{:?}", tj_strchr(Some(s), 'i'));
    println!("3.查找 ' '，应该是5，实际是：{:?}", tj_strchr(Some(s), ' '));
    println!(
        "4.查找 'x'，应该是None，实际是：{:?}",
        tj_strchr(Some(s), 'x')
    );
    println!(
        "5.None中查找，应该是None，实际是：{:?}",
        tj_strchr(None, 'a')
    );
    println!();

    // tj_strstr() 测试
    println!("tj_strstr()测试部分：");
    let s = "This is a pencil.";
    println!(
        "1.查找 'T'，应该是1，实际是：{:?}",
        tj_strstr(Some(s), Some("T"))
    );
    println!(
        "2.查找 'is'，应该是3，实际是：{:?}",
        tj_strstr(Some(s), Some("is"))
    );
    println!(
        "3.查找 'pencil'，应该是11，实际是：{:?}",
        tj_strstr(Some(s), Some("pencil"))
    );
    println!(
        "4.查找 'Pencil'，应该是None，实际是：{:?}",
        tj_strstr(Some(s), Some("Pencil"))
    );
    println!(
        "5.查找 None，应该是None，实际是：{:?}",
        tj_strstr(Some(s), None)
    );
    println!();

    // tj_strrchr() 测试
    println!("tj_strrchr()测试部分：");
    let s = "This is a pencil.";
    println!(
        "1.反向查找 'T'，应该是1，实际是：{:?}",
        tj_strrchr(Some(s), 'T')
    );
    println!(
        "2.反向查找 'i'，应该是15，实际是：{:?}",
        tj_strrchr(Some(s), 'i')
    );
    println!(
        "3.反向查找 ' '，应该是10，实际是：{:?}",
        tj_strrchr(Some(s), ' ')
    );
    println!(
        "4.反向查找 'x'，应该是None，实际是：{:?}",
        tj_strrchr(Some(s), 'x')
    );
    println!();

    // tj_strrstr() 测试
    println!("tj_strrstr()测试部分：");
    let s = "This is a pencil.";
    println!(
        "1.反向查找 'T'，应该是1，实际是：{:?}",
        tj_strrstr(Some(s), Some("T"))
    );
    println!(
        "2.反向查找 'is'，应该是6，实际是：{:?}",
        tj_strrstr(Some(s), Some("is"))
    );
    println!(
        "3.反向查找 'pencil'，应该是11，实际是：{:?}",
        tj_strrstr(Some(s), Some("pencil"))
    );

    let s2 = "aabbbceddbbbceeeff";
    println!(
        "4.在 '{}' 中反向查找 'bb'，应该是11，实际是：{:?}",
        s2,
        tj_strrstr(Some(s2), Some("bb"))
    );
    println!(
        "5.在 '{}' 中反向查找 'bbb'，应该是10，实际是：{:?}",
        s2,
        tj_strrstr(Some(s2), Some("bbb"))
    );
    println!();

    // tj_strrev() 测试
    println!("tj_strrev()测试部分：");
    println!(
        "1.'This is a pencil.' 反转：{}",
        tj_strrev(Some("This is a pencil.")).unwrap()
    );
    println!(
        "2.'aabbbceddbbbceeeff' 反转：{}",
        tj_strrev(Some("aabbbceddbbbceeeff")).unwrap()
    );
    println!("3.空串反转：-{}-", tj_strrev(Some("")).unwrap());
    println!("4.None反转：{:?}", tj_strrev(None));

    println!("\n所有测试完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tj_strlen() {
        assert_eq!(tj_strlen(Some("hello")), 5);
        assert_eq!(tj_strlen(Some("")), 0);
        assert_eq!(tj_strlen(None), 0);
        assert_eq!(tj_strlen(Some("abcdefghijklmnopqrstuvwxyz")), 26);
    }

    #[test]
    fn test_tj_strcat() {
        assert_eq!(tj_strcat("hello", Some(" world")), "hello world");
        assert_eq!(tj_strcat("hello", Some("")), "hello");
        assert_eq!(tj_strcat("hello", None), "hello");
        assert_eq!(tj_strcat("", Some("world")), "world");
    }

    #[test]
    fn test_tj_strncat() {
        assert_eq!(tj_strncat("hello", Some(" world"), 3), "hello wo");
        assert_eq!(tj_strncat("hello", Some(" world"), 100), "hello world");
        assert_eq!(tj_strncat("hello", Some(""), 5), "hello");
        assert_eq!(tj_strncat("hello", None, 5), "hello");
    }

    #[test]
    fn test_tj_strcpy() {
        assert_eq!(tj_strcpy(Some("hello")), "hello");
        assert_eq!(tj_strcpy(Some("")), "");
        assert_eq!(tj_strcpy(None), "");
    }

    #[test]
    fn test_tj_strncpy() {
        assert_eq!(tj_strncpy(Some("hello"), 3), "hel");
        assert_eq!(tj_strncpy(Some("hello"), 10), "hello");
        assert_eq!(tj_strncpy(Some("hello"), 0), "");
        assert_eq!(tj_strncpy(None, 5), "");
    }

    #[test]
    fn test_tj_strcmp() {
        assert_eq!(tj_strcmp(Some("hello"), Some("hello")), Ordering::Equal);
        assert_eq!(tj_strcmp(Some("hello"), Some("world")), Ordering::Less);
        assert_eq!(tj_strcmp(Some("world"), Some("hello")), Ordering::Greater);
        assert_eq!(tj_strcmp(None, Some("hello")), Ordering::Less);
        assert_eq!(tj_strcmp(Some("hello"), None), Ordering::Greater);
        assert_eq!(tj_strcmp(None, None), Ordering::Equal);
    }

    #[test]
    fn test_tj_strcasecmp() {
        assert_eq!(tj_strcasecmp(Some("HELLO"), Some("hello")), Ordering::Equal);
        assert_eq!(tj_strcasecmp(Some("HeLLo"), Some("hEllO")), Ordering::Equal);
        assert_eq!(tj_strcasecmp(Some("hello"), Some("world")), Ordering::Less);
        assert_eq!(
            tj_strcasecmp(Some("WORLD"), Some("hello")),
            Ordering::Greater
        );
    }

    #[test]
    fn test_tj_strncmp() {
        assert_eq!(tj_strncmp(Some("hello"), Some("help"), 3), Ordering::Equal);
        assert_eq!(tj_strncmp(Some("hello"), Some("help"), 4), Ordering::Less); // 'l' < 'p'
        assert_eq!(tj_strncmp(Some("hell"), Some("hello"), 4), Ordering::Equal);
        assert_eq!(tj_strncmp(Some("hell"), Some("hello"), 5), Ordering::Less);
        assert_eq!(
            tj_strncmp(Some("help"), Some("hello"), 4),
            Ordering::Greater
        ); // 'p' > 'l'
    }

    #[test]
    fn test_tj_strcasencmp() {
        assert_eq!(
            tj_strcasencmp(Some("HELLO"), Some("hello"), 5),
            Ordering::Equal
        );
        assert_eq!(
            tj_strcasencmp(Some("HELLO"), Some("HELP"), 3),
            Ordering::Equal
        );
        assert_eq!(
            tj_strcasencmp(Some("HELLO"), Some("HELP"), 4),
            Ordering::Less
        ); // 'L' < 'P'
        assert_eq!(
            tj_strcasencmp(Some("HELP"), Some("HELLO"), 4),
            Ordering::Greater
        ); // 'P' > 'L'
    }

    #[test]
    fn test_tj_strupr() {
        assert_eq!(tj_strupr(Some("hello")), Some("HELLO".to_string()));
        assert_eq!(tj_strupr(Some("HeLLo")), Some("HELLO".to_string()));
        assert_eq!(tj_strupr(Some("123abc")), Some("123ABC".to_string()));
        assert_eq!(tj_strupr(None), None);
    }

    #[test]
    fn test_tj_strlwr() {
        assert_eq!(tj_strlwr(Some("HELLO")), Some("hello".to_string()));
        assert_eq!(tj_strlwr(Some("HeLLo")), Some("hello".to_string()));
        assert_eq!(tj_strlwr(Some("123ABC")), Some("123abc".to_string()));
        assert_eq!(tj_strlwr(None), None);
    }

    #[test]
    fn test_tj_strchr() {
        assert_eq!(tj_strchr(Some("hello"), 'h'), Some(1));
        assert_eq!(tj_strchr(Some("hello"), 'l'), Some(3));
        assert_eq!(tj_strchr(Some("hello"), 'o'), Some(5));
        assert_eq!(tj_strchr(Some("hello"), 'x'), None);
        assert_eq!(tj_strchr(None, 'a'), None);
    }

    #[test]
    fn test_tj_strstr() {
        assert_eq!(tj_strstr(Some("hello world"), Some("world")), Some(7));
        assert_eq!(tj_strstr(Some("hello world"), Some("hello")), Some(1));
        assert_eq!(tj_strstr(Some("hello world"), Some("xyz")), None);
        assert_eq!(tj_strstr(Some("hello"), None), None);
        assert_eq!(tj_strstr(None, Some("hello")), None);
    }

    #[test]
    fn test_tj_strrchr() {
        assert_eq!(tj_strrchr(Some("hello"), 'l'), Some(4));
        assert_eq!(tj_strrchr(Some("hello"), 'h'), Some(1));
        assert_eq!(tj_strrchr(Some("hello"), 'x'), None);
        assert_eq!(tj_strrchr(None, 'a'), None);
    }

    #[test]
    fn test_tj_strrstr() {
        assert_eq!(tj_strrstr(Some("hello hello"), Some("hello")), Some(7));
        assert_eq!(
            tj_strrstr(Some("aabbbceddbbbceeeff"), Some("bbb")),
            Some(10)
        );
        assert_eq!(tj_strrstr(Some("hello world"), Some("xyz")), None);
        assert_eq!(tj_strrstr(Some("hello"), None), None);
        assert_eq!(tj_strrstr(None, Some("hello")), None);
    }

    #[test]
    fn test_tj_strrev() {
        assert_eq!(tj_strrev(Some("hello")), Some("olleh".to_string()));
        assert_eq!(tj_strrev(Some("12345")), Some("54321".to_string()));
        assert_eq!(tj_strrev(Some("")), Some("".to_string()));
        assert_eq!(tj_strrev(None), None);
    }
}
