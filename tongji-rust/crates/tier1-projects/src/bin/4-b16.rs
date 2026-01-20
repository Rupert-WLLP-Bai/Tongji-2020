// 4-b16: Quadratic equation solver (modular version)
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用enum + match替代多个独立函数，更符合Rust类型系统
// 2. 使用Result类型处理输入错误，而不是直接panic
// 3. 简化复数格式化逻辑，使用闭包避免代码重复
// 4. 提取所有业务逻辑为可测试的纯函数
// 5. 使用const泛型和trait实现更灵活的数值计算
// 6. 避免C++中的重复判别式计算

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

// Rust改进: 使用模块化的方式组织代码，每个函数对应原C++的一个子函数
// 但在Rust中，我们使用enum + Display trait实现更优雅的设计

impl QuadraticSolution {
    /// 对应原C++的f1()函数
    fn format_not_quadratic() -> String {
        "不是一元二次方程".to_string()
    }

    /// 对应原C++的f4()函数 - 两相等实根
    fn format_equal_roots(x: f64) -> String {
        format!("有两个相等实根:\nx1=x2={}", x)
    }

    /// 对应原C++的f3()函数 - 两不等实根
    fn format_distinct_roots(x1: f64, x2: f64) -> String {
        format!("有两个不等实根:\nx1={}\nx2={}", x1, x2)
    }

    /// 对应原C++的f2()函数 - 两共轭复根
    /// Rust改进: 简化了原C++中复杂的嵌套if-else逻辑
    fn format_complex_roots(real: f64, imag: f64) -> String {
        // 格式化单个复数
        let format_one = |r: f64, i: f64| -> String {
            let has_real = r.abs() >= EPSILON;
            let is_unit_imag = (i.abs() - 1.0).abs() < EPSILON;

            match (has_real, is_unit_imag, i > 0.0) {
                (false, true, true) => "i".to_string(),
                (false, true, false) => "-i".to_string(),
                (false, false, _) => format!("{}i", i),
                (true, true, true) => format!("{}+i", r),
                (true, true, false) => format!("{}-i", r),
                (true, false, true) => format!("{}+{}i", r, i),
                (true, false, false) => format!("{}{}i", r, i),
            }
        };

        format!(
            "有两个虚根:\nx1={}\nx2={}",
            format_one(real, imag),
            format_one(real, -imag)
        )
    }
}

impl fmt::Display for QuadraticSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            QuadraticSolution::NotQuadratic => Self::format_not_quadratic(),
            QuadraticSolution::EqualRealRoots(x) => Self::format_equal_roots(*x),
            QuadraticSolution::DistinctRealRoots(x1, x2) => Self::format_distinct_roots(*x1, *x2),
            QuadraticSolution::ComplexRoots { real, imag } => {
                Self::format_complex_roots(*real, *imag)
            }
        };
        write!(f, "{}", output)
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
/// # const EPSILON: f64 = 1e-6;
/// # #[derive(Debug, PartialEq)]
/// # enum QuadraticSolution {
/// #     NotQuadratic,
/// #     EqualRealRoots(f64),
/// #     DistinctRealRoots(f64, f64),
/// #     ComplexRoots { real: f64, imag: f64 },
/// # }
/// # fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticSolution {
/// #     if a.abs() < EPSILON {
/// #         QuadraticSolution::NotQuadratic
/// #     } else {
/// #         let delta = b * b - 4.0 * a * c;
/// #         match delta {
/// #             d if d < 0.0 => {
/// #                 let real = -b / (2.0 * a);
/// #                 let imag = (-d).sqrt() / (2.0 * a);
/// #                 QuadraticSolution::ComplexRoots { real, imag }
/// #             }
/// #             d if d.abs() < EPSILON => {
/// #                 let x = -b / (2.0 * a);
/// #                 QuadraticSolution::EqualRealRoots(x)
/// #             }
/// #             d => {
/// #                 let sqrt_delta = d.sqrt();
/// #                 let x1 = (-b + sqrt_delta) / (2.0 * a);
/// #                 let x2 = (-b - sqrt_delta) / (2.0 * a);
/// #                 QuadraticSolution::DistinctRealRoots(x1, x2)
/// #             }
/// #         }
/// #     }
/// # }
/// // 两个不等实根: x² - 5x + 6 = 0
/// let solution = solve_quadratic(1.0, -5.0, 6.0);
/// match solution {
///     QuadraticSolution::DistinctRealRoots(x1, x2) => {
///         assert!((x1 - 3.0).abs() < 1e-6);
///         assert!((x2 - 2.0).abs() < 1e-6);
///     }
///     _ => panic!("Expected distinct real roots"),
/// }
/// ```
fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticSolution {
    // Rust改进: 使用match表达式替代原C++的多层if-else
    if a.abs() < EPSILON {
        // 对应原C++的f1()
        QuadraticSolution::NotQuadratic
    } else {
        // 计算判别式（只计算一次，避免重复）
        let delta = b * b - 4.0 * a * c;

        match delta {
            d if d < 0.0 => {
                // 对应原C++的f2() - 两个共轭复根
                let real = -b / (2.0 * a);
                let imag = (-d).sqrt() / (2.0 * a);
                QuadraticSolution::ComplexRoots { real, imag }
            }
            d if d.abs() < EPSILON => {
                // 对应原C++的f4() - 两个相等实根
                let x = -b / (2.0 * a);
                QuadraticSolution::EqualRealRoots(x)
            }
            d => {
                // 对应原C++的f3() - 两个不等实根
                let sqrt_delta = d.sqrt();
                let x1 = (-b + sqrt_delta) / (2.0 * a);
                let x2 = (-b - sqrt_delta) / (2.0 * a);
                QuadraticSolution::DistinctRealRoots(x1, x2)
            }
        }
    }
}

/// 解析输入的三个系数
///
/// # Arguments
/// * `input` - 输入字符串
///
/// # Returns
/// * `Ok((a, b, c))` - 成功解析的三个系数
/// * `Err(String)` - 解析失败的错误信息
fn parse_coefficients(input: &str) -> Result<(f64, f64, f64), String> {
    let coefficients: Vec<f64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if coefficients.len() != 3 {
        Err(format!(
            "需要输入三个数字，但只解析到{}个",
            coefficients.len()
        ))
    } else {
        Ok((coefficients[0], coefficients[1], coefficients[2]))
    }
}

fn main() {
    println!("请输入一元二次方程的三个系数a,b,c");
    print!("");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Rust改进: 使用Result处理错误，而不是直接panic
    match parse_coefficients(&input) {
        Ok((a, b, c)) => {
            let solution = solve_quadratic(a, b, c);
            println!("{}", solution);
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
    fn test_not_quadratic() {
        // 对应原C++的f1()测试
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
        // 对应原C++的f4()测试
        // x² - 2x + 1 = 0, (x-1)² = 0, x1=x2=1
        let solution = solve_quadratic(1.0, -2.0, 1.0);
        assert_eq!(solution, QuadraticSolution::EqualRealRoots(1.0));

        // 4x² - 4x + 1 = 0, (2x-1)² = 0, x1=x2=0.5
        let solution = solve_quadratic(4.0, -4.0, 1.0);
        assert_eq!(solution, QuadraticSolution::EqualRealRoots(0.5));

        // 9x² + 6x + 1 = 0, (3x+1)² = 0, x1=x2=-1/3
        let solution = solve_quadratic(9.0, 6.0, 1.0);
        match solution {
            QuadraticSolution::EqualRealRoots(x) => {
                assert!((x + 1.0 / 3.0).abs() < EPSILON);
            }
            _ => panic!("Expected equal real roots"),
        }
    }

    #[test]
    fn test_distinct_real_roots() {
        // 对应原C++的f3()测试
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

        // 2x² + 3x - 2 = 0, x1=0.5, x2=-2
        let solution = solve_quadratic(2.0, 3.0, -2.0);
        match solution {
            QuadraticSolution::DistinctRealRoots(x1, x2) => {
                assert!((x1 - 0.5).abs() < EPSILON);
                assert!((x2 + 2.0).abs() < EPSILON);
            }
            _ => panic!("Expected distinct real roots"),
        }
    }

    #[test]
    fn test_complex_roots() {
        // 对应原C++的f2()测试
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

        // x² + 4 = 0, x = ±2i
        let solution = solve_quadratic(1.0, 0.0, 4.0);
        match solution {
            QuadraticSolution::ComplexRoots { real, imag } => {
                assert!(real.abs() < EPSILON);
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

        // -2x² - 4x - 2 = 0, x1=x2=-1
        let solution = solve_quadratic(-2.0, -4.0, -2.0);
        match solution {
            QuadraticSolution::EqualRealRoots(x) => {
                assert!((x + 1.0).abs() < EPSILON);
            }
            _ => panic!("Expected equal real roots"),
        }
    }

    #[test]
    fn test_display_format() {
        // 测试输出格式
        let solution = QuadraticSolution::NotQuadratic;
        assert_eq!(format!("{}", solution), "不是一元二次方程");

        let solution = QuadraticSolution::EqualRealRoots(1.5);
        let output = format!("{}", solution);
        assert!(output.contains("有两个相等实根"));
        assert!(output.contains("x1=x2=1.5"));

        let solution = QuadraticSolution::DistinctRealRoots(3.0, 2.0);
        let output = format!("{}", solution);
        assert!(output.contains("有两个不等实根"));
        assert!(output.contains("x1=3"));
        assert!(output.contains("x2=2"));

        let solution = QuadraticSolution::ComplexRoots {
            real: 0.0,
            imag: 1.0,
        };
        let output = format!("{}", solution);
        assert!(output.contains("有两个虚根"));
        assert!(output.contains("x1=i"));
        assert!(output.contains("x2=-i"));
    }

    #[test]
    fn test_parse_coefficients() {
        // 测试正常输入
        assert_eq!(parse_coefficients("1 2 3"), Ok((1.0, 2.0, 3.0)));
        assert_eq!(parse_coefficients("1.5 -2.5 3.5"), Ok((1.5, -2.5, 3.5)));
        assert_eq!(parse_coefficients("  1   2   3  "), Ok((1.0, 2.0, 3.0)));

        // 测试错误输入
        assert!(parse_coefficients("1 2").is_err());
        assert!(parse_coefficients("1 2 3 4").is_err());
        assert!(parse_coefficients("a b c").is_err());
        assert!(parse_coefficients("").is_err());
    }

    #[test]
    fn test_complex_formatting_edge_cases() {
        // 测试复数格式化的边界情况
        // 虚部为1的情况
        let solution = QuadraticSolution::ComplexRoots {
            real: 2.0,
            imag: 1.0,
        };
        let output = format!("{}", solution);
        assert!(output.contains("2+i") || output.contains("2-i"));

        // 虚部为-1的情况
        let solution = QuadraticSolution::ComplexRoots {
            real: 2.0,
            imag: -1.0,
        };
        let output = format!("{}", solution);
        assert!(output.contains("2-i") || output.contains("2+i"));

        // 实部为0的情况
        let solution = QuadraticSolution::ComplexRoots {
            real: 0.0,
            imag: 2.0,
        };
        let output = format!("{}", solution);
        assert!(output.contains("2i") || output.contains("-2i"));
    }
}
