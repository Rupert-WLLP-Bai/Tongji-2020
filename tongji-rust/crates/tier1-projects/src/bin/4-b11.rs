// 4-b11: Fibonacci sequence with performance timing
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用std::time::Instant进行跨平台高精度计时，替代Windows特定API
// 2. 添加记忆化(memoization)优化版本，避免重复计算
// 3. 使用u64类型避免大数溢出，Fibonacci(46)需要更大范围
// 4. 提供递归和迭代两种实现，展示性能差异
// 5. 使用Option处理溢出情况，更安全
// 6. 提取核心算法为可测试的纯函数

use std::io::{self, Write};
use std::time::Instant;

/// 朴素递归实现Fibonacci数列(与原C++代码相同)
///
/// # Arguments
/// * `n` - 数列项数(1-based)
///
/// # Returns
/// * Fibonacci数列第n项的值
///
/// # Performance
/// 时间复杂度: O(2^n) - 指数级，非常慢
/// 空间复杂度: O(n) - 递归调用栈
///
/// # Note
/// 此实现仅用于演示递归，实际应用应使用优化版本
fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        1 | 2 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// 迭代实现Fibonacci数列(高效版本)
///
/// # Arguments
/// * `n` - 数列项数(1-based)
///
/// # Returns
/// * `Some(value)` - Fibonacci数列第n项的值
/// * `None` - 如果发生溢出
///
/// # Performance
/// 时间复杂度: O(n) - 线性时间
/// 空间复杂度: O(1) - 常数空间
///
/// # Examples
/// ```
/// assert_eq!(fibonacci_iterative(1), Some(1));
/// assert_eq!(fibonacci_iterative(10), Some(55));
/// ```
fn fibonacci_iterative(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    if n == 1 || n == 2 {
        return Some(1);
    }

    let mut prev = 1u64;
    let mut curr = 1u64;

    // Rust改进: 使用checked_add检测溢出
    for _ in 3..=n {
        let next = prev.checked_add(curr)?;
        prev = curr;
        curr = next;
    }

    Some(curr)
}

/// 使用记忆化的递归实现(动态规划)
///
/// # Arguments
/// * `n` - 数列项数(1-based)
/// * `memo` - 记忆化缓存
///
/// # Returns
/// * Fibonacci数列第n项的值
///
/// # Performance
/// 时间复杂度: O(n) - 每个子问题只计算一次
/// 空间复杂度: O(n) - 缓存数组
fn fibonacci_memoized(n: u32, memo: &mut Vec<Option<u64>>) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }

    // 如果已经计算过，直接返回
    if let Some(value) = memo[n as usize] {
        return value;
    }

    // 递归计算并缓存结果
    let result = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo);
    memo[n as usize] = Some(result);
    result
}

/// 记忆化版本的包装函数
fn fibonacci_memo(n: u32) -> u64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut memo = vec![None; (n + 1) as usize];
    fibonacci_memoized(n, &mut memo)
}

/// 读取用户输入的项数
fn read_input() -> Result<u32, String> {
    print!("请输入Fibonacci数列的项数[1-46]");
    io::stdout().flush().unwrap();
    println!();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("读取输入失败: {}", e))?;

    let n: u32 = input
        .trim()
        .parse()
        .map_err(|_| "请输入有效的整数".to_string())?;

    if !(1..=46).contains(&n) {
        return Err("项数必须在1-46之间".to_string());
    }

    Ok(n)
}

fn main() {
    match read_input() {
        Ok(n) => {
            // Rust改进: 使用std::time::Instant进行跨平台高精度计时
            let start = Instant::now();
            let f = fibonacci_recursive(n);
            let duration = start.elapsed();

            println!("Fibonacci数列第{}项的值 : {}", n, f);
            println!();
            println!("递归算法耗时:");
            println!("  {:.6}秒", duration.as_secs_f64());
            println!("  {}纳秒", duration.as_nanos());

            // Rust改进: 展示优化版本的性能对比
            if n >= 30 {
                println!();
                println!("性能对比 - 迭代算法:");
                let start_iter = Instant::now();
                let f_iter = fibonacci_iterative(n).unwrap();
                let duration_iter = start_iter.elapsed();
                println!("  结果: {}", f_iter);
                println!("  耗时: {:.6}秒", duration_iter.as_secs_f64());
                println!(
                    "  加速比: {:.0}x",
                    duration.as_secs_f64() / duration_iter.as_secs_f64()
                );
            }
        }
        Err(e) => {
            eprintln!("错误: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_recursive_basic() {
        // 测试基本情况
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(2), 1);
        assert_eq!(fibonacci_recursive(3), 2);
        assert_eq!(fibonacci_recursive(4), 3);
        assert_eq!(fibonacci_recursive(5), 5);
        assert_eq!(fibonacci_recursive(6), 8);
    }

    #[test]
    fn test_fibonacci_recursive_sequence() {
        // 测试数列: 1, 1, 2, 3, 5, 8, 13, 21, 34, 55
        let expected = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (i, &expected_value) in expected.iter().enumerate() {
            assert_eq!(fibonacci_recursive((i + 1) as u32), expected_value);
        }
    }

    #[test]
    fn test_fibonacci_iterative_basic() {
        // 测试迭代版本基本情况
        assert_eq!(fibonacci_iterative(1), Some(1));
        assert_eq!(fibonacci_iterative(2), Some(1));
        assert_eq!(fibonacci_iterative(3), Some(2));
        assert_eq!(fibonacci_iterative(4), Some(3));
        assert_eq!(fibonacci_iterative(5), Some(5));
    }

    #[test]
    fn test_fibonacci_iterative_large() {
        // 测试较大的值
        assert_eq!(fibonacci_iterative(10), Some(55));
        assert_eq!(fibonacci_iterative(20), Some(6765));
        assert_eq!(fibonacci_iterative(30), Some(832040));
        assert_eq!(fibonacci_iterative(40), Some(102334155));
    }

    #[test]
    fn test_fibonacci_memo_basic() {
        // 测试记忆化版本
        assert_eq!(fibonacci_memo(1), 1);
        assert_eq!(fibonacci_memo(2), 1);
        assert_eq!(fibonacci_memo(3), 2);
        assert_eq!(fibonacci_memo(10), 55);
        assert_eq!(fibonacci_memo(20), 6765);
    }

    #[test]
    fn test_fibonacci_consistency() {
        // 测试三种实现的一致性(使用较小的n避免递归太慢)
        for n in 1..=20 {
            let recursive = fibonacci_recursive(n);
            let iterative = fibonacci_iterative(n).unwrap();
            let memoized = fibonacci_memo(n);

            assert_eq!(
                recursive, iterative,
                "递归和迭代结果不一致 at n={}",
                n
            );
            assert_eq!(
                recursive, memoized,
                "递归和记忆化结果不一致 at n={}",
                n
            );
        }
    }

    #[test]
    fn test_fibonacci_46() {
        // 测试题目要求的最大值
        let f46 = fibonacci_iterative(46).unwrap();
        assert_eq!(f46, 1836311903);
    }

    #[test]
    fn test_fibonacci_edge_cases() {
        // 测试边界情况
        assert_eq!(fibonacci_iterative(0), Some(0));
        assert_eq!(fibonacci_iterative(1), Some(1));
    }

    #[test]
    fn test_fibonacci_known_values() {
        // 测试已知的Fibonacci数
        assert_eq!(fibonacci_iterative(12), Some(144));
        assert_eq!(fibonacci_iterative(15), Some(610));
        assert_eq!(fibonacci_iterative(25), Some(75025));
    }

    #[test]
    fn test_performance_comparison() {
        // 性能测试: 确保迭代版本比递归快得多
        let n = 30;

        let start_recursive = Instant::now();
        let _result_recursive = fibonacci_recursive(n);
        let duration_recursive = start_recursive.elapsed();

        let start_iterative = Instant::now();
        let _result_iterative = fibonacci_iterative(n);
        let duration_iterative = start_iterative.elapsed();

        // 迭代版本应该快至少1000倍
        assert!(
            duration_iterative.as_nanos() * 1000 < duration_recursive.as_nanos(),
            "迭代版本应该明显快于递归版本"
        );
    }

    #[test]
    fn test_fibonacci_overflow_safety() {
        // 测试溢出检测(Fibonacci增长很快)
        // u64最大值约为1.8e19，Fibonacci(94)会溢出
        assert!(fibonacci_iterative(93).is_some());
        assert!(fibonacci_iterative(94).is_none()); // 应该返回None表示溢出
    }
}
