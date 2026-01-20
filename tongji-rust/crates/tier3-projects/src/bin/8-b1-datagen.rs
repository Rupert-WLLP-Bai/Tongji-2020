// 8-b1-datagen: Random data generator for testing
// Original: 为8-b1的测试生成随机的文本
//
// Rust改进:
// 1. 使用rand crate的线程安全RNG，避免全局状态和unsafe操作
// 2. 使用std::fs::File和Write trait进行类型安全的文件操作
// 3. 使用Result<T, E>进行完整的错误处理，避免panic
// 4. 提取数据生成逻辑为纯函数，便于测试和复用
// 5. 使用Vec<u8>代替固定大小全局数组，更灵活且内存安全
// 6. 支持配置化的数据生成策略（任意字节、ASCII、数字、可见字符）
// 7. 使用迭代器和函数式编程，代码更简洁
// 8. 自动资源管理（RAII），无需手动关闭文件

use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// 数据生成策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataGenStrategy {
    /// 任意字节 (0-255)
    AnyByte,
    /// 正ASCII码 (1-127)
    PositiveAscii,
    /// 数字字符 ('0'-'9')
    Digits,
    /// 可见字符 (33-126)
    Visible,
}

impl DataGenStrategy {
    /// 根据策略生成单个字节
    ///
    /// # Arguments
    /// * `rng` - 随机数生成器
    ///
    /// # Returns
    /// 生成的字节
    fn generate_byte<R: Rng>(&self, rng: &mut R) -> u8 {
        match self {
            // Rust改进: 使用match表达式清晰表达不同策略
            DataGenStrategy::AnyByte => rng.gen::<u8>(),
            DataGenStrategy::PositiveAscii => rng.gen_range(1..=127),
            DataGenStrategy::Digits => rng.gen_range(b'0'..=b'9'),
            DataGenStrategy::Visible => rng.gen_range(33..=126),
        }
    }
}

/// 生成指定大小的随机数据
///
/// # Arguments
/// * `size` - 要生成的字节数
/// * `strategy` - 数据生成策略
///
/// # Returns
/// 包含随机数据的Vec<u8>
///
/// # Examples
/// ```
/// use rand::thread_rng;
/// let data = generate_random_data(100, DataGenStrategy::Digits);
/// assert_eq!(data.len(), 100);
/// assert!(data.iter().all(|&b| b >= b'0' && b <= b'9'));
/// ```
pub fn generate_random_data(size: usize, strategy: DataGenStrategy) -> Vec<u8> {
    // Rust改进: 使用thread_rng()获取线程本地RNG，无需全局状态
    let mut rng = rand::thread_rng();

    // Rust改进: 使用迭代器和map生成数据，函数式风格
    (0..size)
        .map(|_| strategy.generate_byte(&mut rng))
        .collect()
}

/// 将数据写入文件
///
/// # Arguments
/// * `path` - 文件路径
/// * `data` - 要写入的数据
///
/// # Returns
/// * `Ok(())` - 写入成功
/// * `Err(io::Error)` - 写入失败
///
/// # Examples
/// ```no_run
/// let data = vec![1, 2, 3, 4, 5];
/// write_data_to_file("test.bin", &data).expect("Failed to write file");
/// ```
pub fn write_data_to_file<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    // Rust改进: File实现了Drop trait，自动关闭文件
    let mut file = File::create(path)?;
    file.write_all(data)?;
    // Rust改进: 显式flush确保数据写入磁盘
    file.flush()?;
    Ok(())
}

/// 将数据写入多个文件
///
/// # Arguments
/// * `paths` - 文件路径列表
/// * `data` - 要写入的数据
///
/// # Returns
/// * `Ok(usize)` - 成功写入的文件数量
/// * `Err((usize, io::Error))` - 失败时返回已成功写入的文件数和错误
pub fn write_data_to_multiple_files<P: AsRef<Path>>(
    paths: &[P],
    data: &[u8],
) -> Result<usize, (usize, io::Error)> {
    let mut success_count = 0;

    for path in paths {
        match write_data_to_file(path, data) {
            Ok(()) => success_count += 1,
            Err(e) => return Err((success_count, e)),
        }
    }

    Ok(success_count)
}

fn main() {
    // 原始代码使用145320字节
    const DATA_SIZE: usize = 145_320;

    // Rust改进: 使用当前目录而不是硬编码路径
    let output_files = vec!["hello.docx", "hello.txt", "hello.bin"];

    println!("开始生成随机数据...");

    // Rust改进: 使用AnyByte策略匹配原始C++代码的rand()行为
    let data = generate_random_data(DATA_SIZE, DataGenStrategy::AnyByte);

    println!("数据生成完成! 大小: {} 字节", data.len());
    println!("开始写入文件...");

    // Rust改进: 使用Result处理错误，而不是exit()
    match write_data_to_multiple_files(&output_files, &data) {
        Ok(count) => {
            println!("文件写入完成! 成功写入 {} 个文件:", count);
            for file in &output_files {
                println!("  - {}", file);
            }
        }
        Err((count, e)) => {
            eprintln!("文件写入失败! 已成功写入 {} 个文件", count);
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_data_gen_strategy_any_byte() {
        // 测试任意字节策略
        let mut rng = rand::thread_rng();
        let _byte = DataGenStrategy::AnyByte.generate_byte(&mut rng);
        // 任意字节 (0-255)，u8类型本身就保证了范围
        // 只需验证函数能正常执行
    }

    #[test]
    fn test_data_gen_strategy_positive_ascii() {
        // 测试正ASCII码策略
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let byte = DataGenStrategy::PositiveAscii.generate_byte(&mut rng);
            assert!(byte >= 1 && byte <= 127);
        }
    }

    #[test]
    fn test_data_gen_strategy_digits() {
        // 测试数字字符策略
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let byte = DataGenStrategy::Digits.generate_byte(&mut rng);
            assert!(byte >= b'0' && byte <= b'9');
        }
    }

    #[test]
    fn test_data_gen_strategy_visible() {
        // 测试可见字符策略
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let byte = DataGenStrategy::Visible.generate_byte(&mut rng);
            assert!(byte >= 33 && byte <= 126);
        }
    }

    #[test]
    fn test_generate_random_data_size() {
        // 测试生成数据的大小
        let sizes = vec![0, 1, 100, 1000, 10000];
        for size in sizes {
            let data = generate_random_data(size, DataGenStrategy::AnyByte);
            assert_eq!(data.len(), size);
        }
    }

    #[test]
    fn test_generate_random_data_digits_only() {
        // 测试数字策略生成的数据
        let data = generate_random_data(1000, DataGenStrategy::Digits);
        assert_eq!(data.len(), 1000);
        assert!(data.iter().all(|&b| b >= b'0' && b <= b'9'));
    }

    #[test]
    fn test_generate_random_data_visible_only() {
        // 测试可见字符策略生成的数据
        let data = generate_random_data(1000, DataGenStrategy::Visible);
        assert_eq!(data.len(), 1000);
        assert!(data.iter().all(|&b| b >= 33 && b <= 126));
    }

    #[test]
    fn test_write_data_to_file() {
        // 测试写入文件
        let test_data = vec![1, 2, 3, 4, 5];
        let test_file = "test_write_data.bin";

        // 写入文件
        write_data_to_file(test_file, &test_data).expect("Failed to write file");

        // 读取并验证
        let mut file = File::open(test_file).expect("Failed to open file");
        let mut read_data = Vec::new();
        file.read_to_end(&mut read_data)
            .expect("Failed to read file");

        assert_eq!(read_data, test_data);

        // 清理
        fs::remove_file(test_file).expect("Failed to remove test file");
    }

    #[test]
    fn test_write_data_to_file_empty() {
        // 测试写入空数据
        let test_data = vec![];
        let test_file = "test_empty.bin";

        write_data_to_file(test_file, &test_data).expect("Failed to write empty file");

        let metadata = fs::metadata(test_file).expect("Failed to get metadata");
        assert_eq!(metadata.len(), 0);

        fs::remove_file(test_file).expect("Failed to remove test file");
    }

    #[test]
    fn test_write_data_to_multiple_files() {
        // 测试写入多个文件
        let test_data = vec![10, 20, 30, 40, 50];
        let test_files = vec!["test_multi_1.bin", "test_multi_2.bin", "test_multi_3.bin"];

        let result = write_data_to_multiple_files(&test_files, &test_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);

        // 验证每个文件
        for file in &test_files {
            let mut f = File::open(file).expect("Failed to open file");
            let mut read_data = Vec::new();
            f.read_to_end(&mut read_data).expect("Failed to read file");
            assert_eq!(read_data, test_data);
            fs::remove_file(file).expect("Failed to remove test file");
        }
    }

    #[test]
    fn test_write_data_to_multiple_files_partial_failure() {
        // 测试部分文件写入失败的情况
        let test_data = vec![1, 2, 3];
        let test_files = vec![
            "test_partial_1.bin",
            "/invalid/path/that/does/not/exist/file.bin", // 这个会失败
            "test_partial_3.bin",
        ];

        let result = write_data_to_multiple_files(&test_files, &test_data);
        assert!(result.is_err());

        if let Err((count, _)) = result {
            // 第一个文件应该成功
            assert_eq!(count, 1);
        }

        // 清理成功创建的文件
        let _ = fs::remove_file("test_partial_1.bin");
    }

    #[test]
    fn test_generate_random_data_deterministic_size() {
        // 测试生成数据的确定性大小
        let size = 145_320; // 原始代码的大小
        let data = generate_random_data(size, DataGenStrategy::AnyByte);
        assert_eq!(data.len(), size);
    }

    #[test]
    fn test_data_gen_strategy_equality() {
        // 测试策略枚举的相等性
        assert_eq!(DataGenStrategy::AnyByte, DataGenStrategy::AnyByte);
        assert_ne!(DataGenStrategy::AnyByte, DataGenStrategy::Digits);
        assert_ne!(DataGenStrategy::Visible, DataGenStrategy::PositiveAscii);
    }
}
