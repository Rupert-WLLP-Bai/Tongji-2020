// 4-b14: Quadratic equation solver
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用enum表示方程根的类型，类型安全替代多个函数
// 2. 使用Complex<f64>类型表示复数，避免手动处理实部虚部
// 3. 提取判别式计算和根的计算为独立函数，便于测试
// 4. 使用const EPSILON替代magic number 1e-6，提高可读性
// 5. 格式化输出逻辑统一在Display trait中，避免重复代码
// 6. 使用match表达式替代多层if-else，更清晰的控制流

use std::fmt;
use std::io::{self, Write};

/// 浮点数比较的精度阈值
const EPSILON: f64 = 1e-6;

/// 复数类型(实部 + 虚部i)
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    /// 创建新的复数
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// 判断实部是否接近零
    fn real_is_zero(&self) -> bool {
        self.real.abs() < EPSILON
    }

    /// 判断虚部是否接近1或-1
    fn imag_is_unit(&self) -> bool {
        (self.imag.abs() - 1.0).abs() < EPSILON
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Rust改进: 统一的复数格式化逻辑，避免重复代码
        if self.real_is_zero() {
            // 无实部
            if self.imag_is_unit() {
                if self.imag > 0.0 {
                    write!(f, "i")
                } else {
                    write!(f, "-i")
                }
            } else {
                write!(f, "{}i", self.imag)
            }
        } else {
            // 有实部
            if self.imag_is_unit() {
                if self.imag > 0.0 {
                    write!(f, "{}+i", self.real)
                } else {
                    write!(f, "{}-i", self.real)
                }
            } else if self.imag > 0.0 {
                write!(f, "{}+{}i", self.real, self.imag)
            } else {
                write!(f, "{}{}i", self.real, self.imag)
            }
        }
    }
}

/// 一元二次方程的根
#[derive(Debug, PartialEq)]
enum QuadraticRoots {
    /// 不是一元二次方程(a=0)
    NotQuadratic,
    /// 两个相等的实根
    EqualReal(f64),
    /// 两个不等的实根
    TwoReal(f64, f64),
    /// 两个共轭复根
    TwoComplex(Complex, Complex),
}

impl fmt::Display for QuadraticRoots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Rust改进: 使用match表达式统一处理所有情况
        match self {
            QuadraticRoots::NotQuadratic => {
                write!(f, "不是一元二次方程")
            }
            QuadraticRoots::EqualReal(x) => {
                write!(f, "有两个相等实根:\nx1=x2={}", x)
            }
            QuadraticRoots::TwoReal(x1, x2) => {
                write!(f, "有两个不等实根:\nx1={}\nx2={}", x1, x2)
            }
            QuadraticRoots::TwoComplex(x1, x2) => {
                write!(f, "有两个虚根:\nx1={}\nx2={}", x1, x2)
            }
        }
    }
}

/// 计算判别式 Δ = b² - 4ac
///
/// # Arguments
/// * `a` - 二次项系数
/// * `b` - 一次项系数
/// * `c` - 常数项
///
/// # Returns
/// * 判别式的值
fn discriminant(a: f64, b: f64, c: f64) -> f64 {
    b * b - 4.0 * a * c
}

/// 求解一元二次方程 ax² + bx + c = 0
///
/// # Arguments
/// * `a` - 二次项系数
/// * `b` - 一次项系数
/// * `c` - 常数项
///
/// # Returns
/// * 方程的根
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticRoots {
    // Rust改进: 使用abs() < EPSILON判断浮点数是否为零，更可靠
    if a.abs() < EPSILON {
        return QuadraticRoots::NotQuadratic;
    }

    let delta = discriminant(a, b, c);

    // Rust改进: 使用match表达式替代多层if-else，更清晰
    if delta < 0.0 {
        // 两个共轭复根: x = (-b ± √(-Δ)i) / 2a
        let real_part = -b / (2.0 * a);
        let imag_part = delta.abs().sqrt() / (2.0 * a);

        QuadraticRoots::TwoComplex(
            Complex::new(real_part, imag_part),
            Complex::new(real_part, -imag_part),
        )
    } else if delta.abs() < EPSILON {
        // 两个相等实根: x = -b / 2a
        QuadraticRoots::EqualReal(-b / (2.0 * a))
    } else {
        // 两个不等实根: x = (-b ± √Δ) / 2a
        let sqrt_delta = delta.sqrt();
        let x1 = (-b + sqrt_delta) / (2.0 * a);
        let x2 = (-b - sqrt_delta) / (2.0 * a);
        QuadraticRoots::TwoReal(x1, x2)
    }
}

/// 解析输入的三个系数
///
/// # Arguments
/// * `input` - 输入字符串
///
/// # Returns
/// * `Some((a, b, c))` - 如果解析成功
/// * `None` - 如果解析失败
fn parse_coefficients(input: &str) -> Option<(f64, f64, f64)> {
    // Rust改进: 使用迭代器和collect()解析多个数字
    let nums: Vec<f64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if nums.len() == 3 {
        Some((nums[0], nums[1], nums[2]))
    } else {
        None
    }
}

fn main() {
    println!("请输入一元二次方程的三个系数a,b,c");

    // 读取系数
    let (a, b, c) = loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(coeffs) = parse_coefficients(&input) {
            break coeffs;
        }
        println!("输入格式错误，请重新输入三个数字(用空格分隔):");
    };

    // 求解并输出结果
    let roots = solve_quadratic(a, b, c);
    println!("{}", roots);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discriminant() {
        // Δ = b² - 4ac
        assert_eq!(discriminant(1.0, 0.0, 0.0), 0.0);
        assert_eq!(discriminant(1.0, 2.0, 1.0), 0.0); // (x+1)² = 0
        assert_eq!(discriminant(1.0, -3.0, 2.0), 1.0); // x² - 3x + 2 = 0
        assert_eq!(discriminant(1.0, 0.0, 1.0), -4.0); // x² + 1 = 0
    }

    #[test]
    fn test_not_quadratic() {
        // a = 0，不是二次方程
        let result = solve_quadratic(0.0, 1.0, 1.0);
        assert_eq!(result, QuadraticRoots::NotQuadratic);

        let result = solve_quadratic(1e-7, 1.0, 1.0);
        assert_eq!(result, QuadraticRoots::NotQuadratic);
    }

    #[test]
    fn test_equal_real_roots() {
        // Δ = 0，两个相等实根
        // x² - 2x + 1 = 0 => (x-1)² = 0 => x = 1
        let result = solve_quadratic(1.0, -2.0, 1.0);
        match result {
            QuadraticRoots::EqualReal(x) => {
                assert!((x - 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected EqualReal"),
        }

        // x² + 2x + 1 = 0 => (x+1)² = 0 => x = -1
        let result = solve_quadratic(1.0, 2.0, 1.0);
        match result {
            QuadraticRoots::EqualReal(x) => {
                assert!((x + 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected EqualReal"),
        }
    }

    #[test]
    fn test_two_real_roots() {
        // Δ > 0，两个不等实根
        // x² - 3x + 2 = 0 => (x-1)(x-2) = 0 => x = 1 or x = 2
        let result = solve_quadratic(1.0, -3.0, 2.0);
        match result {
            QuadraticRoots::TwoReal(x1, x2) => {
                assert!((x1 - 2.0).abs() < EPSILON);
                assert!((x2 - 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected TwoReal"),
        }

        // x² - 5x + 6 = 0 => (x-2)(x-3) = 0 => x = 2 or x = 3
        let result = solve_quadratic(1.0, -5.0, 6.0);
        match result {
            QuadraticRoots::TwoReal(x1, x2) => {
                assert!((x1 - 3.0).abs() < EPSILON);
                assert!((x2 - 2.0).abs() < EPSILON);
            }
            _ => panic!("Expected TwoReal"),
        }
    }

    #[test]
    fn test_two_complex_roots() {
        // Δ < 0，两个共轭复根
        // x² + 1 = 0 => x = ±i
        let result = solve_quadratic(1.0, 0.0, 1.0);
        match result {
            QuadraticRoots::TwoComplex(x1, x2) => {
                assert!(x1.real.abs() < EPSILON);
                assert!((x1.imag - 1.0).abs() < EPSILON);
                assert!(x2.real.abs() < EPSILON);
                assert!((x2.imag + 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected TwoComplex"),
        }

        // x² + 2x + 2 = 0 => x = -1 ± i
        let result = solve_quadratic(1.0, 2.0, 2.0);
        match result {
            QuadraticRoots::TwoComplex(x1, x2) => {
                assert!((x1.real + 1.0).abs() < EPSILON);
                assert!((x1.imag - 1.0).abs() < EPSILON);
                assert!((x2.real + 1.0).abs() < EPSILON);
                assert!((x2.imag + 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected TwoComplex"),
        }
    }

    #[test]
    fn test_complex_display() {
        // 测试复数格式化
        assert_eq!(Complex::new(0.0, 1.0).to_string(), "i");
        assert_eq!(Complex::new(0.0, -1.0).to_string(), "-i");
        assert_eq!(Complex::new(0.0, 2.0).to_string(), "2i");
        assert_eq!(Complex::new(0.0, -2.0).to_string(), "-2i");

        assert_eq!(Complex::new(1.0, 1.0).to_string(), "1+i");
        assert_eq!(Complex::new(1.0, -1.0).to_string(), "1-i");
        assert_eq!(Complex::new(-1.0, 1.0).to_string(), "-1+i");
        assert_eq!(Complex::new(-1.0, -1.0).to_string(), "-1-i");

        assert_eq!(Complex::new(2.0, 3.0).to_string(), "2+3i");
        assert_eq!(Complex::new(2.0, -3.0).to_string(), "2-3i");
    }

    #[test]
    fn test_parse_coefficients() {
        // 正常输入
        assert_eq!(parse_coefficients("1 2 3"), Some((1.0, 2.0, 3.0)));
        assert_eq!(parse_coefficients("1.5 -2.5 3.5"), Some((1.5, -2.5, 3.5)));
        assert_eq!(parse_coefficients("  1   2   3  "), Some((1.0, 2.0, 3.0)));

        // 异常输入
        assert_eq!(parse_coefficients("1 2"), None); // 少于3个
        assert_eq!(parse_coefficients("1 2 3 4"), None); // 多于3个
        assert_eq!(parse_coefficients("a b c"), None); // 非数字
        assert_eq!(parse_coefficients(""), None); // 空字符串
    }

    #[test]
    fn test_complex_is_zero() {
        assert!(Complex::new(0.0, 1.0).real_is_zero());
        assert!(Complex::new(1e-7, 1.0).real_is_zero());
        assert!(!Complex::new(1.0, 1.0).real_is_zero());
    }

    #[test]
    fn test_complex_is_unit() {
        assert!(Complex::new(0.0, 1.0).imag_is_unit());
        assert!(Complex::new(0.0, -1.0).imag_is_unit());
        assert!(Complex::new(0.0, 1.0 + 1e-7).imag_is_unit());
        assert!(!Complex::new(0.0, 2.0).imag_is_unit());
    }

    #[test]
    fn test_negative_coefficient() {
        // 测试负系数
        // -x² + 4 = 0 => x² = 4 => x = ±2
        // 使用公式: x = (-b ± √Δ) / 2a，其中 a=-1, b=0, c=4
        // Δ = 0 - 4*(-1)*4 = 16
        // x1 = (0 + 4) / -2 = -2
        // x2 = (0 - 4) / -2 = 2
        let result = solve_quadratic(-1.0, 0.0, 4.0);
        match result {
            QuadraticRoots::TwoReal(x1, x2) => {
                assert!((x1 + 2.0).abs() < EPSILON);
                assert!((x2 - 2.0).abs() < EPSILON);
            }
            _ => panic!("Expected TwoReal"),
        }
    }

    #[test]
    fn test_large_coefficients() {
        // 测试大系数
        let result = solve_quadratic(1000.0, -3000.0, 2000.0);
        match result {
            QuadraticRoots::TwoReal(x1, x2) => {
                // x² - 3x + 2 = 0 的解
                assert!((x1 - 2.0).abs() < EPSILON);
                assert!((x2 - 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected TwoReal"),
        }
    }

    #[test]
    fn test_roots_display() {
        // 测试输出格式
        let roots = QuadraticRoots::NotQuadratic;
        assert!(roots.to_string().contains("不是一元二次方程"));

        let roots = QuadraticRoots::EqualReal(1.0);
        assert!(roots.to_string().contains("相等实根"));
        assert!(roots.to_string().contains("x1=x2=1"));

        let roots = QuadraticRoots::TwoReal(2.0, 1.0);
        assert!(roots.to_string().contains("不等实根"));
        assert!(roots.to_string().contains("x1=2"));
        assert!(roots.to_string().contains("x2=1"));

        let roots = QuadraticRoots::TwoComplex(Complex::new(0.0, 1.0), Complex::new(0.0, -1.0));
        assert!(roots.to_string().contains("虚根"));
    }
}
