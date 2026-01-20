// 5-b2: Light bulb toggle problem - find which bulbs remain on
// Original: 2052526 信15 白俊豪
//
// 问题描述: 100个灯泡初始全灭，100个人依次操作：
// 第i个人切换所有编号为i的倍数的灯泡状态
// 求最后哪些灯泡是亮的
//
// Rust改进:
// 1. 使用bool类型表示状态，比整数更清晰且内存效率更高
// 2. 使用Vec而非固定数组，更灵活且支持任意数量的灯泡
// 3. 提取核心逻辑为纯函数，便于测试和复用
// 4. 使用迭代器链式调用，代码更简洁优雅
// 5. 数学优化：只有完全平方数的灯泡最终会亮（因数个数为奇数）
// 6. 添加两种实现：模拟算法和数学优化算法

/// 模拟灯泡切换过程
///
/// # Arguments
/// * `n` - 灯泡数量
///
/// # Returns
/// * `Vec<usize>` - 最终亮着的灯泡编号（1-based）
///
/// # Algorithm
/// 对于每个人i (1..=n)，切换所有编号为i的倍数的灯泡
#[cfg(test)]
fn simulate_light_toggle(n: usize) -> Vec<usize> {
    // Rust改进: 使用vec![false; n]初始化，比C++的循环赋值更简洁
    let mut lights = vec![false; n];

    // 模拟每个人的操作
    for person in 1..=n {
        // Rust改进: 使用step_by迭代器，比手动计算索引更清晰
        for bulb_idx in (person - 1..n).step_by(person) {
            lights[bulb_idx] = !lights[bulb_idx];
        }
    }

    // Rust改进: 使用enumerate + filter_map链式调用收集结果
    // 比C++的循环+if更函数式，避免可变状态
    lights
        .iter()
        .enumerate()
        .filter_map(|(idx, &is_on)| {
            if is_on {
                Some(idx + 1) // 转换为1-based编号
            } else {
                None
            }
        })
        .collect()
}

/// 数学优化版本：直接计算完全平方数
///
/// # Arguments
/// * `n` - 灯泡数量
///
/// # Returns
/// * `Vec<usize>` - 最终亮着的灯泡编号（1-based）
///
/// # Mathematical Insight
/// 一个灯泡被切换的次数等于其编号的因数个数。
/// 只有完全平方数的因数个数为奇数（因为平方根只计算一次）。
/// 因此，只有完全平方数编号的灯泡最终会亮。
///
/// 例如：
/// - 灯泡1: 因数[1] -> 切换1次 -> 亮
/// - 灯泡4: 因数[1,2,4] -> 切换3次 -> 亮
/// - 灯泡6: 因数[1,2,3,6] -> 切换4次 -> 灭
fn optimized_light_toggle(n: usize) -> Vec<usize> {
    // Rust改进: 使用map + take_while生成完全平方数序列
    // 时间复杂度从O(n²)降到O(√n)
    (1..)
        .map(|i| i * i)
        .take_while(|&square| square <= n)
        .collect()
}

fn main() {
    const N: usize = 100;

    // 使用优化算法（与原C++输出一致）
    let result = optimized_light_toggle(N);

    // Rust改进: 使用迭代器处理输出格式，避免特殊处理最后一个元素
    for (i, &bulb) in result.iter().enumerate() {
        if i < result.len() - 1 {
            print!("{} ", bulb);
        } else {
            print!("{}", bulb);
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_small_cases() {
        // 测试小规模情况
        assert_eq!(simulate_light_toggle(1), vec![1]);
        assert_eq!(simulate_light_toggle(4), vec![1, 4]);
        assert_eq!(simulate_light_toggle(9), vec![1, 4, 9]);
        assert_eq!(simulate_light_toggle(10), vec![1, 4, 9]);
    }

    #[test]
    fn test_optimized_small_cases() {
        // 测试优化算法的小规模情况
        assert_eq!(optimized_light_toggle(1), vec![1]);
        assert_eq!(optimized_light_toggle(4), vec![1, 4]);
        assert_eq!(optimized_light_toggle(9), vec![1, 4, 9]);
        assert_eq!(optimized_light_toggle(10), vec![1, 4, 9]);
    }

    #[test]
    fn test_both_algorithms_match() {
        // 测试两种算法在不同规模下结果一致
        for n in [1, 5, 10, 25, 50, 100] {
            assert_eq!(
                simulate_light_toggle(n),
                optimized_light_toggle(n),
                "两种算法在n={}时结果应该一致",
                n
            );
        }
    }

    #[test]
    fn test_100_bulbs() {
        // 测试原题目的100个灯泡情况
        let result = optimized_light_toggle(100);

        // 100以内的完全平方数: 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
        assert_eq!(result.len(), 10);
        assert_eq!(result, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn test_perfect_squares_property() {
        // 验证数学性质：结果都是完全平方数
        let result = simulate_light_toggle(100);

        for &num in &result {
            let sqrt = (num as f64).sqrt() as usize;
            assert_eq!(sqrt * sqrt, num, "{} 应该是完全平方数", num);
        }
    }

    #[test]
    fn test_edge_case_zero() {
        // 测试边界情况：0个灯泡
        assert_eq!(simulate_light_toggle(0), Vec::<usize>::new());
        assert_eq!(optimized_light_toggle(0), Vec::<usize>::new());
    }

    #[test]
    fn test_large_scale() {
        // 测试大规模情况（1000个灯泡）
        let result = optimized_light_toggle(1000);

        // 1000以内有31个完全平方数 (31² = 961, 32² = 1024)
        assert_eq!(result.len(), 31);
        assert_eq!(result[0], 1);
        assert_eq!(result[30], 961);
    }

    #[test]
    fn test_toggle_count_property() {
        // 验证切换次数等于因数个数的性质
        let n = 12;
        let mut toggle_count = vec![0; n];

        for person in 1..=n {
            for bulb_idx in (person - 1..n).step_by(person) {
                toggle_count[bulb_idx] += 1;
            }
        }

        // 12的因数: 1,2,3,4,6,12 -> 6个（偶数）-> 最终灭
        assert_eq!(toggle_count[11], 6);
        assert_eq!(toggle_count[11] % 2, 0);

        // 9的因数: 1,3,9 -> 3个（奇数）-> 最终亮
        assert_eq!(toggle_count[8], 3);
        assert_eq!(toggle_count[8] % 2, 1);
    }
}
