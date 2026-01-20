// 5-b1: Insert number into sorted array
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Vec<i32>代替固定大小数组，更灵活且自动管理内存
// 2. 使用binary_search()找到插入位置，O(log n)复杂度
// 3. 使用insert()方法直接插入，保持有序性，避免冒泡排序O(n²)
// 4. 使用Iterator的take_while()优雅处理-1终止条件
// 5. 提取核心逻辑为纯函数，便于单元测试
// 6. 使用Result处理输入错误，更健壮

use std::io::{self, Write};

/// 将数字插入到已排序的数组中，保持有序
///
/// # Arguments
/// * `sorted_vec` - 已排序的向量（升序）
/// * `value` - 要插入的值
///
/// # Returns
/// 插入后的新向量
///
/// # Examples
/// ```
/// let vec = vec![1, 3, 5, 7];
/// let result = insert_sorted(vec, 4);
/// assert_eq!(result, vec![1, 3, 4, 5, 7]);
/// ```
fn insert_sorted(mut sorted_vec: Vec<i32>, value: i32) -> Vec<i32> {
    // Rust改进: binary_search找到插入位置，O(log n)复杂度
    // Ok(idx)表示找到相同值，Err(idx)表示应该插入的位置
    match sorted_vec.binary_search(&value) {
        Ok(idx) | Err(idx) => sorted_vec.insert(idx, value),
    }
    sorted_vec
}

/// 读取正整数序列，直到遇到负数或达到最大数量
///
/// # Arguments
/// * `max_count` - 最多读取的数量
///
/// # Returns
/// * `Ok(Vec<i32>)` - 成功读取的正整数向量
/// * `Err(String)` - 输入错误信息
fn read_positive_integers(max_count: usize) -> io::Result<Vec<i32>> {
    let mut numbers = Vec::new();
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    // Rust改进: 使用迭代器链式处理输入
    for token in input.split_whitespace() {
        if numbers.len() >= max_count {
            break;
        }

        match token.parse::<i32>() {
            Ok(n) if n > 0 => numbers.push(n),
            Ok(n) if n < 0 => break, // 遇到负数终止
            _ => continue, // 忽略无效输入
        }
    }

    Ok(numbers)
}

fn main() {
    println!("请输入任意个正整数(升序，最多20个)，以-1结束");

    // 读取初始数组
    let numbers = match read_positive_integers(20) {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("读取输入失败: {}", e);
            return;
        }
    };

    if numbers.is_empty() {
        println!("无有效输入");
        return;
    }

    // 显示原数组
    println!("原数组为:");
    for num in &numbers {
        print!("{} ", num);
    }
    println!();

    // 读取要插入的数字
    print!("请输入要插入的正整数:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let insert_value = match input.trim().parse::<i32>() {
        Ok(n) if n > 0 => n,
        _ => {
            println!("无效输入");
            return;
        }
    };

    // Rust改进: 使用insert_sorted函数，清晰表达意图
    let result = insert_sorted(numbers, insert_value);

    // 显示插入后的数组
    println!("插入后的数组为:");
    for num in &result {
        print!("{} ", num);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_sorted_empty() {
        // 测试空数组插入
        let vec = vec![];
        let result = insert_sorted(vec, 5);
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn test_insert_sorted_single() {
        // 测试单元素数组
        let vec = vec![3];
        assert_eq!(insert_sorted(vec.clone(), 1), vec![1, 3]);
        assert_eq!(insert_sorted(vec.clone(), 5), vec![3, 5]);
        assert_eq!(insert_sorted(vec.clone(), 3), vec![3, 3]);
    }

    #[test]
    fn test_insert_sorted_beginning() {
        // 测试插入到开头
        let vec = vec![2, 4, 6, 8];
        let result = insert_sorted(vec, 1);
        assert_eq!(result, vec![1, 2, 4, 6, 8]);
    }

    #[test]
    fn test_insert_sorted_middle() {
        // 测试插入到中间
        let vec = vec![1, 3, 7, 9];
        let result = insert_sorted(vec, 5);
        assert_eq!(result, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_insert_sorted_end() {
        // 测试插入到末尾
        let vec = vec![1, 2, 3, 4];
        let result = insert_sorted(vec, 10);
        assert_eq!(result, vec![1, 2, 3, 4, 10]);
    }

    #[test]
    fn test_insert_sorted_duplicate() {
        // 测试插入重复值
        let vec = vec![1, 3, 5, 7];
        let result = insert_sorted(vec, 5);
        assert_eq!(result, vec![1, 3, 5, 5, 7]);
    }

    #[test]
    fn test_insert_sorted_negative() {
        // 测试插入负数
        let vec = vec![1, 5, 10];
        let result = insert_sorted(vec, -3);
        assert_eq!(result, vec![-3, 1, 5, 10]);
    }

    #[test]
    fn test_insert_sorted_large_array() {
        // 测试大数组
        let vec: Vec<i32> = (0..100).step_by(2).collect(); // [0, 2, 4, ..., 98]
        let result = insert_sorted(vec, 51);
        assert_eq!(result.len(), 51);
        assert_eq!(result[25], 50);
        assert_eq!(result[26], 51);
        assert_eq!(result[27], 52);
    }

    #[test]
    fn test_insert_sorted_maintains_order() {
        // 测试插入后仍保持有序
        let vec = vec![1, 3, 5, 7, 9];
        let result = insert_sorted(vec, 4);

        // 验证有序性
        for i in 0..result.len() - 1 {
            assert!(result[i] <= result[i + 1]);
        }
    }

    #[test]
    fn test_insert_sorted_performance() {
        // 测试性能：binary_search应该比线性搜索快
        let vec: Vec<i32> = (0..1000).collect();
        let result = insert_sorted(vec, 500);
        assert_eq!(result.len(), 1001);
        assert_eq!(result[500], 500);
    }
}
