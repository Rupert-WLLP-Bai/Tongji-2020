// 5-b18: Password validator
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用结构体PasswordRequirements封装验证规则
// 2. 使用迭代器和filter统计字符类型，避免手动循环
// 3. 使用Result<bool, String>处理验证逻辑，错误信息更清晰
// 4. 提取字符分类函数，提高代码可读性
// 5. 使用Vec<String>代替固定数组，更灵活
// 6. 分离输入读取和验证逻辑，便于单元测试
// 7. 使用chars()迭代器处理字符，支持Unicode

use std::io::{self, BufRead};

/// 密码要求配置
#[derive(Debug, Clone, PartialEq)]
struct PasswordRequirements {
    length: usize,
    min_uppercase: usize,
    min_lowercase: usize,
    min_digits: usize,
    min_special: usize,
}

impl PasswordRequirements {
    /// 验证要求参数是否有效
    fn validate(&self) -> Result<(), String> {
        if !(12..=16).contains(&self.length) {
            return Err(format!("密码长度[{}]不在12-16范围内", self.length));
        }

        if self.min_uppercase < 2 {
            return Err(format!("大写字母最小个数[{}]小于2", self.min_uppercase));
        }

        if self.min_lowercase < 2 {
            return Err(format!("小写字母最小个数[{}]小于2", self.min_lowercase));
        }

        if self.min_digits < 2 {
            return Err(format!("数字最小个数[{}]小于2", self.min_digits));
        }

        if self.min_special < 2 {
            return Err(format!("特殊字符最小个数[{}]小于2", self.min_special));
        }

        Ok(())
    }
}

/// 字符统计结果
#[derive(Debug, PartialEq)]
struct CharacterStats {
    length: usize,
    uppercase: usize,
    lowercase: usize,
    digits: usize,
    special: usize,
}

/// 统计密码中各类字符的数量
///
/// # Arguments
/// * `password` - 待统计的密码字符串
///
/// # Returns
/// * `Ok(CharacterStats)` - 统计结果
/// * `Err(String)` - 如果包含非法字符（非可打印ASCII）
///
/// Rust改进: 使用迭代器和filter，函数式编程风格
fn count_characters(password: &str) -> Result<CharacterStats, String> {
    // 检查是否包含非法字符（非可打印ASCII字符）
    for ch in password.chars() {
        if !ch.is_ascii() || ch.is_ascii_control() || (ch as u8) < 33 || (ch as u8) > 126 {
            return Err(format!("密码包含非法字符: {:?}", ch));
        }
    }

    let uppercase = password.chars().filter(|c| c.is_ascii_uppercase()).count();
    let lowercase = password.chars().filter(|c| c.is_ascii_lowercase()).count();
    let digits = password.chars().filter(|c| c.is_ascii_digit()).count();
    let special = password
        .chars()
        .filter(|c| !c.is_ascii_alphanumeric())
        .count();

    Ok(CharacterStats {
        length: password.len(),
        uppercase,
        lowercase,
        digits,
        special,
    })
}

/// 验证单个密码是否符合要求
///
/// # Arguments
/// * `password` - 待验证的密码
/// * `requirements` - 密码要求
///
/// # Returns
/// * `Ok(true)` - 密码符合要求
/// * `Ok(false)` - 密码不符合要求
/// * `Err(String)` - 验证过程出错
pub fn validate_password(
    password: &str,
    requirements: &PasswordRequirements,
) -> Result<bool, String> {
    let stats = count_characters(password)?;

    // 检查长度
    if stats.length != requirements.length {
        return Ok(false);
    }

    // 检查各类字符数量是否满足最小要求
    if stats.uppercase < requirements.min_uppercase {
        return Ok(false);
    }

    if stats.lowercase < requirements.min_lowercase {
        return Ok(false);
    }

    if stats.digits < requirements.min_digits {
        return Ok(false);
    }

    if stats.special < requirements.min_special {
        return Ok(false);
    }

    Ok(true)
}

/// 验证多个密码是否都符合要求
///
/// # Arguments
/// * `passwords` - 密码列表
/// * `requirements` - 密码要求
///
/// # Returns
/// * `Ok(true)` - 所有密码都符合要求
/// * `Ok(false)` - 至少有一个密码不符合要求
/// * `Err(String)` - 验证过程出错
pub fn validate_all_passwords(
    passwords: &[String],
    requirements: &PasswordRequirements,
) -> Result<bool, String> {
    for password in passwords {
        if !validate_password(password, requirements)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// 从标准输入读取密码验证数据
///
/// # Returns
/// * `Ok((PasswordRequirements, Vec<String>))` - 要求和密码列表
/// * `Err(String)` - 读取或解析失败
fn read_input() -> Result<(PasswordRequirements, Vec<String>), String> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 读取并丢弃第一行提示信息
    lines
        .next()
        .ok_or("无法读取第一行")?
        .map_err(|e| format!("读取第一行失败: {}", e))?;

    // 读取第二行：5个参数
    let params_line = lines
        .next()
        .ok_or("无法读取参数行")?
        .map_err(|e| format!("读取参数行失败: {}", e))?;

    let params: Vec<usize> = params_line
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "参数解析失败")?;

    if params.len() != 5 {
        return Err(format!("参数数量错误，期望5个，实际{}", params.len()));
    }

    let requirements = PasswordRequirements {
        length: params[0],
        min_uppercase: params[1],
        min_lowercase: params[2],
        min_digits: params[3],
        min_special: params[4],
    };

    requirements.validate()?;

    // 读取10行密码
    let mut passwords = Vec::with_capacity(10);
    for _ in 0..10 {
        let password = lines
            .next()
            .ok_or("密码行数不足10行")?
            .map_err(|e| format!("读取密码行失败: {}", e))?;
        passwords.push(password);
    }

    Ok((requirements, passwords))
}

fn main() {
    match read_input() {
        Ok((requirements, passwords)) => match validate_all_passwords(&passwords, &requirements) {
            Ok(true) => println!("正确"),
            Ok(false) => println!("错误"),
            Err(e) => println!("错误: {}", e),
        },
        Err(e) => {
            println!("错误");
            eprintln!("输入错误: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_requirements_validate_valid() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_password_requirements_validate_invalid_length() {
        let req = PasswordRequirements {
            length: 11,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_count_characters_valid() {
        let stats = count_characters("ABcd12!@").unwrap();
        assert_eq!(stats.length, 8);
        assert_eq!(stats.uppercase, 2);
        assert_eq!(stats.lowercase, 2);
        assert_eq!(stats.digits, 2);
        assert_eq!(stats.special, 2);
    }

    #[test]
    fn test_count_characters_only_uppercase() {
        let stats = count_characters("ABCDEF").unwrap();
        assert_eq!(stats.uppercase, 6);
        assert_eq!(stats.lowercase, 0);
        assert_eq!(stats.digits, 0);
        assert_eq!(stats.special, 0);
    }

    #[test]
    fn test_count_characters_only_lowercase() {
        let stats = count_characters("abcdef").unwrap();
        assert_eq!(stats.uppercase, 0);
        assert_eq!(stats.lowercase, 6);
        assert_eq!(stats.digits, 0);
        assert_eq!(stats.special, 0);
    }

    #[test]
    fn test_count_characters_only_digits() {
        let stats = count_characters("123456").unwrap();
        assert_eq!(stats.uppercase, 0);
        assert_eq!(stats.lowercase, 0);
        assert_eq!(stats.digits, 6);
        assert_eq!(stats.special, 0);
    }

    #[test]
    fn test_count_characters_only_special() {
        let stats = count_characters("!@#$%^").unwrap();
        assert_eq!(stats.uppercase, 0);
        assert_eq!(stats.lowercase, 0);
        assert_eq!(stats.digits, 0);
        assert_eq!(stats.special, 6);
    }

    #[test]
    fn test_count_characters_mixed() {
        let stats = count_characters("Aa1!Bb2@Cc3#").unwrap();
        assert_eq!(stats.length, 12);
        assert_eq!(stats.uppercase, 3);
        assert_eq!(stats.lowercase, 3);
        assert_eq!(stats.digits, 3);
        assert_eq!(stats.special, 3);
    }

    #[test]
    fn test_validate_password_valid() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "AA11aa!!bbcc";
        assert_eq!(validate_password(password, &req).unwrap(), true);
    }

    #[test]
    fn test_validate_password_wrong_length() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "AA11aa!!bb"; // 长度10，不是12
        assert_eq!(validate_password(password, &req).unwrap(), false);
    }

    #[test]
    fn test_validate_password_insufficient_uppercase() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "A111aa!!bbcc"; // 只有1个大写
        assert_eq!(validate_password(password, &req).unwrap(), false);
    }

    #[test]
    fn test_validate_password_insufficient_lowercase() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "AA11a!!!BBCC"; // 只有1个小写
        assert_eq!(validate_password(password, &req).unwrap(), false);
    }

    #[test]
    fn test_validate_password_insufficient_digits() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "AA1aaa!!bbcc"; // 只有1个数字
        assert_eq!(validate_password(password, &req).unwrap(), false);
    }

    #[test]
    fn test_validate_password_insufficient_special() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "AA11aa!bbbcc"; // 只有1个特殊字符
        assert_eq!(validate_password(password, &req).unwrap(), false);
    }

    #[test]
    fn test_validate_password_exact_minimum() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "AA11aa!!****"; // 恰好满足最小要求
        assert_eq!(validate_password(password, &req).unwrap(), true);
    }

    #[test]
    fn test_validate_password_exceeds_minimum() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let password = "AAA111aaa!!!"; // 超过最小要求
        assert_eq!(validate_password(password, &req).unwrap(), true);
    }

    #[test]
    fn test_validate_all_passwords_all_valid() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let passwords = vec![
            "AA11aa!!bbcc".to_string(),
            "BB22bb##ddee".to_string(),
            "CC33cc$$ffgg".to_string(),
        ];
        assert_eq!(validate_all_passwords(&passwords, &req).unwrap(), true);
    }

    #[test]
    fn test_validate_all_passwords_one_invalid() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let passwords = vec![
            "AA11aa!!bbcc".to_string(),
            "B22bb##ddee".to_string(), // 只有1个大写
            "CC33cc$$ffgg".to_string(),
        ];
        assert_eq!(validate_all_passwords(&passwords, &req).unwrap(), false);
    }

    #[test]
    fn test_validate_all_passwords_empty() {
        let req = PasswordRequirements {
            length: 12,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
        };
        let passwords: Vec<String> = vec![];
        assert_eq!(validate_all_passwords(&passwords, &req).unwrap(), true);
    }
}
