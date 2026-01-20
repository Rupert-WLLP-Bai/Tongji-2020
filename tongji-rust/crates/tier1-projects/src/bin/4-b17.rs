// 4-b17: Quadratic equation solver
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用enum表示方程的不同解类型，类型安全且避免全局变量
// 2. 使用Result处理输入错误，而不是直接panic
// 3. 将复数表示为结构体Complex，提供Display trait实现格式化输出
// 4. 提取核心求解逻辑为纯函数，便于单元测试
// 5. 使用const EPSILON替代魔数1e-6，提高代码可读性
// 6. 避免重复计算，将判别式作为参数传递
// 7. 使用match表达式替代if-else链，更符合Rust习惯

use std::fmt;
use std::io::{self, Write};

/// 浮点数比较的精度阈值
const EPSILON: f64 = 1e-6;

/// 复数表示
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Rust改进: 实现Display trait，统一处理复数格式化
        // 自动处理符号、特殊值(±i)等情况
        if self.real.abs() < EPSILON {
            // 无实部
            if (self.imag - 1.0).abs() < EPSILON {
                write!(f, "i")
            } else if (self.imag + 1.0).abs() < EPSILON {
                write!(f, "-i")
            } else {
                write!(f, "{}i", self.imag)
            }
        } else {
            // 有实部
            if (self.imag - 1.0).abs() < EPSILON {
                write!(f, "{}+i", self.real)
            } else if (self.imag + 1.0).abs() < EPSILON {
                write!(f, "{}-i", self.real)
            } else if self.imag >= 0.0 {
                write!(f, "{}+{}i", self.real, self.imag)
            } else {
                write!(f, "{}{}i", self.real, self.imag)
            }
        }
    }
}

/// 方程解的类型
#[derive(Debug, PartialEq)]
enum Solution {
    /// 不是二次方程 (a=0)
    NotQuadratic,
    /// 两个复数根
    ComplexRoots(Complex, Complex),
    /// 两个不等实根
    TwoDistinctRoots(f64, f64),
    /// 两个相等实根
    TwoEqualRoots(f64),
}

/// 判断浮点数是否接近零
fn is_near_zero(x: f64) -> bool {
    x.abs() < EPSILON
}

/// 求解一元二次方程 ax² + bx + c = 0
///
/// # Arguments
/// * `a` - 二次项系数
/// * `b` - 一次项系数
/// * `c` - 常数项
///
/// # Returns
/// * `Solution` - 方程的解类型
///
/// # Examples
/// ```
/// // 两个不等实根: x² - 5x + 6 = 0
/// let sol = solve_quadratic(1.0, -5.0, 6.0);
/// // x1 = 3, x2 = 2
/// ```
fn solve_quadratic(a: f64, b: f64, c: f64) -> Solution {
    // Rust改进: 使用match表达式，清晰表达不同情况
    if is_near_zero(a) {
        return Solution::NotQuadratic;
    }

    let delta = b * b - 4.0 * a * c;

    match delta {
        d if d < 0.0 => {
            // 两个共轭复根
            // x = (-b ± √(-delta)i) / (2a)
            let real_part = -b / (2.0 * a);
            let imag_part = (-d).sqrt() / (2.0 * a);
            Solution::ComplexRoots(
                Complex::new(real_part, imag_part),
                Complex::new(real_part, -imag_part),
            )
        }
        d if is_near_zero(d) => {
            // 两个相等实根
            Solution::TwoEqualRoots(-b / (2.0 * a))
        }
        d => {
            // 两个不等实根
            let sqrt_delta = d.sqrt();
            let x1 = (-b + sqrt_delta) / (2.0 * a);
            let x2 = (-b - sqrt_delta) / (2.0 * a);
            Solution::TwoDistinctRoots(x1, x2)
        }
    }
}

/// 打印方程的解
fn print_solution(solution: &Solution) {
    match solution {
        Solution::NotQuadratic => {
            println!("不是一元二次方程");
        }
        Solution::ComplexRoots(x1, x2) => {
            println!("有两个虚根:");
            println!("x1={}", x1);
            println!("x2={}", x2);
        }
        Solution::TwoDistinctRoots(x1, x2) => {
            println!("有两个不等实根:");
            println!("x1={}", x1);
            println!("x2={}", x2);
        }
        Solution::TwoEqualRoots(x) => {
            println!("有两个相等实根:");
            println!("x1=x2={}", x);
        }
    }
}

/// 从标准输入读取三个浮点数
fn read_coefficients() -> io::Result<(f64, f64, f64)> {
    print!("请输入一元二次方程的三个系数a,b,c\n");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Rust改进: 使用迭代器和collect，简洁地解析多个数字
    let coeffs: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if coeffs.len() == 3 {
        Ok((coeffs[0], coeffs[1], coeffs[2]))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "需要输入三个数字",
        ))
    }
}

fn main() {
    match read_coefficients() {
        Ok((a, b, c)) => {
            let solution = solve_quadratic(a, b, c);
            print_solution(&solution);
        }
        Err(e) => {
            eprintln!("输入错误: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_quadratic() {
        // a = 0，不是二次方程
        assert_eq!(solve_quadratic(0.0, 2.0, 3.0), Solution::NotQuadratic);
        assert_eq!(
            solve_quadratic(1e-7, 2.0, 3.0),
            Solution::NotQuadratic
        );
    }

    #[test]
    fn test_two_equal_roots() {
        // x² - 2x + 1 = 0, (x-1)² = 0, x = 1
        assert_eq!(
            solve_quadratic(1.0, -2.0, 1.0),
            Solution::TwoEqualRoots(1.0)
        );

        // x² + 4x + 4 = 0, (x+2)² = 0, x = -2
        assert_eq!(
            solve_quadratic(1.0, 4.0, 4.0),
            Solution::TwoEqualRoots(-2.0)
        );
    }

    #[test]
    fn test_two_distinct_real_roots() {
        // x² - 5x + 6 = 0, (x-2)(x-3) = 0, x = 2 or 3
        if let Solution::TwoDistinctRoots(x1, x2) = solve_quadratic(1.0, -5.0, 6.0) {
            assert!((x1 - 3.0).abs() < EPSILON);
            assert!((x2 - 2.0).abs() < EPSILON);
        } else {
            panic!("Expected TwoDistinctRoots");
        }

        // x² - 1 = 0, x = ±1
        if let Solution::TwoDistinctRoots(x1, x2) = solve_quadratic(1.0, 0.0, -1.0) {
            assert!((x1 - 1.0).abs() < EPSILON);
            assert!((x2 + 1.0).abs() < EPSILON);
        } else {
            panic!("Expected TwoDistinctRoots");
        }
    }

    #[test]
    fn test_complex_roots() {
        // x² + 1 = 0, x = ±i
        if let Solution::ComplexRoots(x1, x2) = solve_quadratic(1.0, 0.0, 1.0) {
            assert!(is_near_zero(x1.real));
            assert!((x1.imag - 1.0).abs() < EPSILON);
            assert!(is_near_zero(x2.real));
            assert!((x2.imag + 1.0).abs() < EPSILON);
        } else {
            panic!("Expected ComplexRoots");
        }

        // x² + 2x + 2 = 0, x = -1 ± i
        if let Solution::ComplexRoots(x1, x2) = solve_quadratic(1.0, 2.0, 2.0) {
            assert!((x1.real + 1.0).abs() < EPSILON);
            assert!((x1.imag - 1.0).abs() < EPSILON);
            assert!((x2.real + 1.0).abs() < EPSILON);
            assert!((x2.imag + 1.0).abs() < EPSILON);
        } else {
            panic!("Expected ComplexRoots");
        }
    }

    #[test]
    fn test_complex_display_pure_imaginary() {
        // 纯虚数
        assert_eq!(Complex::new(0.0, 1.0).to_string(), "i");
        assert_eq!(Complex::new(0.0, -1.0).to_string(), "-i");
        assert_eq!(Complex::new(0.0, 2.0).to_string(), "2i");
        assert_eq!(Complex::new(0.0, -3.5).to_string(), "-3.5i");
    }

    #[test]
    fn test_complex_display_with_real_part() {
        // 有实部和虚部
        assert_eq!(Complex::new(1.0, 1.0).to_string(), "1+i");
        assert_eq!(Complex::new(1.0, -1.0).to_string(), "1-i");
        assert_eq!(Complex::new(-2.0, 3.0).to_string(), "-2+3i");
        assert_eq!(Complex::new(2.5, -4.5).to_string(), "2.5-4.5i");
    }

    #[test]
    fn test_is_near_zero() {
        assert!(is_near_zero(0.0));
        assert!(is_near_zero(1e-7));
        assert!(is_near_zero(-1e-7));
        assert!(!is_near_zero(1e-5));
        assert!(!is_near_zero(0.1));
    }

    #[test]
    fn test_negative_coefficient() {
        // -x² + 4x - 4 = 0, 即 x² - 4x + 4 = 0, x = 2
        assert_eq!(
            solve_quadratic(-1.0, 4.0, -4.0),
            Solution::TwoEqualRoots(2.0)
        );
    }

    #[test]
    fn test_large_coefficients() {
        // 测试大系数
        if let Solution::TwoDistinctRoots(x1, x2) = solve_quadratic(1.0, -1000.0, 1.0) {
            // x ≈ 999.999 或 0.001
            assert!(x1 > 999.0);
            assert!(x2 < 1.0 && x2 > 0.0);
        } else {
            panic!("Expected TwoDistinctRoots");
        }
    }
}
