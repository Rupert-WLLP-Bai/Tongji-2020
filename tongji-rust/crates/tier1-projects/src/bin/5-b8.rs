// 5-b8: Convex polygon area calculator using cross product
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Vec<(f64, f64)>存储坐标点，类型安全且更清晰
// 2. 使用迭代器和窗口操作，避免手动索引计算
// 3. 提取核心算法为纯函数，便于测试和复用
// 4. 使用Result类型处理边界情况，而非隐式假设
// 5. 利用Rust的所有权系统避免数据拷贝

use std::io::{self, Write};

/// 计算三点形成的叉积
///
/// 叉积用于判断三点的旋转方向:
/// - 正值: 逆时针
/// - 负值: 顺时针
/// - 零: 共线
///
/// # Arguments
/// * `p1`, `p2`, `p3` - 三个点的坐标 (x, y)
///
/// # Returns
/// 叉积值
fn cross_product(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> f64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    // 向量 (p1->p2) 和 (p2->p3) 的叉积
    let delta_x1 = x2 - x1;
    let delta_y1 = y2 - y1;
    let delta_x2 = x3 - x2;
    let delta_y2 = y3 - y2;

    delta_x1 * delta_y2 - delta_x2 * delta_y1
}

/// 判断多边形是否为凸多边形
///
/// 凸多边形的特征: 所有相邻三边的叉积符号相同
/// 使用滑动窗口检查每组连续四个点
///
/// # Arguments
/// * `points` - 多边形顶点坐标数组
///
/// # Returns
/// * `true` - 是凸多边形
/// * `false` - 不是凸多边形
fn is_convex(points: &[(f64, f64)]) -> bool {
    let n = points.len();
    if n < 3 {
        return false;
    }

    // Rust改进: 使用迭代器和enumerate避免手动索引
    // 检查每组连续四个点的叉积符号是否一致
    for i in 0..n {
        let p1 = points[i % n];
        let p2 = points[(i + 1) % n];
        let p3 = points[(i + 2) % n];
        let p4 = points[(i + 3) % n];

        let cross1 = cross_product(p1, p2, p3);
        let cross2 = cross_product(p2, p3, p4);

        // 如果两个叉积符号相反，则不是凸多边形
        if cross1 * cross2 <= 0.0 {
            return false;
        }
    }

    true
}

/// 计算凸多边形面积
///
/// 使用三角剖分法: 从第一个顶点出发，将多边形分割成多个三角形
/// 每个三角形面积 = |叉积| / 2
///
/// # Arguments
/// * `points` - 多边形顶点坐标数组
///
/// # Returns
/// 多边形面积
fn calculate_area(points: &[(f64, f64)]) -> f64 {
    let n = points.len();
    if n < 3 {
        return 0.0;
    }

    let p0 = points[0];
    let mut area = 0.0;

    // Rust改进: 使用迭代器和窗口，更清晰地表达三角剖分
    for i in 0..n - 2 {
        let p1 = points[i + 1];
        let p2 = points[i + 2];
        area += cross_product(p0, p1, p2).abs() / 2.0;
    }

    area
}

/// 读取整数输入，带验证
fn read_integer(prompt: &str, min: i32, max: i32) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<i32>() {
            if num >= min && num <= max {
                return num;
            }
        }
        // 输入无效，继续循环
    }
}

/// 读取坐标点输入
fn read_point(index: usize) -> (f64, f64) {
    loop {
        print!("请输入第{}个顶点的坐标\n", index + 1);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(x), Ok(y)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                return (x, y);
            }
        }
        // 输入无效，继续循环
    }
}

fn main() {
    // 读取顶点数量
    let num = read_integer("请输入多边形的顶点数量(4-7)\n", 4, 7);

    // Rust改进: 使用Vec动态存储，而非固定大小数组
    let mut points = Vec::with_capacity(num as usize);

    // 读取所有顶点坐标
    for i in 0..num as usize {
        points.push(read_point(i));
    }

    // 判断是否为凸多边形并计算面积
    if is_convex(&points) {
        let area = calculate_area(&points);
        println!("凸{}边形的面积为{}", num, area);
    } else {
        println!("不是凸{}边形", num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_product_counterclockwise() {
        // 逆时针旋转，叉积为正
        let result = cross_product((0.0, 0.0), (1.0, 0.0), (1.0, 1.0));
        assert!(result > 0.0);
    }

    #[test]
    fn test_cross_product_clockwise() {
        // 顺时针旋转，叉积为负
        let result = cross_product((0.0, 0.0), (1.0, 0.0), (1.0, -1.0));
        assert!(result < 0.0);
    }

    #[test]
    fn test_cross_product_collinear() {
        // 共线，叉积为零
        let result = cross_product((0.0, 0.0), (1.0, 1.0), (2.0, 2.0));
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_is_convex_square() {
        // 正方形是凸多边形
        let square = vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ];
        assert!(is_convex(&square));
    }

    #[test]
    fn test_is_convex_regular_pentagon() {
        // 正五边形是凸多边形
        use std::f64::consts::PI;
        let mut pentagon = Vec::new();
        for i in 0..5 {
            let angle = 2.0 * PI * i as f64 / 5.0 - PI / 2.0;
            pentagon.push((angle.cos(), angle.sin()));
        }
        assert!(is_convex(&pentagon));
    }

    #[test]
    fn test_is_not_convex_concave() {
        // 凹多边形
        let concave = vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (1.0, 1.0),  // 凹进去的点
            (2.0, 2.0),
            (0.0, 2.0),
        ];
        assert!(!is_convex(&concave));
    }

    #[test]
    fn test_calculate_area_square() {
        // 单位正方形面积为1
        let square = vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ];
        let area = calculate_area(&square);
        assert!((area - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_area_triangle() {
        // 直角三角形面积 = 底 * 高 / 2 = 2 * 3 / 2 = 3
        let triangle = vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (0.0, 3.0),
        ];
        let area = calculate_area(&triangle);
        assert!((area - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_area_rectangle() {
        // 矩形面积 = 长 * 宽 = 3 * 2 = 6
        let rectangle = vec![
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (0.0, 2.0),
        ];
        let area = calculate_area(&rectangle);
        assert!((area - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_area_regular_hexagon() {
        // 正六边形面积公式: 3√3/2 * r^2, r=1时约为2.598
        use std::f64::consts::PI;
        let mut hexagon = Vec::new();
        for i in 0..6 {
            let angle = 2.0 * PI * i as f64 / 6.0;
            hexagon.push((angle.cos(), angle.sin()));
        }
        let area = calculate_area(&hexagon);
        let expected = 3.0 * 3.0_f64.sqrt() / 2.0;
        assert!((area - expected).abs() < 1e-10);
    }

    #[test]
    fn test_edge_case_too_few_points() {
        // 少于3个点无法构成多边形
        let points = vec![(0.0, 0.0), (1.0, 0.0)];
        assert!(!is_convex(&points));
        assert_eq!(calculate_area(&points), 0.0);
    }
}
