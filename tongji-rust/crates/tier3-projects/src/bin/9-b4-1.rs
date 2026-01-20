// 9-b4-1: Triangle area calculator using cross product
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Copy trait的Point结构体，避免不必要的克隆
// 2. 使用Option<f64>代替返回-1表示错误，类型安全的错误处理
// 3. 零unsafe代码，完全内存安全
// 4. 使用const fn构造函数，编译期常量计算
// 5. 提取面积计算为纯函数，便于单元测试
// 6. 使用f64::abs代替C++的abs，避免整数除法精度损失
// 7. 添加comprehensive单元测试覆盖各种三角形情况
// 8. 使用叉积公式计算面积，数学上更清晰
// 9. 实现Display trait提供友好的输出格式
// 10. 使用derive宏自动实现常用trait，减少样板代码

/// 二维平面上的点
///
/// Rust改进: 使用Copy trait，Point是小型值类型，复制比引用更高效
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// 创建新的点
    ///
    /// Rust改进: const fn允许编译期计算，可用于常量定义
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

/// 三角形结构
///
/// Rust改进: 使用三个Point组成，类型更清晰
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
    /// Rust改进: 返回Option<f64>而不是-1表示错误
    /// - Some(area) 表示有效三角形及其面积
    /// - None 表示三点共线，无法构成三角形
    ///
    /// 使用叉积公式: area = |AB × AC| / 2
    /// 其中 AB = (x2-x1, y2-y1), AC = (x3-x1, y3-y1)
    /// 叉积 = (x2-x1)*(y3-y1) - (x3-x1)*(y2-y1)
    ///
    /// # Returns
    /// * `Some(area)` - 三角形面积（大于0）
    /// * `None` - 三点共线，无法构成三角形
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

/// 计算三角形面积的纯函数
///
/// Rust改进: 提取为独立函数，便于测试和复用
///
/// # Arguments
/// * `p1`, `p2`, `p3` - 三角形的三个顶点
///
/// # Returns
/// * `Some(area)` - 三角形面积
/// * `None` - 三点共线
///
/// # Algorithm
/// 使用向量叉积公式:
/// - 向量AB = (x2-x1, y2-y1)
/// - 向量AC = (x3-x1, y3-y1)
/// - 叉积 = AB.x * AC.y - AC.x * AB.y
/// - 面积 = |叉积| / 2
fn calculate_triangle_area(p1: Point, p2: Point, p3: Point) -> Option<f64> {
    // Rust改进: 使用i64避免i32乘法溢出
    let x1 = i64::from(p1.x());
    let y1 = i64::from(p1.y());
    let x2 = i64::from(p2.x());
    let y2 = i64::from(p2.y());
    let x3 = i64::from(p3.x());
    let y3 = i64::from(p3.y());

    // 计算向量AB和AC
    let dx1 = x2 - x1; // AB.x
    let dy1 = y2 - y1; // AB.y
    let dx2 = x3 - x1; // AC.x
    let dy2 = y3 - y1; // AC.y

    // 计算叉积: AB × AC
    let cross_product = dx1 * dy2 - dx2 * dy1;

    // Rust改进: 使用f64::abs和除法，避免整数除法精度损失
    let area = (cross_product as f64).abs() / 2.0;

    // Rust改进: 使用Option表示是否为有效三角形
    if area == 0.0 {
        None // 三点共线
    } else {
        Some(area)
    }
}

/// 格式化面积输出
///
/// Rust改进: 统一的格式化函数，处理Option类型
fn format_area(area: Option<f64>) -> String {
    match area {
        Some(a) => format!("{}", a),
        None => "-1".to_string(),
    }
}

fn main() {
    // 测试用例1: (0,0) (0,1) (1,0) - 面积应为0.5
    if true {
        let tr = Triangle::new(0, 0, 0, 1, 1, 0);
        println!(
            "三角形面积应该是：0.5，实际是：{}",
            format_area(tr.area())
        );
    }

    // 测试用例2: (0,2) (-1,-1) (1,-1) - 面积应为3
    if true {
        let tr = Triangle::new(0, 2, -1, -1, 1, -1);
        println!(
            "三角形面积应该是：3，  实际是：{}",
            format_area(tr.area())
        );
    }

    // 测试用例3: (5,5) (-3,1) (9,-2) - 面积应为36
    if true {
        let tr = Triangle::new(5, 5, -3, 1, 9, -2);
        println!(
            "三角形面积应该是：36， 实际是：{}",
            format_area(tr.area())
        );
    }

    // 测试用例4: (0,0) (1,1) (2,2) - 三点共线，应返回-1
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
    fn test_point_copy() {
        // 测试Point的Copy trait
        let p1 = Point::new(1, 2);
        let p2 = p1; // Copy而不是Move
        assert_eq!(p1, p2);
        assert_eq!(p1.x(), 1); // p1仍然可用
    }

    #[test]
    fn test_triangle_area_simple() {
        // 测试简单直角三角形: (0,0) (0,1) (1,0)
        // 面积 = 1 * 1 / 2 = 0.5
        let tr = Triangle::new(0, 0, 0, 1, 1, 0);
        assert_eq!(tr.area(), Some(0.5));
    }

    #[test]
    fn test_triangle_area_case2() {
        // 测试用例2: (0,2) (-1,-1) (1,-1)
        // 向量AB = (-1, -3), AC = (1, -3)
        // 叉积 = (-1)*(-3) - 1*(-3) = 3 + 3 = 6
        // 面积 = 6 / 2 = 3
        let tr = Triangle::new(0, 2, -1, -1, 1, -1);
        assert_eq!(tr.area(), Some(3.0));
    }

    #[test]
    fn test_triangle_area_case3() {
        // 测试用例3: (5,5) (-3,1) (9,-2)
        // 向量AB = (-8, -4), AC = (4, -7)
        // 叉积 = (-8)*(-7) - 4*(-4) = 56 + 16 = 72
        // 面积 = 72 / 2 = 36
        let tr = Triangle::new(5, 5, -3, 1, 9, -2);
        assert_eq!(tr.area(), Some(36.0));
    }

    #[test]
    fn test_triangle_collinear() {
        // 测试共线点: (0,0) (1,1) (2,2)
        // 向量AB = (1, 1), AC = (2, 2)
        // 叉积 = 1*2 - 2*1 = 0
        let tr = Triangle::new(0, 0, 1, 1, 2, 2);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_triangle_collinear_horizontal() {
        // 测试水平共线: (0,0) (1,0) (2,0)
        let tr = Triangle::new(0, 0, 1, 0, 2, 0);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_triangle_collinear_vertical() {
        // 测试垂直共线: (0,0) (0,1) (0,2)
        let tr = Triangle::new(0, 0, 0, 1, 0, 2);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_triangle_area_order_independence() {
        // 测试顶点顺序不影响面积（只影响符号，但我们取绝对值）
        let tr1 = Triangle::new(0, 0, 0, 1, 1, 0);
        let tr2 = Triangle::new(0, 0, 1, 0, 0, 1); // 不同顺序
        assert_eq!(tr1.area(), tr2.area());
    }

    #[test]
    fn test_triangle_negative_coordinates() {
        // 测试负坐标: (-1,-1) (1,-1) (0,1)
        // 向量AB = (2, 0), AC = (1, 2)
        // 叉积 = 2*2 - 1*0 = 4
        // 面积 = 4 / 2 = 2
        let tr = Triangle::new(-1, -1, 1, -1, 0, 1);
        assert_eq!(tr.area(), Some(2.0));
    }

    #[test]
    fn test_triangle_large_coordinates() {
        // 测试大坐标值（检查i64转换）
        let tr = Triangle::new(0, 0, 1000, 0, 0, 1000);
        // 面积 = 1000 * 1000 / 2 = 500000
        assert_eq!(tr.area(), Some(500000.0));
    }

    #[test]
    fn test_calculate_triangle_area_function() {
        // 测试独立的面积计算函数
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 0);
        let p3 = Point::new(0, 4);
        // 3-4-5直角三角形，面积 = 3 * 4 / 2 = 6
        assert_eq!(calculate_triangle_area(p1, p2, p3), Some(6.0));
    }

    #[test]
    fn test_format_area_some() {
        // 测试格式化有效面积
        assert_eq!(format_area(Some(3.5)), "3.5");
        assert_eq!(format_area(Some(0.5)), "0.5");
    }

    #[test]
    fn test_format_area_none() {
        // 测试格式化无效面积（共线）
        assert_eq!(format_area(None), "-1");
    }

    #[test]
    fn test_triangle_same_point() {
        // 测试相同点（退化情况）
        let tr = Triangle::new(1, 1, 1, 1, 1, 1);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_triangle_two_same_points() {
        // 测试两个点相同
        let tr = Triangle::new(0, 0, 0, 0, 1, 1);
        assert_eq!(tr.area(), None);
    }

    #[test]
    fn test_triangle_equilateral() {
        // 测试等边三角形（近似）
        // 边长为2的等边三角形，高 = sqrt(3)，面积 = 2 * sqrt(3) / 2 = sqrt(3) ≈ 1.732
        let tr = Triangle::new(0, 0, 2, 0, 1, 2);
        // 向量AB = (2, 0), AC = (1, 2)
        // 叉积 = 2*2 - 1*0 = 4
        // 面积 = 4 / 2 = 2
        assert_eq!(tr.area(), Some(2.0));
    }

    #[test]
    fn test_triangle_isosceles() {
        // 测试等腰三角形: (0,0) (4,0) (2,3)
        // 向量AB = (4, 0), AC = (2, 3)
        // 叉积 = 4*3 - 2*0 = 12
        // 面积 = 12 / 2 = 6
        let tr = Triangle::new(0, 0, 4, 0, 2, 3);
        assert_eq!(tr.area(), Some(6.0));
    }

    #[test]
    fn test_triangle_copy_trait() {
        // 测试Triangle的Copy trait
        let tr1 = Triangle::new(0, 0, 1, 0, 0, 1);
        let tr2 = tr1; // Copy而不是Move
        assert_eq!(tr1.area(), tr2.area());
        assert_eq!(tr1, tr2); // tr1仍然可用
    }
}
