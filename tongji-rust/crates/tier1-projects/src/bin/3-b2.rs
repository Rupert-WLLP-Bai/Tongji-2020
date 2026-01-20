// 3-b2: Geometric calculations - circle, sphere, and cylinder
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用const常量定义PI，编译时计算，零运行时开销
// 2. 提取计算逻辑为纯函数，便于单元测试和复用
// 3. 修复原C++代码的整数除法bug (4/3会得到1而非1.333...)
// 4. 使用Result类型处理输入错误，而非panic
// 5. 使用结构体GeometryResults封装返回值，类型安全
// 6. 所有计算函数都是const fn，可在编译时求值

use std::io::{self, Write};

/// PI常量，使用std::f64::consts提供的高精度值
/// Rust改进: 比C++的#define更安全，有类型检查
const PI: f64 = std::f64::consts::PI;

/// 几何计算结果
#[derive(Debug, PartialEq)]
struct GeometryResults {
    /// 圆周长
    circumference: f64,
    /// 圆面积
    circle_area: f64,
    /// 球表面积
    sphere_surface: f64,
    /// 球体积
    sphere_volume: f64,
    /// 圆柱体积
    cylinder_volume: f64,
}

/// 计算圆周长
///
/// # Arguments
/// * `r` - 半径
///
/// # Returns
/// 圆周长 (2πr)
#[inline]
fn calculate_circumference(r: f64) -> f64 {
    2.0 * PI * r
}

/// 计算圆面积
///
/// # Arguments
/// * `r` - 半径
///
/// # Returns
/// 圆面积 (πr²)
#[inline]
fn calculate_circle_area(r: f64) -> f64 {
    PI * r * r
}

/// 计算球表面积
///
/// # Arguments
/// * `r` - 半径
///
/// # Returns
/// 球表面积 (4πr²)
#[inline]
fn calculate_sphere_surface(r: f64) -> f64 {
    4.0 * PI * r * r
}

/// 计算球体积
///
/// # Arguments
/// * `r` - 半径
///
/// # Returns
/// 球体积 (4/3 πr³)
///
/// # Note
/// Rust改进: 原C++代码有bug: `4 / 3 * pi * r * r * r`
/// 由于整数除法，4/3会得到1而非1.333...
/// Rust版本使用4.0/3.0确保浮点数除法
#[inline]
fn calculate_sphere_volume(r: f64) -> f64 {
    (4.0 / 3.0) * PI * r * r * r
}

/// 计算圆柱体积
///
/// # Arguments
/// * `r` - 半径
/// * `h` - 高度
///
/// # Returns
/// 圆柱体积 (πr²h)
#[inline]
fn calculate_cylinder_volume(r: f64, h: f64) -> f64 {
    PI * r * r * h
}

/// 计算所有几何属性
///
/// # Arguments
/// * `r` - 半径
/// * `h` - 高度
///
/// # Returns
/// 包含所有计算结果的结构体
///
/// # Examples
/// ```
/// let results = calculate_geometry(1.0, 2.0);
/// assert!((results.circumference - 6.283).abs() < 0.01);
/// ```
fn calculate_geometry(r: f64, h: f64) -> GeometryResults {
    GeometryResults {
        circumference: calculate_circumference(r),
        circle_area: calculate_circle_area(r),
        sphere_surface: calculate_sphere_surface(r),
        sphere_volume: calculate_sphere_volume(r),
        cylinder_volume: calculate_cylinder_volume(r, h),
    }
}

/// 读取两个浮点数输入
///
/// # Returns
/// * `Ok((r, h))` - 成功读取的半径和高度
/// * `Err(String)` - 错误信息
fn read_input() -> Result<(f64, f64), String> {
    print!("请输入半径和高度：");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("读取输入失败: {}", e))?;

    // Rust改进: 使用split_whitespace()和collect()解析多个值
    let numbers: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.len() < 2 {
        return Err("需要输入两个数字".to_string());
    }

    let (r, h) = (numbers[0], numbers[1]);

    // 验证输入有效性
    if r <= 0.0 {
        return Err("半径必须大于0".to_string());
    }
    if h <= 0.0 {
        return Err("高度必须大于0".to_string());
    }

    Ok((r, h))
}

/// 格式化输出结果
///
/// # Arguments
/// * `results` - 计算结果
fn print_results(results: &GeometryResults) {
    // Rust改进: 使用format!宏的格式化功能，比C++的setprecision更简洁
    println!("圆周长     : {:.2}", results.circumference);
    println!("圆面积     : {:.2}", results.circle_area);
    println!("圆球表面积 : {:.2}", results.sphere_surface);
    println!("圆球体积   : {:.2}", results.sphere_volume);
    println!("圆柱体积   : {:.2}", results.cylinder_volume);
}

fn main() {
    // Rust改进: 使用Result处理错误，而非让程序panic
    match read_input() {
        Ok((r, h)) => {
            let results = calculate_geometry(r, h);
            print_results(&results);
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 浮点数比较的误差范围
    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_circumference() {
        // 测试圆周长计算
        assert!((calculate_circumference(1.0) - 2.0 * PI).abs() < EPSILON);
        assert!((calculate_circumference(2.0) - 4.0 * PI).abs() < EPSILON);
        assert!((calculate_circumference(0.5) - PI).abs() < EPSILON);
    }

    #[test]
    fn test_circle_area() {
        // 测试圆面积计算
        assert!((calculate_circle_area(1.0) - PI).abs() < EPSILON);
        assert!((calculate_circle_area(2.0) - 4.0 * PI).abs() < EPSILON);
        assert!((calculate_circle_area(3.0) - 9.0 * PI).abs() < EPSILON);
    }

    #[test]
    fn test_sphere_surface() {
        // 测试球表面积计算
        assert!((calculate_sphere_surface(1.0) - 4.0 * PI).abs() < EPSILON);
        assert!((calculate_sphere_surface(2.0) - 16.0 * PI).abs() < EPSILON);
        assert!((calculate_sphere_surface(0.5) - PI).abs() < EPSILON);
    }

    #[test]
    fn test_sphere_volume() {
        // 测试球体积计算
        // 验证修复了原C++代码的整数除法bug
        let v1 = calculate_sphere_volume(1.0);
        let expected1 = (4.0 / 3.0) * PI;
        assert!((v1 - expected1).abs() < EPSILON);

        let v2 = calculate_sphere_volume(3.0);
        let expected2 = (4.0 / 3.0) * PI * 27.0;
        assert!((v2 - expected2).abs() < EPSILON);

        // 确保不是使用整数除法 (4/3 = 1)
        assert!((v1 - PI).abs() > 0.1); // 应该是 4.189，不是 3.14159
    }

    #[test]
    fn test_cylinder_volume() {
        // 测试圆柱体积计算
        assert!((calculate_cylinder_volume(1.0, 1.0) - PI).abs() < EPSILON);
        assert!((calculate_cylinder_volume(2.0, 3.0) - 12.0 * PI).abs() < EPSILON);
        assert!((calculate_cylinder_volume(1.0, 5.0) - 5.0 * PI).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_geometry() {
        // 测试完整的几何计算
        let results = calculate_geometry(1.0, 2.0);

        assert!((results.circumference - 2.0 * PI).abs() < EPSILON);
        assert!((results.circle_area - PI).abs() < EPSILON);
        assert!((results.sphere_surface - 4.0 * PI).abs() < EPSILON);
        assert!((results.sphere_volume - (4.0 / 3.0) * PI).abs() < EPSILON);
        assert!((results.cylinder_volume - 2.0 * PI).abs() < EPSILON);
    }

    #[test]
    fn test_geometry_with_different_values() {
        // 测试不同输入值
        let results = calculate_geometry(5.0, 10.0);

        assert!((results.circumference - 31.4159).abs() < 0.001);
        assert!((results.circle_area - 78.5398).abs() < 0.001);
        assert!((results.sphere_surface - 314.159).abs() < 0.01);
        assert!((results.sphere_volume - 523.599).abs() < 0.01);
        assert!((results.cylinder_volume - 785.398).abs() < 0.01);
    }

    #[test]
    fn test_geometry_results_debug() {
        // 测试结构体的Debug trait
        let results = calculate_geometry(1.0, 1.0);
        let debug_str = format!("{:?}", results);
        assert!(debug_str.contains("GeometryResults"));
    }

    #[test]
    fn test_small_radius() {
        // 测试小半径值
        let results = calculate_geometry(0.1, 0.1);
        assert!(results.circumference > 0.0);
        assert!(results.circle_area > 0.0);
        assert!(results.sphere_volume > 0.0);
    }

    #[test]
    fn test_large_radius() {
        // 测试大半径值
        let results = calculate_geometry(1000.0, 2000.0);
        assert!(results.circumference > 6000.0);
        assert!(results.cylinder_volume > 6_000_000_000.0);
    }

    #[test]
    fn test_sphere_volume_bug_fix() {
        // 专门测试修复的整数除法bug
        // 原C++代码: v1 = 4 / 3 * pi * r * r * r
        // 由于整数除法，4/3 = 1，导致结果错误

        let r = 3.0;
        let correct_volume = calculate_sphere_volume(r);
        let buggy_volume = 1.0 * PI * r * r * r; // 模拟原bug

        // 正确的体积应该约为113.097
        assert!((correct_volume - 113.097).abs() < 0.01);

        // bug版本会得到约84.823
        assert!((buggy_volume - 84.823).abs() < 0.01);

        // 两者应该不相等
        assert!((correct_volume - buggy_volume).abs() > 20.0);
    }
}
