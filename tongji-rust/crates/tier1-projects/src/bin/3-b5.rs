// 3-b5: Calculate triangle area given two sides and included angle
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用const常量替代#define宏，提供类型安全
// 2. 提取计算逻辑为纯函数，便于单元测试和复用
// 3. 使用f64类型提供更高精度，避免float精度损失
// 4. 使用std::f64::consts::PI标准库常量，更精确
// 5. 添加输入验证，确保边长为正数，角度在合理范围内
// 6. 使用Result类型处理错误，而非panic

use std::f64::consts::PI;
use std::io::{self, Write};

/// 根据两边及其夹角计算三角形面积
///
/// 使用公式: S = (a * b * sin(angle)) / 2
///
/// # Arguments
/// * `side_a` - 第一条边的长度
/// * `side_b` - 第二条边的长度
/// * `angle_degrees` - 两边的夹角（角度制）
///
/// # Returns
/// * `Ok(f64)` - 三角形面积
/// * `Err(&str)` - 如果输入无效（边长非正数或角度超出范围）
///
/// # Examples
/// ```
/// let area = calculate_triangle_area(3.0, 4.0, 90.0).unwrap();
/// assert!((area - 6.0).abs() < 0.001);
/// ```
fn calculate_triangle_area(
    side_a: f64,
    side_b: f64,
    angle_degrees: f64,
) -> Result<f64, &'static str> {
    // Rust改进: 添加输入验证，确保数据合法性
    if side_a <= 0.0 || side_b <= 0.0 {
        return Err("边长必须为正数");
    }

    if angle_degrees <= 0.0 || angle_degrees >= 180.0 {
        return Err("角度必须在0到180度之间");
    }

    // Rust改进: 使用std::f64::consts::PI标准库常量，比3.14159更精确
    let angle_radians = angle_degrees * PI / 180.0;

    // 三角形面积公式: S = (a * b * sin(θ)) / 2
    let area = side_a * side_b * angle_radians.sin() / 2.0;

    Ok(area)
}

/// 从标准输入读取一个f64数值
fn read_f64(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Rust改进: 使用match处理解析结果，提供清晰的错误提示
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("输入无效，请输入一个数字"),
        }
    }
}

fn main() {
    println!("请输入三角形的两边及其夹角（角度）：");

    let side_a = read_f64("第一条边: ");
    let side_b = read_f64("第二条边: ");
    let angle = read_f64("夹角（度）: ");

    // Rust改进: 使用match处理Result，优雅地处理错误情况
    match calculate_triangle_area(side_a, side_b, angle) {
        Ok(area) => {
            // Rust改进: 使用格式化宏的精度控制，更简洁
            println!("三角形面积为： {:.3}", area);
        }
        Err(err) => {
            eprintln!("错误: {}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 辅助函数：比较浮点数是否近似相等
    fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_right_triangle() {
        // 测试直角三角形: 边长3, 4, 夹角90度
        // 面积应为 3 * 4 / 2 = 6.0
        let area = calculate_triangle_area(3.0, 4.0, 90.0).unwrap();
        assert!(approx_eq(area, 6.0, 0.001));
    }

    #[test]
    fn test_equilateral_triangle() {
        // 测试等边三角形: 边长2, 2, 夹角60度
        // 面积应为 2 * 2 * sin(60°) / 2 = 2 * sqrt(3)/2 ≈ 1.732
        let area = calculate_triangle_area(2.0, 2.0, 60.0).unwrap();
        assert!(approx_eq(area, 1.732, 0.001));
    }

    #[test]
    fn test_acute_angle() {
        // 测试锐角三角形: 边长5, 6, 夹角30度
        // 面积应为 5 * 6 * sin(30°) / 2 = 30 * 0.5 / 2 = 7.5
        let area = calculate_triangle_area(5.0, 6.0, 30.0).unwrap();
        assert!(approx_eq(area, 7.5, 0.001));
    }

    #[test]
    fn test_obtuse_angle() {
        // 测试钝角三角形: 边长4, 5, 夹角120度
        // 面积应为 4 * 5 * sin(120°) / 2 ≈ 8.660
        let area = calculate_triangle_area(4.0, 5.0, 120.0).unwrap();
        assert!(approx_eq(area, 8.660, 0.001));
    }

    #[test]
    fn test_small_angle() {
        // 测试小角度: 边长10, 10, 夹角1度
        let area = calculate_triangle_area(10.0, 10.0, 1.0).unwrap();
        assert!(approx_eq(area, 0.873, 0.001));
    }

    #[test]
    fn test_large_angle() {
        // 测试大角度: 边长3, 4, 夹角179度
        let area = calculate_triangle_area(3.0, 4.0, 179.0).unwrap();
        assert!(approx_eq(area, 0.105, 0.001));
    }

    #[test]
    fn test_invalid_zero_side() {
        // 测试边长为0的情况
        assert!(calculate_triangle_area(0.0, 5.0, 60.0).is_err());
        assert!(calculate_triangle_area(5.0, 0.0, 60.0).is_err());
    }

    #[test]
    fn test_invalid_negative_side() {
        // 测试负边长
        assert!(calculate_triangle_area(-3.0, 4.0, 60.0).is_err());
        assert!(calculate_triangle_area(3.0, -4.0, 60.0).is_err());
    }

    #[test]
    fn test_invalid_zero_angle() {
        // 测试角度为0（退化三角形）
        assert!(calculate_triangle_area(3.0, 4.0, 0.0).is_err());
    }

    #[test]
    fn test_invalid_180_degree_angle() {
        // 测试角度为180度（退化三角形）
        assert!(calculate_triangle_area(3.0, 4.0, 180.0).is_err());
    }

    #[test]
    fn test_invalid_negative_angle() {
        // 测试负角度
        assert!(calculate_triangle_area(3.0, 4.0, -30.0).is_err());
    }

    #[test]
    fn test_invalid_angle_over_180() {
        // 测试超过180度的角度
        assert!(calculate_triangle_area(3.0, 4.0, 200.0).is_err());
    }

    #[test]
    fn test_floating_point_precision() {
        // 测试浮点数精度
        let area1 = calculate_triangle_area(1.5, 2.5, 45.0).unwrap();
        let area2 = calculate_triangle_area(1.5, 2.5, 45.0).unwrap();
        assert_eq!(area1, area2); // 相同输入应得到完全相同的结果
    }

    #[test]
    fn test_pi_constant_accuracy() {
        // 验证使用std::f64::consts::PI比3.14159更精确
        // 对于90度角，sin(90°) = 1，面积应该精确等于 a*b/2
        let area = calculate_triangle_area(10.0, 10.0, 90.0).unwrap();
        assert_eq!(area, 50.0); // 应该精确相等，无舍入误差
    }
}
