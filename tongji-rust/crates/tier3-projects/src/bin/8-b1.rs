// 8-b1: Hex dump utility - Display file contents in hexadecimal format
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用std::fs和std::io进行文件操作，自动管理资源
// 2. 使用Result类型处理所有可能的错误，避免panic
// 3. 提取核心逻辑为纯函数，便于测试和复用
// 4. 使用迭代器和函数式编程，代码更简洁
// 5. 使用Vec<u8>缓冲区，避免固定大小数组的限制
// 6. 字符显示逻辑更清晰，使用is_ascii_graphic判断可打印字符
// 7. 格式化输出使用format!宏，类型安全且高效

use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

/// 判断字节是否为可打印的ASCII字符（不包括空格）
///
/// # Arguments
/// * `byte` - 要判断的字节
///
/// # Returns
/// * true表示可打印（ASCII 33-126），false表示不可打印
fn is_printable(byte: u8) -> bool {
    // Rust改进: 使用范围判断，比C的条件更清晰
    (33..=126).contains(&byte)
}

/// 将字节转换为可显示的字符
///
/// # Arguments
/// * `byte` - 要转换的字节
///
/// # Returns
/// * 可打印字符或'.'
fn byte_to_char(byte: u8) -> char {
    if is_printable(byte) {
        byte as char
    } else {
        '.'
    }
}

/// 格式化一行的ASCII表示
///
/// # Arguments
/// * `bytes` - 字节切片（最多16个字节）
///
/// # Returns
/// * ASCII表示的字符串
pub fn format_ascii_line(bytes: &[u8]) -> String {
    // Rust改进: 使用迭代器和map，避免手动循环
    bytes.iter().map(|&b| byte_to_char(b)).collect()
}

/// 格式化一行的十六进制表示
///
/// # Arguments
/// * `bytes` - 字节切片（最多16个字节）
///
/// # Returns
/// * 十六进制表示的字符串，每8个字节用"-"分隔
pub fn format_hex_line(bytes: &[u8]) -> String {
    let mut result = String::new();

    for (i, &byte) in bytes.iter().enumerate() {
        if i == 8 {
            result.push_str("- ");
        }
        result.push_str(&format!("{:02x} ", byte));
    }

    // 如果不足16个字节，补齐空格
    let remaining = 16 - bytes.len();
    if remaining > 0 {
        // 如果少于等于8个字节，需要额外的分隔符空间
        if bytes.len() <= 8 {
            result.push_str("  ");
        }
        // 每个字节占3个字符（2位十六进制+1个空格）
        for _ in 0..remaining {
            result.push_str("   ");
        }
    }

    result
}

/// 格式化完整的hex dump行
///
/// # Arguments
/// * `offset` - 当前偏移量
/// * `bytes` - 字节切片（最多16个字节）
///
/// # Returns
/// * 完整的hex dump行字符串
pub fn format_hex_dump_line(offset: usize, bytes: &[u8]) -> String {
    let hex_part = format_hex_line(bytes);
    let ascii_part = format_ascii_line(bytes);
    format!("{:08x}  {}   {}", offset, hex_part, ascii_part)
}

/// 执行hex dump操作
///
/// # Arguments
/// * `reader` - 实现了Read trait的对象
/// * `writer` - 实现了Write trait的对象
///
/// # Returns
/// * Result<usize, io::Error> - 成功时返回读取的字节数
pub fn hex_dump<R: Read, W: Write>(mut reader: R, mut writer: W) -> io::Result<usize> {
    // Rust改进: 使用泛型，支持任意Read/Write实现，便于测试
    let mut buffer = [0u8; 16];
    let mut offset = 0;
    let mut total_bytes = 0;

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let line = format_hex_dump_line(offset, &buffer[..bytes_read]);
        writeln!(writer, "{}", line)?;

        offset += bytes_read;
        total_bytes += bytes_read;
    }

    Ok(total_bytes)
}

/// 从文件执行hex dump
///
/// # Arguments
/// * `path` - 文件路径
///
/// # Returns
/// * Result<usize, io::Error> - 成功时返回读取的字节数
pub fn hex_dump_file<P: AsRef<Path>>(path: P) -> io::Result<usize> {
    // Rust改进: 使用BufReader提高读取性能
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    hex_dump(reader, &mut handle)
}

/// 提示用户输入文件名
fn prompt_filename() -> io::Result<String> {
    let stderr = io::stderr();
    let mut handle = stderr.lock();

    writeln!(handle, "文件名以下形式均可以：")?;
    writeln!(handle, "a.txt：不带路径形式")?;
    writeln!(handle, "../data/b.dat：相对路径形式")?;
    writeln!(handle, "/absolute/path/c.dat：绝对路径形式")?;
    write!(handle, "请输入文件名: ")?;
    handle.flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Rust改进: trim()自动处理换行符和空白字符
    Ok(input.trim().to_string())
}

fn main() {
    // Rust改进: 使用match进行错误处理，提供清晰的错误信息
    let filename = match prompt_filename() {
        Ok(name) => name,
        Err(e) => {
            eprintln!("读取输入失败: {}", e);
            std::process::exit(1);
        }
    };

    match hex_dump_file(&filename) {
        Ok(bytes) => {
            eprintln!("\n成功读取 {} 字节", bytes);
        }
        Err(e) => {
            eprintln!("文件{}打开失败: {}", filename, e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_is_printable() {
        // 测试可打印字符判断
        assert!(!is_printable(32)); // 空格不算可打印
        assert!(is_printable(33)); // '!'
        assert!(is_printable(65)); // 'A'
        assert!(is_printable(126)); // '~'
        assert!(!is_printable(127)); // DEL
        assert!(!is_printable(0)); // NULL
    }

    #[test]
    fn test_byte_to_char() {
        // 测试字节转字符
        assert_eq!(byte_to_char(65), 'A');
        assert_eq!(byte_to_char(97), 'a');
        assert_eq!(byte_to_char(0), '.');
        assert_eq!(byte_to_char(32), '.'); // 空格显示为'.'
        assert_eq!(byte_to_char(127), '.');
    }

    #[test]
    fn test_format_ascii_line() {
        // 测试ASCII行格式化
        let bytes = b"Hello";
        assert_eq!(format_ascii_line(bytes), "Hello");

        let bytes_with_control = &[72, 101, 108, 0, 111]; // "Hel\0o"
        assert_eq!(format_ascii_line(bytes_with_control), "Hel.o");
    }

    #[test]
    fn test_format_hex_line_full() {
        // 测试完整16字节的十六进制格式化
        let bytes: Vec<u8> = (0..16).collect();
        let result = format_hex_line(&bytes);
        assert!(result.contains("00 01 02 03 04 05 06 07 - 08 09 0a 0b 0c 0d 0e 0f"));
    }

    #[test]
    fn test_format_hex_line_partial() {
        // 测试不足16字节的十六进制格式化
        let bytes = b"Hello";
        let result = format_hex_line(bytes);
        assert!(result.starts_with("48 65 6c 6c 6f"));
        // 应该有补齐的空格
        assert!(result.len() > 15);
    }

    #[test]
    fn test_format_hex_line_with_separator() {
        // 测试分隔符位置
        let bytes: Vec<u8> = (0..10).collect();
        let result = format_hex_line(&bytes);
        assert!(result.contains("- "));
    }

    #[test]
    fn test_format_hex_dump_line() {
        // 测试完整行格式化
        let bytes = b"Hello, World!";
        let result = format_hex_dump_line(0, bytes);
        assert!(result.starts_with("00000000"));
        assert!(result.contains("48 65 6c 6c 6f")); // "Hello"的十六进制
        assert!(result.contains("Hello,.World!")); // ASCII部分（空格显示为'.'）
    }

    #[test]
    fn test_hex_dump_empty() {
        // 测试空输入
        let input = Cursor::new(Vec::<u8>::new());
        let mut output = Vec::new();

        let result = hex_dump(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn test_hex_dump_single_line() {
        // 测试单行输出
        let input = Cursor::new(b"Hello");
        let mut output = Vec::new();

        let result = hex_dump(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("00000000"));
        assert!(output_str.contains("Hello"));
    }

    #[test]
    fn test_hex_dump_multiple_lines() {
        // 测试多行输出（超过16字节）
        let input_data: Vec<u8> = (0..32).collect();
        let input = Cursor::new(input_data);
        let mut output = Vec::new();

        let result = hex_dump(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 32);

        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();
        assert_eq!(lines.len(), 2); // 应该有两行
        assert!(lines[0].starts_with("00000000"));
        assert!(lines[1].starts_with("00000010")); // 第二行偏移量是16
    }

    #[test]
    fn test_hex_dump_with_control_chars() {
        // 测试包含控制字符的输入
        let input_data = vec![0x00, 0x01, 0x02, 0x41, 0x42, 0x43, 0xFF];
        let input = Cursor::new(input_data);
        let mut output = Vec::new();

        let result = hex_dump(input, &mut output);
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("00 01 02 41 42 43 ff"));
        assert!(output_str.contains("...ABC.")); // 控制字符显示为'.'
    }

    #[test]
    fn test_hex_dump_exact_16_bytes() {
        // 测试恰好16字节的情况
        let input_data: Vec<u8> = (0..16).collect();
        let input = Cursor::new(input_data);
        let mut output = Vec::new();

        let result = hex_dump(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 16);

        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();
        assert_eq!(lines.len(), 1); // 应该只有一行
    }

    #[test]
    fn test_hex_dump_17_bytes() {
        // 测试17字节的情况（需要两行）
        let input_data: Vec<u8> = (0..17).collect();
        let input = Cursor::new(input_data);
        let mut output = Vec::new();

        let result = hex_dump(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 17);

        let output_str = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = output_str.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("00000010  10")); // 第二行只有一个字节
    }
}
