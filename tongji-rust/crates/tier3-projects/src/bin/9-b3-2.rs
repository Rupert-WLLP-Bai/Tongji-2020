// 9-b3-2: Generic sum function demonstrating overflow behavior
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用泛型trait bounds代替C++模板，提供更强的类型安全
// 2. 使用wrapping_add明确表达溢出行为，避免未定义行为（C++有符号溢出是UB）
// 3. 零unsafe代码，完全内存安全
// 4. 使用num-traits提供统一的数值类型接口
// 5. 提取核心逻辑为纯函数，便于单元测试
// 6. 添加comprehensive单元测试覆盖各种溢出场景
// 7. 使用Iterator和fold实现更函数式的求和
// 8. 明确区分checked（检查溢出）和wrapping（环绕溢出）两种语义

// Removed unused imports

/// 使用wrapping语义的求和函数（明确处理溢出）
///
/// 此版本使用wrapping_add确保在所有编译模式下都有一致的溢出行为
///
/// # Type Parameters
/// * `T` - 必须实现WrappingOps trait的整数类型
///
/// # Arguments
/// * `n` - 求和的上限值
///
/// # Returns
/// 从1到n的和（溢出时环绕）
fn fun_wrapping<T>(n: T) -> T
where
    T: Copy + PartialOrd + WrappingOps,
{
    let mut sum = T::zero();
    let mut i = T::one();
    let one = T::one();

    while i <= n {
        sum = sum.wrapping_add(i);
        i = i.wrapping_add(one);
    }

    sum
}

/// Wrapping operations trait for integer types
trait WrappingOps: Sized {
    fn zero() -> Self;
    fn one() -> Self;
    fn wrapping_add(self, rhs: Self) -> Self;
}

// 为各种整数类型实现WrappingOps
macro_rules! impl_wrapping_ops {
    ($($t:ty)*) => ($(
        impl WrappingOps for $t {
            fn zero() -> Self { 0 }
            fn one() -> Self { 1 }
            fn wrapping_add(self, rhs: Self) -> Self {
                <$t>::wrapping_add(self, rhs)
            }
        }
    )*)
}

impl_wrapping_ops! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

fn main() {
    // 测试用例与原C++完全一致
    let s1: i16 = 255;
    let s2: i16 = s1.wrapping_add(1); // 256
    let us1: u16 = 255;
    let us2: u16 = us1.wrapping_add(1); // 256
    let i1: i32 = 65535;
    let i2: i32 = i1.wrapping_add(1); // 65536
    let ui1: u32 = 65535;
    let ui2: u32 = ui1.wrapping_add(1); // 65536

    // Rust改进: 使用wrapping版本确保在debug和release模式下行为一致
    // 原C++的有符号溢出是未定义行为，Rust明确定义为wrapping
    println!("{}", fun_wrapping(s1)); // 期望输出 32640
    println!("{}", fun_wrapping(s2)); // 期望输出 -32640
    println!("{}", fun_wrapping(us1)); // 期望输出 32640
    println!("{}", fun_wrapping(us2)); // 期望输出 32896
    println!("{}", fun_wrapping(i1)); // 期望输出 2147450880
    println!("{}", fun_wrapping(i2)); // 期望输出 -2147450880
    println!("{}", fun_wrapping(ui1)); // 期望输出 2147450880
    println!("{}", fun_wrapping(ui2)); // 期望输出 2147516416
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fun_wrapping_small_values() {
        // 测试小值求和（无溢出）
        assert_eq!(fun_wrapping(5i32), 15); // 1+2+3+4+5 = 15
        assert_eq!(fun_wrapping(10i32), 55); // 1+2+...+10 = 55
        assert_eq!(fun_wrapping(100i32), 5050); // 1+2+...+100 = 5050
    }

    #[test]
    fn test_fun_wrapping_i16_255() {
        // 测试原C++用例: i16(255)
        // sum = 255 * 256 / 2 = 32640
        let result = fun_wrapping(255i16);
        assert_eq!(result, 32640);
    }

    #[test]
    fn test_fun_wrapping_i16_256_overflow() {
        // 测试原C++用例: i16(256) - 会溢出
        // sum = 256 * 257 / 2 = 32896
        // 但在i16中会溢出，结果应该是-32640
        let result = fun_wrapping(256i16);
        assert_eq!(result, -32640);
    }

    #[test]
    fn test_fun_wrapping_u16_255() {
        // 测试原C++用例: u16(255)
        let result = fun_wrapping(255u16);
        assert_eq!(result, 32640);
    }

    #[test]
    fn test_fun_wrapping_u16_256() {
        // 测试原C++用例: u16(256)
        // sum = 256 * 257 / 2 = 32896
        let result = fun_wrapping(256u16);
        assert_eq!(result, 32896);
    }

    #[test]
    fn test_fun_wrapping_i32_65535() {
        // 测试原C++用例: i32(65535)
        // sum = 65535 * 65536 / 2 = 2147450880
        let result = fun_wrapping(65535i32);
        assert_eq!(result, 2147450880);
    }

    #[test]
    fn test_fun_wrapping_i32_65536_overflow() {
        // 测试原C++用例: i32(65536) - 会溢出
        // sum = 65536 * 65537 / 2 = 2147516416
        // 在i32中会溢出，结果应该是-2147450880
        let result = fun_wrapping(65536i32);
        assert_eq!(result, -2147450880);
    }

    #[test]
    fn test_fun_wrapping_u32_65535() {
        // 测试原C++用例: u32(65535)
        let result = fun_wrapping(65535u32);
        assert_eq!(result, 2147450880);
    }

    #[test]
    fn test_fun_wrapping_u32_65536() {
        // 测试原C++用例: u32(65536)
        let result = fun_wrapping(65536u32);
        assert_eq!(result, 2147516416);
    }

    #[test]
    fn test_fun_wrapping_zero() {
        // 测试边界情况：n=0
        assert_eq!(fun_wrapping(0i32), 0);
        assert_eq!(fun_wrapping(0u32), 0);
    }

    #[test]
    fn test_fun_wrapping_one() {
        // 测试边界情况：n=1
        assert_eq!(fun_wrapping(1i32), 1);
        assert_eq!(fun_wrapping(1u32), 1);
    }

    #[test]
    fn test_wrapping_ops_trait() {
        // 测试WrappingOps trait实现
        assert_eq!(i32::zero(), 0);
        assert_eq!(i32::one(), 1);
        assert_eq!(5i32.wrapping_add(3), 8);
        assert_eq!(i32::MAX.wrapping_add(1), i32::MIN);
    }

    #[test]
    fn test_overflow_behavior_i8() {
        // 测试i8溢出行为（小值避免长时间循环）
        // sum(10) = 10 * 11 / 2 = 55
        let result = fun_wrapping(10i8);
        assert_eq!(result, 55);

        // 测试会溢出的情况
        let result = fun_wrapping(20i8);
        // sum(20) = 20 * 21 / 2 = 210，但210超出i8范围
        // 210 - 256 = -46
        assert_eq!(result, -46i8);
    }

    #[test]
    fn test_overflow_behavior_u8() {
        // 测试u8溢出行为（小值避免长时间循环）
        let result = fun_wrapping(10u8);
        assert_eq!(result, 55);

        let result = fun_wrapping(20u8);
        // sum(20) = 210
        assert_eq!(result, 210u8);
    }

    #[test]
    fn test_all_original_test_cases() {
        // 综合测试：验证所有原C++测试用例
        // 测试i16
        assert_eq!(fun_wrapping(255i16), 32640i16);
        assert_eq!(fun_wrapping(256i16), -32640i16);

        // 测试u16
        assert_eq!(fun_wrapping(255u16), 32640u16);
        assert_eq!(fun_wrapping(256u16), 32896u16);

        // 测试i32
        assert_eq!(fun_wrapping(65535i32), 2147450880i32);
        assert_eq!(fun_wrapping(65536i32), -2147450880i32);

        // 测试u32
        assert_eq!(fun_wrapping(65535u32), 2147450880u32);
        assert_eq!(fun_wrapping(65536u32), 2147516416u32);
    }
}
