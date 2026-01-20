// 8-b2: Hex file to binary converter
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Result<T, E>进行错误处理，避免程序崩溃
// 2. 使用BufReader和BufWriter提高I/O性能
// 3. 使用char::to_digit()安全解析十六进制字符
// 4. 提取核心逻辑为纯函数，便于单元测试
// 5. 使用迭代器和函数式编程，代码更简洁
// 6. 自动资源管理，无需手动fclose
// 7. 使用&str切片避免不必要的内存分配
// 8. 类型安全的错误处理，编译时捕获错误

use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// 将两个十六进制字符转换为字节
///
/// # Arguments
/// * `ch1` - 高位十六进制字符 (0-9, a-f, A-F)
/// * `ch2` - 低位十六进制字符 (0-9, a-f, A-F)
///
/// # Returns
/// * `Some(u8)` - 成功转换的字节值
/// * `None` - 无效的十六进制字符
///
/// # Examples
/// ```
/// assert_eq!(convert_hex('a', 'b'), Some(0xab));
/// assert_eq!(convert_hex('F', 'F'), Some(0xff));
/// assert_eq!(convert_hex('0', '0'), Some(0x00));
/// assert_eq!(convert_hex('g', '0'), None);
/// ```
fn convert_hex(ch1: char, ch2: char) -> Option<u8> {
    // Rust改进: 使用char::to_digit()安全解析，自动处理大小写
    let high = ch1.to_digit(16)? as u8;
    let low = ch2.to_digit(16)? as u8;
    Some(high * 16 + low)
}

/// 从hex格式行中提取并转换字节数据
///
/// Hex文件格式说明:
/// - C代码使用getc()读取第一个字符，然后fgets()读取剩余部分
/// - Rust读取完整行，所以需要将C的位置+1
/// - C代码中line[9]对应Rust的chars[10]
/// - 每个字节用3个字符表示: 两位十六进制 + 一个空格
/// - count = strlen(line) - 64，表示剩余字符数
///
/// # Arguments
/// * `line` - hex格式的行
///
/// # Returns
/// * `Ok(Vec<u8>)` - 成功解析的字节向量
/// * `Err(String)` - 解析错误信息
fn parse_hex_line(line: &str) -> Result<Vec<u8>, String> {
    // C代码: strlen(line) - 64，但C的line是fgets()读取的（不含首字符）
    // Rust读取完整行，所以实际长度 = C的strlen(line) + 1
    // 因此: count = (rust_len - 1) - 64 = rust_len - 65
    let line_len = line.len();

    if line_len <= 65 {
        return Ok(Vec::new());
    }

    let count = line_len - 65;

    // Rust改进: 使用chars().collect()安全访问字符
    let chars: Vec<char> = line.chars().collect();

    // 需要至少12个字符才能解析第一个字节 (C的11 + 1)
    if chars.len() < 12 {
        return Ok(Vec::new());
    }

    let mut bytes = Vec::new();

    // 处理前8个字节（或全部字节，如果count <= 24）
    if count <= 24 {
        // count <= 24: 最多8个字节
        // C代码: line[9 + i*3], line[10 + i*3]
        // Rust: chars[10 + i*3], chars[11 + i*3]
        let num_bytes = count / 3;
        for i in 0..num_bytes {
            let pos = 10 + i * 3;
            if pos + 1 >= chars.len() {
                break;
            }

            if let Some(byte) = convert_hex(chars[pos], chars[pos + 1]) {
                bytes.push(byte);
            }
        }
    } else {
        // count > 24: 超过8个字节
        // 前8个字节: C的line[9 + i*3] = Rust的chars[10 + i*3]
        for i in 0..8 {
            let pos = 10 + i * 3;
            if pos + 1 >= chars.len() {
                break;
            }

            if let Some(byte) = convert_hex(chars[pos], chars[pos + 1]) {
                bytes.push(byte);
            }
        }

        // 剩余字节: C的line[11 + i*3] = Rust的chars[12 + i*3]
        let remaining_count = (count - 24) / 3;
        for i in 0..remaining_count {
            let pos = 12 + (8 + i) * 3;
            if pos + 1 >= chars.len() {
                break;
            }

            if let Some(byte) = convert_hex(chars[pos], chars[pos + 1]) {
                bytes.push(byte);
            }
        }
    }

    Ok(bytes)
}

/// 转换hex文件到二进制文件
///
/// # Arguments
/// * `input_path` - 输入hex文件路径
/// * `output_path` - 输出二进制文件路径
///
/// # Returns
/// * `Ok(usize)` - 成功转换的字节数
/// * `Err(io::Error)` - I/O错误
fn convert_hex_file<P: AsRef<Path>>(input_path: P, output_path: P) -> io::Result<usize> {
    // Rust改进: 使用BufReader和BufWriter提高性能
    let input_file = File::open(input_path.as_ref())?;
    let reader = BufReader::new(input_file);

    let output_file = File::create(output_path.as_ref())?;
    let mut writer = BufWriter::new(output_file);

    let mut total_bytes = 0;
    let mut line_number = 0;

    // Rust改进: 使用迭代器处理每一行
    for line in reader.lines() {
        line_number += 1;
        let line = line?;

        // 跳过第一个字符（原C代码中的getc(fp1)）
        if line_number == 1 && line.is_empty() {
            continue;
        }

        // 解析并写入字节
        match parse_hex_line(&line) {
            Ok(bytes) => {
                writer.write_all(&bytes)?;
                total_bytes += bytes.len();
            }
            Err(e) => {
                eprintln!("警告: 第{}行解析失败: {}", line_number, e);
                // 继续处理下一行，不中断整个转换过程
            }
        }
    }

    // Rust改进: flush确保所有数据写入磁盘
    writer.flush()?;
    Ok(total_bytes)
}

/// 读取用户输入的文件名，去除末尾换行符
fn read_filename(prompt: &str) -> io::Result<String> {
    eprint!("{}", prompt);
    io::stderr().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Rust改进: 使用trim_end()安全去除末尾空白字符
    Ok(input.trim_end().to_string())
}

fn main() {
    eprintln!("文件名以下形式均可以：");
    eprintln!("a.txt：不带路径形式");
    eprintln!("../data/b.dat：相对路径形式");
    eprintln!("C:\\Windows\\System32\\c.dat：绝对路径形式");

    // 读取输入文件名
    let input_file = match read_filename("请输入要转换的hex格式文件名: ") {
        Ok(name) => name,
        Err(e) => {
            eprintln!("读取输入失败: {}", e);
            return;
        }
    };

    // 读取输出文件名
    let output_file = match read_filename("请输入转换后的文件名: ") {
        Ok(name) => name,
        Err(e) => {
            eprintln!("读取输入失败: {}", e);
            return;
        }
    };

    // 执行转换
    match convert_hex_file(&input_file, &output_file) {
        Ok(bytes) => {
            eprintln!("转换成功! 共写入 {} 字节", bytes);
        }
        Err(e) => {
            eprintln!("文件转换失败: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_convert_hex_valid() {
        // 测试有效的十六进制转换
        assert_eq!(convert_hex('0', '0'), Some(0x00));
        assert_eq!(convert_hex('f', 'f'), Some(0xff));
        assert_eq!(convert_hex('F', 'F'), Some(0xFF));
        assert_eq!(convert_hex('a', 'b'), Some(0xab));
        assert_eq!(convert_hex('1', '2'), Some(0x12));
        assert_eq!(convert_hex('9', 'A'), Some(0x9a));
    }

    #[test]
    fn test_convert_hex_invalid() {
        // 测试无效的十六进制字符
        assert_eq!(convert_hex('g', '0'), None);
        assert_eq!(convert_hex('0', 'z'), None);
        assert_eq!(convert_hex('x', 'y'), None);
        assert_eq!(convert_hex(' ', '0'), None);
    }

    #[test]
    fn test_convert_hex_case_insensitive() {
        // 测试大小写不敏感
        assert_eq!(convert_hex('a', 'b'), convert_hex('A', 'B'));
        assert_eq!(convert_hex('f', '0'), convert_hex('F', '0'));
    }

    #[test]
    fn test_parse_hex_line_empty() {
        // 测试空行或短行
        assert_eq!(parse_hex_line("").unwrap(), Vec::<u8>::new());
        assert_eq!(parse_hex_line("short").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_parse_hex_line_single_byte() {
        // 测试单字节行
        // Rust长度 = C的strlen + 1 = 67 + 1 = 68，count = 68 - 65 = 3
        // 格式: 首字符(1) + 9个前导(9) + "ab "(3) + 填充到68
        let line = "X123456789ab                                                        ";
        assert_eq!(line.len(), 68); // 验证长度
        let result = parse_hex_line(line).unwrap();
        assert_eq!(result, vec![0xab]);
    }

    #[test]
    fn test_parse_hex_line_multiple_bytes() {
        // 测试多字节行 (count <= 24)
        // 4个字节: Rust长度 = 1 + 64 + 12 = 77，count = 77 - 65 = 12
        let line = "X12345678901 02 03 04                                                        ";
        assert_eq!(line.len(), 77);
        let result = parse_hex_line(line).unwrap();
        assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_parse_hex_line_eight_bytes() {
        // 测试8字节边界情况
        // 8个字节: Rust长度 = 1 + 64 + 24 = 89，count = 89 - 65 = 24
        let line = "X12345678900 11 22 33 44 55 66 77                                                        ";
        assert_eq!(line.len(), 89);
        let result = parse_hex_line(line).unwrap();
        assert_eq!(result, vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77]);
    }

    #[test]
    fn test_parse_hex_line_more_than_eight() {
        // 测试超过8字节的情况（位置偏移变化）
        // 11个字节: Rust长度 = 1 + 64 + 33 = 98，count = 98 - 65 = 33
        let line = "X12345678900 11 22 33 44 55 66 77   88 99 aa                                                      ";
        assert_eq!(line.len(), 98);
        let result = parse_hex_line(line).unwrap();
        assert_eq!(
            result,
            vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa]
        );
    }

    #[test]
    fn test_parse_hex_line_invalid_hex() {
        // 测试无效的十六进制字符 - 现在返回Ok但跳过无效字节
        let line = "X123456789zz                                                        ";
        assert_eq!(line.len(), 68);
        let result = parse_hex_line(line).unwrap();
        // 无效的hex字符会被跳过，返回空向量
        assert_eq!(result, Vec::<u8>::new());
    }

    #[test]
    fn test_convert_hex_file_integration() {
        // 集成测试: 创建临时hex文件并转换
        let mut input_file = NamedTempFile::new().unwrap();
        let output_file = NamedTempFile::new().unwrap();

        // 写入测试hex数据
        // "Hello" = 5 bytes, Rust长度 = 1 + 64 + 15 = 80
        let line1 =
            "X12345678948 65 6c 6c 6f                                                        ";
        assert_eq!(line1.len(), 80);
        writeln!(input_file, "{}", line1).unwrap();

        // " World" = 6 bytes, Rust长度 = 1 + 64 + 18 = 83
        let line2 =
            "X12345678920 57 6f 72 6c 64                                                        ";
        assert_eq!(line2.len(), 83);
        writeln!(input_file, "{}", line2).unwrap();
        input_file.flush().unwrap();

        // 执行转换
        let bytes_written = convert_hex_file(input_file.path(), output_file.path()).unwrap();

        // 验证结果
        assert_eq!(bytes_written, 11); // "Hello World" = 11 bytes

        // 读取输出文件验证内容
        let output_content = std::fs::read(output_file.path()).unwrap();
        assert_eq!(output_content, b"Hello World");
    }

    #[test]
    fn test_convert_hex_file_empty() {
        // 测试空文件
        let input_file = NamedTempFile::new().unwrap();
        let output_file = NamedTempFile::new().unwrap();

        let bytes_written = convert_hex_file(input_file.path(), output_file.path()).unwrap();

        assert_eq!(bytes_written, 0);
    }

    #[test]
    fn test_convert_hex_boundary_values() {
        // 测试边界值
        assert_eq!(convert_hex('0', '0'), Some(0));
        assert_eq!(convert_hex('f', 'f'), Some(255));
        assert_eq!(convert_hex('8', '0'), Some(128));
        assert_eq!(convert_hex('7', 'f'), Some(127));
    }
}
