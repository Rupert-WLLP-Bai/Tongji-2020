// 4-b15: Quadratic equation solver
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用enum表示方程的解，类型安全且避免多个输出函数
// 2. 使用match表达式替代if-else链，更清晰的控制流
// 3. 使用const EPSILON替代魔数1e-6，提高可读性
// 4. 提取核心求解逻辑为纯函数，便于单元测试
// 5. 使用Display trait实现优雅的输出格式化
// 6. 避免重复计算判别式delta

use std::fmt;
use std::io::{self, Write};

/// 浮点数比较的精度阈值
const EPSILON: f64 = 1e-6;

/// 一元二次方程的解
#[derive(Debug, PartialEq)]
enum QuadraticSolution {
    /// 不是二次方程（a=0）
    NotQuadratic,
    /// 两个相等的实根
    EqualRealRoots(f64),
    /// 两个不等的实根
    DistinctRealRoots(f64, f64),
    /// 两个共轭复根
    ComplexRoots { real: f64, imag: f64 },
}

impl fmt::Display for QuadraticSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuadraticSolution::NotQuadratic => {
                write!(f, "不是一元二次方程")
            }
            QuadraticSolution::EqualRealRoots(x) => {
                write!(f, "有两个相等实根:\nx1=x2={}", x)
            }
            QuadraticSolution::DistinctRealRoots(x1, x2) => {
                write!(f, "有两个不等实根:\nx1={}\nx2={}", x1, x2)
            }
            QuadraticSolution::ComplexRoots { real, imag } => {
                // Rust改进: 使用format!宏和条件表达式简化复数格式化
                let format_complex = |r: f64, i: f64| -> String {
                    if r.abs() < EPSILON {
                        // 无实部
                        if (i.abs() - 1.0).abs() < EPSILON {
                            "i".to_string()
                        } else {
                            format!("{}i", i)
                        }
                    } else {
                        // 有实部
                        if (i.abs() - 1.0).abs() < EPSILON {
                            format!("{}+i", r)
                        } else if i > 0.0 {
                            format!("{}+{}i", r, i)
                        } else {
                            format!("{}{}i", r, i)
                        }
                    }
                };

                write!(
                    f,
                    "有两个虚根:\nx1={}\nx2={}",
                    format_complex(*real, *imag),
                    format_complex(*real, -*imag)
                )
            }
        }
    }
}

/// 求解一元二次方程 ax² + bx + c = 0
///
/// # Arguments
/// * `a` - 二次项系数
/// * `b` - 一次项系数
/// * `c` - 常数项
///
/// # Returns
/// 方程的解类型
///
/// # Examples
/// ```
/// # use tier1_projects::*;
/// // 两个不等实根: x² - 5x + 6 = 0
/// let solution = solve_quadratic(1.0, -5.0, 6.0);
/// // x1 = 3, x2 = 2
/// ```
fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticSolution {
    // Rust改进: 使用match表达式替代多层if-else，更清晰
    if a.abs() < EPSILON {
        // 不是二次方程
        QuadraticSolution::NotQuadratic
    } else {
        // 计算判别式
        let delta = b * b - 4.0 * a * c;

        match delta {
            d if d < 0.0 => {
                // 两个共轭复根
                let real = -b / (2.0 * a);
                let imag = (-d).sqrt() / (2.0 * a);
                QuadraticSolution::ComplexRoots { real, imag }
            }
            d if d.abs() < EPSILON => {
                // 两个相等实根
                let x = -b / (2.0 * a);
                QuadraticSolution::EqualRealRoots(x)
            }
            d => {
                // 两个不等实根
                let sqrt_delta = d.sqrt();
                let x1 = (-b + sqrt_delta) / (2.0 * a);
                let x2 = (-b - sqrt_delta) / (2.0 * a);
                QuadraticSolution::DistinctRealRoots(x1, x2)
            }
        }
    }
}

fn main() {
    println!("请输入一元二次方程的三个系数a,b,c");
    print!("");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Rust改进: 使用迭代器和collect简化输入解析
    let coefficients: Vec<f64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if coefficients.len() != 3 {
        eprintln!("错误: 需要输入三个数字");
        return;
    }

    let (a, b, c) = (coefficients[0], coefficients[1], coefficients[2]);
    let solution = solve_quadratic(a, b, c);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_quadratic() {
        // a = 0，不是二次方程
        assert_eq!(
            solve_quadratic(0.0, 2.0, 3.0),
            QuadraticSolution::NotQuadratic
        );
        assert_eq!(
            solve_quadratic(1e-7, 2.0, 3.0),
            QuadraticSolution::NotQuadratic
        );
    }

    #[test]
    fn test_equal_real_roots() {
        // x² - 2x + 1 = 0, (x-1)² = 0, x1=x2=1
        let solution = solve_quadratic(1.0, -2.0, 1.0);
        assert_eq!(solution, QuadraticSolution::EqualRealRoots(1.0));

        // 4x² - 4x + 1 = 0, (2x-1)² = 0, x1=x2=0.5
        let solution = solve_quadratic(4.0, -4.0, 1.0);
        assert_eq!(solution, QuadraticSolution::EqualRealRoots(0.5));
    }

    #[test]
    fn test_distinct_real_roots() {
        // x² - 5x + 6 = 0, (x-2)(x-3) = 0, x1=3, x2=2
        let solution = solve_quadratic(1.0, -5.0, 6.0);
        match solution {
            QuadraticSolution::DistinctRealRoots(x1, x2) => {
                assert!((x1 - 3.0).abs() < EPSILON);
                assert!((x2 - 2.0).abs() < EPSILON);
            }
            _ => panic!("Expected distinct real roots"),
        }

        // x² - 1 = 0, x1=1, x2=-1
        let solution = solve_quadratic(1.0, 0.0, -1.0);
        match solution {
            QuadraticSolution::DistinctRealRoots(x1, x2) => {
                assert!((x1 - 1.0).abs() < EPSILON);
                assert!((x2 + 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected distinct real roots"),
        }
    }

    #[test]
    fn test_complex_roots() {
        // x² + 1 = 0, x = ±i
        let solution = solve_quadratic(1.0, 0.0, 1.0);
        match solution {
            QuadraticSolution::ComplexRoots { real, imag } => {
                assert!(real.abs() < EPSILON);
                assert!((imag - 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected complex roots"),
        }

        // x² + 2x + 2 = 0, x = -1 ± i
        let solution = solve_quadratic(1.0, 2.0, 2.0);
        match solution {
            QuadraticSolution::ComplexRoots { real, imag } => {
                assert!((real + 1.0).abs() < EPSILON);
                assert!((imag - 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected complex roots"),
        }

        // x² - 2x + 5 = 0, x = 1 ± 2i
        let solution = solve_quadratic(1.0, -2.0, 5.0);
        match solution {
            QuadraticSolution::ComplexRoots { real, imag } => {
                assert!((real - 1.0).abs() < EPSILON);
                assert!((imag - 2.0).abs() < EPSILON);
            }
            _ => panic!("Expected complex roots"),
        }
    }

    #[test]
    fn test_negative_coefficient() {
        // -x² + 4x - 3 = 0, x1=3, x2=1
        let solution = solve_quadratic(-1.0, 4.0, -3.0);
        match solution {
            QuadraticSolution::DistinctRealRoots(x1, x2) => {
                assert!((x1 - 1.0).abs() < EPSILON);
                assert!((x2 - 3.0).abs() < EPSILON);
            }
            _ => panic!("Expected distinct real roots"),
        }
    }

    #[test]
    fn test_display_format() {
        // 测试输出格式
        let solution = QuadraticSolution::NotQuadratic;
        assert_eq!(format!("{}", solution), "不是一元二次方程");

        let solution = QuadraticSolution::EqualRealRoots(1.5);
        assert!(format!("{}", solution).contains("有两个相等实根"));
        assert!(format!("{}", solution).contains("x1=x2=1.5"));

        let solution = QuadraticSolution::DistinctRealRoots(3.0, 2.0);
        let output = format!("{}", solution);
        assert!(output.contains("有两个不等实根"));
        assert!(output.contains("x1=3"));
        assert!(output.contains("x2=2"));
    }
}
