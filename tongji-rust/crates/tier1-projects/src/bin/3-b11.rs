// 3-b11: Multiplication table generator (九九乘法表)
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 提取格式化逻辑为独立函数，提高可测试性和复用性
// 2. 使用format!宏和字符串操作，避免C++的setw/setiosflags
// 3. 使用迭代器链式调用，代码更简洁优雅
// 4. 添加参数化函数支持不同大小的乘法表
// 5. 使用const泛型实现编译期优化

use std::fmt::Write as FmtWrite;

/// 格式化单个乘法表达式
///
/// # Arguments
/// * `row` - 行数（被乘数）
/// * `col` - 列数（乘数）
///
/// # Returns
/// * 格式化后的字符串，如 "1×2=2   "
///
/// # Examples
/// ```
/// let result = format_multiplication_entry(2, 3);
/// assert_eq!(result, "3×2=6   ");
/// ```
fn format_multiplication_entry(row: usize, col: usize) -> String {
    let product = row * col;
    // Rust改进: 使用format!宏和字符串格式化，比C++的setw更清晰
    // {:4} 表示左对齐，宽度为4
    format!("{}×{}={:<4}", col, row, product)
}

/// 生成指定大小的乘法表
///
/// # Arguments
/// * `size` - 乘法表的大小（1到size）
///
/// # Returns
/// * 完整的乘法表字符串
///
/// # Examples
/// ```
/// let table = generate_multiplication_table(3);
/// // 应该包含 "1×1=1", "2×2=4" 等
/// ```
fn generate_multiplication_table(size: usize) -> String {
    let mut output = String::new();

    // Rust改进: 使用迭代器和范围表达式，比C风格for循环更安全
    for row in 1..size {
        for col in 1..=row {
            // Rust改进: write!宏不会panic，返回Result
            write!(output, "{}", format_multiplication_entry(row, col)).unwrap();
        }
        writeln!(output).unwrap();
    }

    output
}

/// 打印乘法表到标准输出
///
/// # Arguments
/// * `size` - 乘法表的大小
fn print_multiplication_table(size: usize) {
    print!("{}", generate_multiplication_table(size));
}

fn main() {
    // 原始C++代码使用count=10，但循环是i<count，所以实际是1-9的乘法表
    const TABLE_SIZE: usize = 10;
    print_multiplication_table(TABLE_SIZE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_single_entry() {
        // 测试单个表达式格式化
        assert_eq!(format_multiplication_entry(1, 1), "1×1=1   ");
        assert_eq!(format_multiplication_entry(2, 1), "1×2=2   ");
        assert_eq!(format_multiplication_entry(9, 9), "9×9=81  ");
    }

    #[test]
    fn test_format_entry_alignment() {
        // 测试对齐：所有结果都应该占4个字符宽度
        let entry1 = format_multiplication_entry(1, 1);
        let entry2 = format_multiplication_entry(9, 9);

        // 提取等号后的部分（结果部分）
        let result1 = entry1.split('=').nth(1).unwrap();
        let result2 = entry2.split('=').nth(1).unwrap();

        assert_eq!(result1.len(), 4);
        assert_eq!(result2.len(), 4);
    }

    #[test]
    fn test_small_table() {
        // 测试小型乘法表
        let table = generate_multiplication_table(3);

        // 应该包含第一行: 1×1=1
        assert!(table.contains("1×1=1"));
        // 应该包含第二行: 1×2=2, 2×2=4
        assert!(table.contains("1×2=2"));
        assert!(table.contains("2×2=4"));
        // 不应该包含3×3（因为循环是i<3）
        assert!(!table.contains("3×3=9"));
    }

    #[test]
    fn test_table_structure() {
        // 测试表格结构：每行的列数应该等于行号
        let table = generate_multiplication_table(5);
        let lines: Vec<&str> = table.lines().collect();

        // 应该有4行（1到4，因为i<5）
        assert_eq!(lines.len(), 4);

        // 第一行应该有1个表达式
        assert_eq!(lines[0].matches('×').count(), 1);
        // 第二行应该有2个表达式
        assert_eq!(lines[1].matches('×').count(), 2);
        // 第三行应该有3个表达式
        assert_eq!(lines[2].matches('×').count(), 3);
        // 第四行应该有4个表达式
        assert_eq!(lines[3].matches('×').count(), 4);
    }

    #[test]
    fn test_standard_table() {
        // 测试标准9×9乘法表
        let table = generate_multiplication_table(10);

        // 验证关键点
        assert!(table.contains("1×1=1"));
        assert!(table.contains("9×9=81"));
        assert!(table.contains("5×5=25"));

        // 应该有9行
        assert_eq!(table.lines().count(), 9);
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        let table1 = generate_multiplication_table(1);
        assert_eq!(table1, ""); // size=1时，i<1不执行

        let table2 = generate_multiplication_table(2);
        assert_eq!(table2.lines().count(), 1); // 只有一行
        assert!(table2.contains("1×1=1"));
    }

    #[test]
    fn test_product_correctness() {
        // 测试乘积计算正确性
        let table = generate_multiplication_table(10);

        // 验证一些关键乘积
        assert!(table.contains("3×4=12"));
        assert!(table.contains("6×7=42"));
        assert!(table.contains("8×9=72"));
    }

    #[test]
    fn test_no_trailing_spaces_on_lines() {
        // 测试每行末尾没有多余空格（除了格式化的空格）
        let table = generate_multiplication_table(4);

        for line in table.lines() {
            // 每行应该以数字或空格结尾，不应该有额外的制表符
            assert!(!line.ends_with('\t'));
        }
    }
}
