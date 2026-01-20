// 5-b17: Password generator with character requirements
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用rand crate的thread_rng()，线程安全且更好的随机性
// 2. 使用Vec<char>代替固定数组，避免魔法数字
// 3. 使用Rng::gen_range()代替手动取模，分布更均匀
// 4. 使用shuffle()代替手动交换，Fisher-Yates算法更高效
// 5. 提取密码生成逻辑为独立函数，便于测试
// 6. 使用Result处理输入验证，错误处理更清晰
// 7. 使用迭代器和函数式编程，代码更简洁

use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};

/// 密码配置参数
#[derive(Debug, Clone, PartialEq)]
struct PasswordConfig {
    length: usize,
    uppercase_count: usize,
    lowercase_count: usize,
    digit_count: usize,
    special_count: usize,
}

impl PasswordConfig {
    /// 验证配置参数是否有效
    fn validate(&self) -> Result<(), String> {
        if !(12..=16).contains(&self.length) {
            return Err(format!("密码长度[{}]不正确", self.length));
        }

        if self.uppercase_count < 2 {
            return Err(format!("大写字母个数[{}]不正确", self.uppercase_count));
        }

        if self.lowercase_count < 2 {
            return Err(format!("小写字母个数[{}]不正确", self.lowercase_count));
        }

        if self.digit_count < 2 {
            return Err(format!("数字个数[{}]不正确", self.digit_count));
        }

        if self.special_count < 2 {
            return Err(format!("其他符号个数[{}]不正确", self.special_count));
        }

        let total = self.uppercase_count + self.lowercase_count + self.digit_count + self.special_count;
        if total > self.length {
            return Err(format!(
                "所有字符类型之和[{}+{}+{}+{}]大于总密码长度[{}]",
                self.uppercase_count,
                self.lowercase_count,
                self.digit_count,
                self.special_count,
                self.length
            ));
        }

        Ok(())
    }
}

/// 生成指定数量的大写字母
///
/// Rust改进: 使用thread_rng()和gen_range()，随机性更好
fn generate_uppercase(count: usize, rng: &mut impl Rng) -> Vec<char> {
    (0..count)
        .map(|_| rng.gen_range(b'A'..=b'Z') as char)
        .collect()
}

/// 生成指定数量的小写字母
fn generate_lowercase(count: usize, rng: &mut impl Rng) -> Vec<char> {
    (0..count)
        .map(|_| rng.gen_range(b'a'..=b'z') as char)
        .collect()
}

/// 生成指定数量的数字
fn generate_digits(count: usize, rng: &mut impl Rng) -> Vec<char> {
    (0..count)
        .map(|_| rng.gen_range(b'0'..=b'9') as char)
        .collect()
}

/// 生成指定数量的特殊字符（排除字母和数字）
///
/// Rust改进: 使用loop + 条件判断，逻辑更清晰
fn generate_special(count: usize, rng: &mut impl Rng) -> Vec<char> {
    let mut result = Vec::with_capacity(count);
    while result.len() < count {
        let ch = rng.gen_range(33u8..=126) as char;
        if !ch.is_ascii_alphanumeric() {
            result.push(ch);
        }
    }
    result
}

/// 生成任意可打印ASCII字符（用于填充剩余长度）
fn generate_any_printable(count: usize, rng: &mut impl Rng) -> Vec<char> {
    (0..count)
        .map(|_| rng.gen_range(33u8..=126) as char)
        .collect()
}

/// 根据配置生成一个密码
///
/// # Arguments
/// * `config` - 密码配置参数
///
/// # Returns
/// * `String` - 生成的密码
///
/// Rust改进: 使用shuffle()代替手动交换，Fisher-Yates算法O(n)
fn generate_password(config: &PasswordConfig) -> String {
    let mut rng = rand::thread_rng();
    let mut chars = Vec::with_capacity(config.length);

    // 生成各类字符
    chars.extend(generate_uppercase(config.uppercase_count, &mut rng));
    chars.extend(generate_lowercase(config.lowercase_count, &mut rng));
    chars.extend(generate_digits(config.digit_count, &mut rng));
    chars.extend(generate_special(config.special_count, &mut rng));

    // 填充剩余长度
    let remaining = config.length - chars.len();
    if remaining > 0 {
        chars.extend(generate_any_printable(remaining, &mut rng));
    }

    // Rust改进: 使用shuffle()打乱顺序，比手动交换100次更可靠
    chars.shuffle(&mut rng);

    chars.into_iter().collect()
}

/// 读取并验证整数输入
fn read_usize(prompt: &str) -> Result<usize, String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("读取输入失败: {}", e))?;

    input
        .trim()
        .parse::<usize>()
        .map_err(|_| "输入含有非法字符".to_string())
}

/// 读取密码配置
fn read_config() -> Result<PasswordConfig, String> {
    println!("输入请输入密码长度(12-16)，大写字母个数(≥2)，小写字母个数(≥2)，数字个数(≥2)，其它符号个数(≥2)");

    let length = read_usize("")?;
    let uppercase_count = read_usize("")?;
    let lowercase_count = read_usize("")?;
    let digit_count = read_usize("")?;
    let special_count = read_usize("")?;

    let config = PasswordConfig {
        length,
        uppercase_count,
        lowercase_count,
        digit_count,
        special_count,
    };

    config.validate()?;
    Ok(config)
}

fn main() {
    match read_config() {
        Ok(config) => {
            // 输出配置信息
            println!(
                "{} {} {} {} {}",
                config.length,
                config.uppercase_count,
                config.lowercase_count,
                config.digit_count,
                config.special_count
            );

            // 生成10个密码
            for _ in 0..10 {
                let password = generate_password(&config);
                println!("{}", password);
            }
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
    fn test_password_config_validate_valid() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_password_config_validate_length_too_short() {
        let config = PasswordConfig {
            length: 11,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_password_config_validate_length_too_long() {
        let config = PasswordConfig {
            length: 17,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_password_config_validate_uppercase_too_few() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 1,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_password_config_validate_total_exceeds_length() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 5,
            lowercase_count: 5,
            digit_count: 5,
            special_count: 5,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_generate_password_length() {
        let config = PasswordConfig {
            length: 14,
            uppercase_count: 3,
            lowercase_count: 3,
            digit_count: 3,
            special_count: 3,
        };
        let password = generate_password(&config);
        assert_eq!(password.len(), 14);
    }

    #[test]
    fn test_generate_password_contains_uppercase() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        let password = generate_password(&config);
        let uppercase_count = password.chars().filter(|c| c.is_ascii_uppercase()).count();
        assert!(uppercase_count >= 2);
    }

    #[test]
    fn test_generate_password_contains_lowercase() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        let password = generate_password(&config);
        let lowercase_count = password.chars().filter(|c| c.is_ascii_lowercase()).count();
        assert!(lowercase_count >= 2);
    }

    #[test]
    fn test_generate_password_contains_digits() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        let password = generate_password(&config);
        let digit_count = password.chars().filter(|c| c.is_ascii_digit()).count();
        assert!(digit_count >= 2);
    }

    #[test]
    fn test_generate_password_contains_special() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        let password = generate_password(&config);
        let special_count = password
            .chars()
            .filter(|c| !c.is_ascii_alphanumeric() && c.is_ascii())
            .count();
        assert!(special_count >= 2);
    }

    #[test]
    fn test_generate_password_all_printable() {
        let config = PasswordConfig {
            length: 16,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        let password = generate_password(&config);
        assert!(password.chars().all(|c| c.is_ascii() && !c.is_ascii_control()));
    }

    #[test]
    fn test_generate_password_uniqueness() {
        let config = PasswordConfig {
            length: 12,
            uppercase_count: 2,
            lowercase_count: 2,
            digit_count: 2,
            special_count: 2,
        };
        let password1 = generate_password(&config);
        let password2 = generate_password(&config);
        // 两次生成的密码应该不同（概率极高）
        assert_ne!(password1, password2);
    }

    #[test]
    fn test_generate_uppercase() {
        let mut rng = rand::thread_rng();
        let chars = generate_uppercase(5, &mut rng);
        assert_eq!(chars.len(), 5);
        assert!(chars.iter().all(|c| c.is_ascii_uppercase()));
    }

    #[test]
    fn test_generate_lowercase() {
        let mut rng = rand::thread_rng();
        let chars = generate_lowercase(5, &mut rng);
        assert_eq!(chars.len(), 5);
        assert!(chars.iter().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_generate_digits() {
        let mut rng = rand::thread_rng();
        let chars = generate_digits(5, &mut rng);
        assert_eq!(chars.len(), 5);
        assert!(chars.iter().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_generate_special() {
        let mut rng = rand::thread_rng();
        let chars = generate_special(5, &mut rng);
        assert_eq!(chars.len(), 5);
        assert!(chars
            .iter()
            .all(|c| !c.is_ascii_alphanumeric() && c.is_ascii()));
    }
}
