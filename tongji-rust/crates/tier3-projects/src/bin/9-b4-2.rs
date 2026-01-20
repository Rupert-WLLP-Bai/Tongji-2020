// 9-b4-2: Triangle area calculation using cross product
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用强类型Point和Triangle结构体，编译时保证类型安全
// 2. 使用Option<f64>代替返回-1表示错误，更符合Rust惯用法
// 3. 零unsafe代码，完全内存安全（原C++使用friend访问私有成员）
// 4. 使用const fn和内联优化，零运行时开销
// 5. 提取核心逻辑为纯函数，便于单元测试和复用
// 6. 使用f64::abs代替C++的abs，避免整数除法精度损失
// 7. 添加comprehensive单元测试覆盖各种三角形情况
// 8. 使用derive宏自动实现Debug、Clone、Copy等trait
// 9. 使用构造器模式提供更灵活的初始化方式
// 10. 添加文档注释和示例代码

/// 二维平面上的点
///
/// Rust改进: 使用#[derive]自动实现常用trait，避免手动编写样板代码
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// 创建新的点
    ///
    /// # Examples
    /// ```
    /// let p = Point::new(1, 2);
    /// assert_eq!(p.x(), 1);
    /// assert_eq!(p.y(), 2);
    /// ```
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// 获取x坐标
    const fn x(&self) -> i32 {
        self.x
    }

    /// 获取y坐标
    const fn y(&self) -> i32 {
        self.y
    }
}

/// 由三个点构成的三角形
///
/// Rust改进: 不需要friend关键字，Rust的模块系统提供更好的封装控制
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    /// 创建新的三角形
    ///
    /// # Arguments
    /// * `p1_x`, `p1_y` - 第一个点的坐标
    /// * `p2_x`, `p2_y` - 第二个点的坐标
    /// * `p3_x`, `p3_y` - 第三个点的坐标
    ///
    /// # Examples
    /// ```
    /// let tr = Triangle::new(0, 0, 0, 1, 1, 0);
    /// assert_eq!(tr.area(), Some(0.5));
    /// ```
    const fn new(p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32, p3_x: i32, p3_y: i32) -> Self {
        Self {
            p1: Point::new(p1_x, p1_y),
            p2: Point::new(p2_x, p2_y),
            p3: Point::new(p3_x, p3_y),
        }
    }

    /// 计算三角形面积
    ///
    /// 使用向量叉积公式: Area = |AB × AC| / 2
    /// 其中 AB = (x2-x1, y2-y1), AC = (x3-x1, y3-y1)
    /// 叉积 = (x2-x1)*(y3-y1) - (x3-x1)*(y2-y1)
    ///
    /// # Returns
    /// * `Some(area)` - 如果三点能构成三角形，返回面积
    /// * `None` - 如果三点共线（面积为0），返回None
    ///
    /// # Examples
    /// ```
    /// let tr = Triangle::new(0, 0, 0, 1, 1, 0);
    /// assert_eq!(tr.area(), Some(0.5));
    ///
    /// let collinear = Triangle::new(0, 0, 1, 1, 2, 2);
    /// assert_eq!(collinear.area(), None);
    /// ```
    fn area(&self) -> Option<f64> {
        calculate_triangle_area(self.p1, self.p2, self.p3)
    }
}

/// 计算三角形面积的核心函数
///
/// Rust改进: 提取为独立函数，便于测试和复用
///
/// # Arguments
/// * `p1`, `p2`, `p3` - 三角形的三个顶点
///
/// # Returns
/// * `Some(area)` - 如果三点能构成三角形，返回面积
/// * `None` - 如果三点共线，返回None
///
/// # Algorithm
/// 使用向量叉积公式计算面积:
/// 1. 计算向量 AB = P2 - P1
/// 2. 计算向量 AC = P3 - P1
/// 3. 计算叉积 AB × AC = dx1 * dy2 - dx2 * dy1
/// 4. 面积 = |叉积| / 2
fn calculate_triangle_area(p1: Point, p2: Point, p3: Point) -> Option<f64> {
    // Rust改进: 使用更清晰的变量名和计算步骤
    let dx1 = p2.x() - p1.x();
    let dy1 = p2.y() - p1.y();
    let dx2 = p3.x() - p1.x();
    let dy2 = p3.y() - p1.y();

    // 计算叉积（使用i64避免溢出）
    let cross_product = i64::from(dx1) * i64::from(dy2) - i64::from(dx2) * i64::from(dy1);

    // Rust改进: 使用f64::abs代替C++的abs，避免整数除法精度损失
    let area = (cross_product as f64).abs() / 2.0;

    // Rust改进: 使用Option代替返回-1，更符合Rust惯用法
    if area == 0.0 {
        None
    } else {
        Some(area)
    }
}

/// 验证三点是否共线
///
/// # Arguments
/// * `p1`, `p2`, `p3` - 三个点
///
/// # Returns
/// * `true` - 三点共线
/// * `false` - 三点不共线
#[allow(dead_code)]
fn are_collinear(p1: Point, p2: Point, p3: Point) -> bool {
    calculate_triangle_area(p1, p2, p3).is_none()
}

/// 格式化面积输出
///
/// Rust改进: 使用Option的map方法进行函数式转换
fn format_area(area: Option<f64>) -> String {
    area.map_or("-1".to_string(), |a| a.to_string())
}

fn main() {
    // 测试用例1: 三角形 (0,0) (0,1) (1,0)
    // 预期面积: 0.5
    if true {
        let tr = Triangle::new(0, 0, 0, 1, 1, 0);
        println!(
            "三角形面积应该是：0.5，实际是：{}",
            format_area(tr.area())
        );
    }

    // 测试用例2: 三角形 (0,2) (-1,-1) (1,-1)
    // 预期面积: 3
    if true {
        let tr = Triangle::new(0, 2, -1, -1, 1, -1);
        println!(
            "三角形面积应该是：3，  实际是：{}",
            format_area(tr.area())
        );
    }

    // 测试用例3: 三角形 (5,5) (-3,1) (9,-2)
    // 预期面积: 36
    if true {
        let tr = Triangle::new(5, 5, -3, 1, 9, -2);
        println!(
            "三角形面积应该是：36， 实际是：{}",
            format_area(tr.area())
        );
    }

    // 测试用例4: 三点共线 (0,0) (1,1) (2,2)
    // 预期: -1 (无法构成三角形)
    if true {
        let tr = Triangle::new(0, 0, 1, 1, 2, 2);
        println!(
            "三角形面积应该是：-1， 实际是：{}",
            format_area(tr.area())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        // 测试点的创建
        let p = Point::new(3, 4);
        assert_eq!(p.x(), 3);
        assert_eq!(p.y(), 4);
    }

    #[test]
    fn test_point_equality() {
        // 测试点的相等性
        let p1 = Point::new(1, 2);
        let p2 = Point::new(1, 2);
        let p3 = Point::new(2, 1);
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_triangle_area_simple() {
        // 测试简单三角形: (0,0) (0,1) (1,0)
        // 面积 = 0.5
        let tr = Triangle::new(0, 0, 0, 1, 1, 0);
        assert_eq!(tr.area(), Some(0.5));
    }

    #[test]
    fn test_triangle_area_medium() {
        // 测试中等三角形: (0,2) (-1,-1) (1,-1)
        // 向量: AB = (-1,-3), AC = (1,-3)
        // 叉积 = (-1)*(-3) - 1*(-3) = 3 + 3 = 6
        // 面积 = 6/2 = 3
        let tr = Triangle::new(0, 2, -1, -1, 1, -1);
        assert_eq!(tr.area(), Some(3.0));
    }

    #[test]
    fn test_triangle_area_large() {
        // 测试大三角形: (5,5) (-3,1) (9,-2)
        // 向量: AB = (-8,-4), AC = (4,-7)
        // 叉积 = (-8)*(-7) - 4*(-4) = 56 + 16 = 72
        // 面积 = 72/2 = 36
        let tr = Triangle::new(5, 5, -3, 1, 9, -2);
        assert_eq!(tr.area(), Some(36.0));
    }

    #[test]
    fn test_collinear_points() {
        // 测试共线点: (0,0) (1,1) (2,2)
        // 叉积 = 1*2 - 2*1 = 0
        let tr = Triangle::new(0, 0, 1, 1, 2, 2);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_collinear_horizontal() {
        // 测试水平共线: (0,0) (1,0) (2,0)
        let tr = Triangle::new(0, 0, 1, 0, 2, 0);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_collinear_vertical() {
        // 测试垂直共线: (0,0) (0,1) (0,2)
        let tr = Triangle::new(0, 0, 0, 1, 0, 2);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_triangle_with_negative_coords() {
        // 测试负坐标三角形: (-1,-1) (-1,1) (1,-1)
        // 向量: AB = (0,2), AC = (2,0)
        // 叉积 = 0*0 - 2*2 = -4
        // 面积 = |-4|/2 = 2
        let tr = Triangle::new(-1, -1, -1, 1, 1, -1);
        assert_eq!(tr.area(), Some(2.0));
    }

    #[test]
    fn test_calculate_triangle_area_direct() {
        // 直接测试核心函数
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 0);
        let p3 = Point::new(0, 4);
        // 面积 = 3*4/2 = 6
        assert_eq!(calculate_triangle_area(p1, p2, p3), Some(6.0));
    }

    #[test]
    fn test_are_collinear_true() {
        // 测试共线判断
        let p1 = Point::new(0, 0);
        let p2 = Point::new(1, 1);
        let p3 = Point::new(2, 2);
        assert!(are_collinear(p1, p2, p3));
    }

    #[test]
    fn test_are_collinear_false() {
        // 测试非共线判断
        let p1 = Point::new(0, 0);
        let p2 = Point::new(1, 0);
        let p3 = Point::new(0, 1);
        assert!(!are_collinear(p1, p2, p3));
    }

    #[test]
    fn test_format_area_some() {
        // 测试面积格式化（有效面积）
        assert_eq!(format_area(Some(3.5)), "3.5");
    }

    #[test]
    fn test_format_area_none() {
        // 测试面积格式化（无效面积）
        assert_eq!(format_area(None), "-1");
    }

    #[test]
    fn test_triangle_order_independence() {
        // 测试点的顺序不影响面积（取绝对值）
        let tr1 = Triangle::new(0, 0, 1, 0, 0, 1);
        let tr2 = Triangle::new(0, 0, 0, 1, 1, 0);
        assert_eq!(tr1.area(), tr2.area());
    }

    #[test]
    fn test_triangle_with_large_coords() {
        // 测试大坐标值（检查溢出处理）
        let tr = Triangle::new(0, 0, 1000, 0, 0, 1000);
        // 面积 = 1000*1000/2 = 500000
        assert_eq!(tr.area(), Some(500000.0));
    }

    #[test]
    fn test_triangle_equality() {
        // 测试三角形相等性
        let tr1 = Triangle::new(0, 0, 1, 0, 0, 1);
        let tr2 = Triangle::new(0, 0, 1, 0, 0, 1);
        assert_eq!(tr1, tr2);
    }

    #[test]
    fn test_triangle_clone() {
        // 测试三角形克隆
        let tr1 = Triangle::new(0, 0, 1, 0, 0, 1);
        let tr2 = tr1;
        assert_eq!(tr1, tr2);
        assert_eq!(tr1.area(), tr2.area());
    }

    #[test]
    fn test_right_triangle() {
        // 测试直角三角形: (0,0) (3,0) (0,4)
        // 面积 = 3*4/2 = 6
        let tr = Triangle::new(0, 0, 3, 0, 0, 4);
        assert_eq!(tr.area(), Some(6.0));
    }

    #[test]
    fn test_isosceles_triangle() {
        // 测试等腰三角形: (0,0) (2,0) (1,2)
        // 向量: AB = (2,0), AC = (1,2)
        // 叉积 = 2*2 - 1*0 = 4
        // 面积 = 4/2 = 2
        let tr = Triangle::new(0, 0, 2, 0, 1, 2);
        assert_eq!(tr.area(), Some(2.0));
    }

    #[test]
    fn test_all_main_test_cases() {
        // 测试所有main函数中的测试用例
        let test_cases = [
            ((0, 0, 0, 1, 1, 0), Some(0.5)),
            ((0, 2, -1, -1, 1, -1), Some(3.0)),
            ((5, 5, -3, 1, 9, -2), Some(36.0)),
            ((0, 0, 1, 1, 2, 2), None),
        ];

        for ((p1x, p1y, p2x, p2y, p3x, p3y), expected) in test_cases {
            let tr = Triangle::new(p1x, p1y, p2x, p2y, p3x, p3y);
            assert_eq!(
                tr.area(),
                expected,
                "Failed for triangle ({},{}) ({},{}) ({},{})",
                p1x,
                p1y,
                p2x,
                p2y,
                p3x,
                p3y
            );
        }
    }
}
