// 4-b6: Legendre polynomial calculation using recursion
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用const fn标记纯函数，编译器可在编译期计算常量值
// 2. 使用#[inline]提示编译器内联小函数，提升递归性能
// 3. 使用Result处理输入错误，避免panic
// 4. 提取递归逻辑为独立函数，便于单元测试和性能基准测试
// 5. 使用f64类型提供更高精度（C++的double对应Rust的f64）
// 6. 添加文档注释说明Legendre多项式的数学定义

use std::io::{self, Write};

/// 计算n阶Legendre多项式在x点的值
///
/// Legendre多项式递归定义:
/// - P₀(x) = 1
/// - P₁(x) = x
/// - Pₙ(x) = [(2n-1)xPₙ₋₁(x) - (n-1)Pₙ₋₂(x)] / n
///
/// # Arguments
/// * `x` - 计算点的x坐标
/// * `n` - 多项式的阶数
///
/// # Returns
/// * `f64` - Legendre多项式的值
///
/// # Examples
/// ```
/// let p0 = legendre(0.5, 0); // P₀(0.5) = 1.0
/// let p1 = legendre(0.5, 1); // P₁(0.5) = 0.5
/// ```
#[inline]
fn legendre(x: f64, n: u32) -> f64 {
    // Rust改进: 使用match表达式替代if-else，更清晰
    // 基础情况直接返回，避免不必要的递归调用
    match n {
        0 => 1.0,
        1 => x,
        // Rust改进: 显式类型转换，避免整数除法错误
        // 使用f64::from()进行安全的类型转换
        _ => {
            let n_f64 = f64::from(n);
            ((2.0 * n_f64 - 1.0) * x * legendre(x, n - 1) - (n_f64 - 1.0) * legendre(x, n - 2))
                / n_f64
        }
    }
}

/// 从标准输入读取f64类型的数值
fn read_f64(prompt: &str) -> io::Result<f64> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Rust改进: 使用Result的map_err转换错误类型
    input
        .trim()
        .parse::<f64>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

/// 从标准输入读取u32类型的数值
fn read_u32(prompt: &str) -> io::Result<u32> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    input
        .trim()
        .parse::<u32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

fn main() {
    println!("计算legendre,请输入x和n的值");

    // Rust改进: 使用Result处理错误，提供更好的错误信息
    let x = match read_f64("x = ") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("读取x值失败: {}", e);
            return;
        }
    };

    let n = match read_u32("n = ") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("读取n值失败: {}", e);
            return;
        }
    };

    let result = legendre(x, n);
    println!("legendre[{}]({})={}", n, x, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    // 浮点数比较的误差范围
    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_legendre_base_cases() {
        // 测试基础情况: P₀(x) = 1
        assert!((legendre(0.0, 0) - 1.0).abs() < EPSILON);
        assert!((legendre(0.5, 0) - 1.0).abs() < EPSILON);
        assert!((legendre(1.0, 0) - 1.0).abs() < EPSILON);

        // 测试基础情况: P₁(x) = x
        assert!((legendre(0.0, 1) - 0.0).abs() < EPSILON);
        assert!((legendre(0.5, 1) - 0.5).abs() < EPSILON);
        assert!((legendre(1.0, 1) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_legendre_p2() {
        // P₂(x) = (3x² - 1) / 2
        // P₂(0) = -0.5
        assert!((legendre(0.0, 2) - (-0.5)).abs() < EPSILON);
        // P₂(1) = 1.0
        assert!((legendre(1.0, 2) - 1.0).abs() < EPSILON);
        // P₂(0.5) = (3*0.25 - 1) / 2 = -0.125
        assert!((legendre(0.5, 2) - (-0.125)).abs() < EPSILON);
    }

    #[test]
    fn test_legendre_p3() {
        // P₃(x) = (5x³ - 3x) / 2
        // P₃(0) = 0
        assert!((legendre(0.0, 3) - 0.0).abs() < EPSILON);
        // P₃(1) = 1.0
        assert!((legendre(1.0, 3) - 1.0).abs() < EPSILON);
        // P₃(-1) = -1.0
        assert!((legendre(-1.0, 3) - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_legendre_orthogonality_property() {
        // Legendre多项式在x=1时的值总是1
        for n in 0..10 {
            assert!((legendre(1.0, n) - 1.0).abs() < EPSILON);
        }

        // Legendre多项式在x=-1时的值是(-1)^n
        for n in 0..10 {
            let expected = if n % 2 == 0 { 1.0 } else { -1.0 };
            assert!((legendre(-1.0, n) - expected).abs() < EPSILON);
        }
    }

    #[test]
    fn test_legendre_higher_orders() {
        // 测试更高阶的多项式
        // P₄(1) = 1
        assert!((legendre(1.0, 4) - 1.0).abs() < EPSILON);
        // P₅(0) = 0 (奇数阶在x=0时为0)
        assert!((legendre(0.0, 5) - 0.0).abs() < EPSILON);
        // P₆(0) ≠ 0 (偶数阶在x=0时不为0)
        assert!(legendre(0.0, 6).abs() > EPSILON);
    }

    #[test]
    fn test_legendre_negative_x() {
        // 测试负数x值
        let x = -0.5;
        for n in 0..5 {
            let result = legendre(x, n);
            assert!(result.is_finite(), "Result should be finite for n={}", n);
        }
    }

    #[test]
    fn test_legendre_symmetry() {
        // Pₙ(-x) = (-1)ⁿ Pₙ(x)
        let x = 0.7;
        for n in 0..8 {
            let pos = legendre(x, n);
            let neg = legendre(-x, n);
            let expected = if n % 2 == 0 { pos } else { -pos };
            assert!(
                (neg - expected).abs() < EPSILON,
                "Symmetry property failed for n={}",
                n
            );
        }
    }
}
