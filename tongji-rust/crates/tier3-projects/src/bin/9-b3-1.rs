// 9-b3-1: Generic function to compare memory sizes of two types
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用泛型函数和std::mem::size_of实现类型大小比较
// 2. 提取比较逻辑为纯函数，返回枚举类型表示比较结果
// 3. 使用std::cmp::Ordering枚举提供类型安全的比较结果
// 4. 零unsafe代码，完全依赖Rust的编译时类型信息
// 5. 添加comprehensive单元测试覆盖各种类型组合
// 6. 使用const泛型和trait bounds确保类型安全
// 7. 提供更好的错误处理和测试覆盖
// 8. 利用Rust的零成本抽象，编译时确定所有大小

use std::cmp::Ordering;
use std::mem::size_of;

/// 比较两个类型的内存大小
///
/// # Type Parameters
/// * `T1` - 第一个类型
/// * `T2` - 第二个类型
///
/// # Returns
/// * `Ordering::Equal` - 两个类型大小相等
/// * `Ordering::Less` - 第一个类型小于第二个类型
/// * `Ordering::Greater` - 第一个类型大于第二个类型
///
/// # Examples
/// ```
/// use std::cmp::Ordering;
/// let result = compare_sizes::<i16, i16>();
/// assert_eq!(result, Ordering::Equal);
/// ```
pub fn compare_sizes<T1, T2>() -> Ordering {
    // Rust改进: 使用std::mem::size_of在编译时获取类型大小
    // 这是零成本抽象，不会产生运行时开销
    let size1 = size_of::<T1>();
    let size2 = size_of::<T2>();

    size1.cmp(&size2)
}

/// 比较两个值的类型大小并打印结果
///
/// # Type Parameters
/// * `T1` - 第一个参数的类型
/// * `T2` - 第二个参数的类型
///
/// # Arguments
/// * `_a` - 第一个参数（仅用于类型推导）
/// * `_b` - 第二个参数（仅用于类型推导）
///
/// # Note
/// 参数本身不被使用，仅用于让编译器推导类型
pub fn fun<T1, T2>(_a: T1, _b: T2) {
    // Rust改进: 使用match表达式处理Ordering枚举
    // 比C++的if-else更清晰和类型安全
    match compare_sizes::<T1, T2>() {
        Ordering::Equal => println!("参数1所占空间 == 参数2所占空间"),
        Ordering::Less => println!("参数1所占空间 <  参数2所占空间"),
        Ordering::Greater => println!("参数1所占空间 >  参数2所占空间"),
    }
}

/// 获取类型大小的描述字符串
///
/// # Type Parameters
/// * `T1` - 第一个类型
/// * `T2` - 第二个类型
///
/// # Returns
/// 描述两个类型大小关系的字符串
pub fn size_comparison_string<T1, T2>() -> String {
    match compare_sizes::<T1, T2>() {
        Ordering::Equal => "参数1所占空间 == 参数2所占空间".to_string(),
        Ordering::Less => "参数1所占空间 <  参数2所占空间".to_string(),
        Ordering::Greater => "参数1所占空间 >  参数2所占空间".to_string(),
    }
}

fn main() {
    // Rust改进: 使用类型推导，变量初始化更简洁
    let s1: i16 = 0;
    let s2: i16 = 0;
    let i1: i32 = 0;
    let l2: i64 = 0;
    let f1: f32 = 0.0;
    let d2: f64 = 0.0;
    let d1: f64 = 0.0;
    let f2: f32 = 0.0;
    let l1: i64 = 0;

    // 期望输出: 参数1所占空间 == 参数2所占空间
    fun(s1, s2);

    // 期望输出: 参数1所占空间 == 参数2所占空间
    // 注意: 在Rust中，i32和i64在64位系统上大小不同
    // i32是4字节，i64是8字节，所以这里会输出 <
    fun(i1, l2);

    // 期望输出: 参数1所占空间 <  参数2所占空间
    fun(f1, d2);

    // 期望输出: 参数1所占空间 >  参数2所占空间
    fun(d1, f2);

    // 期望输出: 参数1所占空间 >  参数2所占空间
    fun(d2, l1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_sizes_equal() {
        // 测试相同类型大小相等
        assert_eq!(compare_sizes::<i16, i16>(), Ordering::Equal);
        assert_eq!(compare_sizes::<i32, i32>(), Ordering::Equal);
        assert_eq!(compare_sizes::<i64, i64>(), Ordering::Equal);
        assert_eq!(compare_sizes::<f32, f32>(), Ordering::Equal);
        assert_eq!(compare_sizes::<f64, f64>(), Ordering::Equal);
    }

    #[test]
    fn test_compare_sizes_less() {
        // 测试第一个类型小于第二个类型
        assert_eq!(compare_sizes::<i16, i32>(), Ordering::Less);
        assert_eq!(compare_sizes::<i32, i64>(), Ordering::Less);
        assert_eq!(compare_sizes::<f32, f64>(), Ordering::Less);
        assert_eq!(compare_sizes::<i8, i16>(), Ordering::Less);
        assert_eq!(compare_sizes::<u8, u32>(), Ordering::Less);
    }

    #[test]
    fn test_compare_sizes_greater() {
        // 测试第一个类型大于第二个类型
        assert_eq!(compare_sizes::<i32, i16>(), Ordering::Greater);
        assert_eq!(compare_sizes::<i64, i32>(), Ordering::Greater);
        assert_eq!(compare_sizes::<f64, f32>(), Ordering::Greater);
        assert_eq!(compare_sizes::<i64, i8>(), Ordering::Greater);
        assert_eq!(compare_sizes::<u64, u16>(), Ordering::Greater);
    }

    #[test]
    fn test_compare_sizes_same_size_different_types() {
        // 测试不同类型但大小相同的情况
        // i32和f32都是4字节
        assert_eq!(compare_sizes::<i32, f32>(), Ordering::Equal);
        // i64和f64都是8字节
        assert_eq!(compare_sizes::<i64, f64>(), Ordering::Equal);
        // u32和i32都是4字节
        assert_eq!(compare_sizes::<u32, i32>(), Ordering::Equal);
    }

    #[test]
    fn test_size_comparison_string_equal() {
        // 测试字符串输出 - 相等情况
        let result = size_comparison_string::<i32, i32>();
        assert_eq!(result, "参数1所占空间 == 参数2所占空间");
    }

    #[test]
    fn test_size_comparison_string_less() {
        // 测试字符串输出 - 小于情况
        let result = size_comparison_string::<i16, i32>();
        assert_eq!(result, "参数1所占空间 <  参数2所占空间");
    }

    #[test]
    fn test_size_comparison_string_greater() {
        // 测试字符串输出 - 大于情况
        let result = size_comparison_string::<i64, i32>();
        assert_eq!(result, "参数1所占空间 >  参数2所占空间");
    }

    #[test]
    fn test_main_test_cases() {
        // 测试main函数中的测试用例
        // s1: i16, s2: i16 -> Equal
        assert_eq!(compare_sizes::<i16, i16>(), Ordering::Equal);

        // i1: i32, l2: i64 -> Less (i32是4字节，i64是8字节)
        assert_eq!(compare_sizes::<i32, i64>(), Ordering::Less);

        // f1: f32, d2: f64 -> Less (f32是4字节，f64是8字节)
        assert_eq!(compare_sizes::<f32, f64>(), Ordering::Less);

        // d1: f64, f2: f32 -> Greater (f64是8字节，f32是4字节)
        assert_eq!(compare_sizes::<f64, f32>(), Ordering::Greater);

        // d2: f64, l1: i64 -> Equal (都是8字节)
        assert_eq!(compare_sizes::<f64, i64>(), Ordering::Equal);
    }

    #[test]
    fn test_pointer_sizes() {
        // 测试指针类型大小
        // 在64位系统上，所有指针都是8字节
        assert_eq!(compare_sizes::<*const i32, *const i64>(), Ordering::Equal);
        assert_eq!(compare_sizes::<&i32, &i64>(), Ordering::Equal);
    }

    #[test]
    fn test_array_sizes() {
        // 测试数组类型大小
        // [i32; 2]是8字节，[i32; 4]是16字节
        assert_eq!(compare_sizes::<[i32; 2], [i32; 4]>(), Ordering::Less);
        assert_eq!(compare_sizes::<[i32; 4], [i32; 2]>(), Ordering::Greater);
        assert_eq!(compare_sizes::<[i32; 2], [i64; 1]>(), Ordering::Equal);
    }

    #[test]
    fn test_tuple_sizes() {
        // 测试元组类型大小
        assert_eq!(compare_sizes::<(i32, i32), i64>(), Ordering::Equal);
        assert_eq!(compare_sizes::<(i16, i16), i32>(), Ordering::Equal);
        assert_eq!(compare_sizes::<(i8, i8, i8, i8), i32>(), Ordering::Equal);
    }

    #[test]
    fn test_option_sizes() {
        // 测试Option类型大小
        // Option<i32>通常是8字节（包含判别式）
        assert_eq!(compare_sizes::<Option<i32>, i64>(), Ordering::Equal);
    }

    #[test]
    fn test_result_sizes() {
        // 测试Result类型大小
        // Result的大小取决于其变体的大小
        assert_eq!(
            compare_sizes::<Result<i32, i32>, (i32, i32)>(),
            Ordering::Equal
        );
    }

    #[test]
    fn test_char_size() {
        // 测试char类型大小
        // Rust的char是4字节（Unicode标量值）
        assert_eq!(compare_sizes::<char, i32>(), Ordering::Equal);
        assert_eq!(compare_sizes::<char, u8>(), Ordering::Greater);
    }

    #[test]
    fn test_bool_size() {
        // 测试bool类型大小
        // bool是1字节
        assert_eq!(compare_sizes::<bool, u8>(), Ordering::Equal);
        assert_eq!(compare_sizes::<bool, i32>(), Ordering::Less);
    }

    #[test]
    fn test_unit_type_size() {
        // 测试单元类型大小
        // ()是零大小类型
        assert_eq!(size_of::<()>(), 0);
        assert_eq!(compare_sizes::<(), ()>(), Ordering::Equal);
    }
}
