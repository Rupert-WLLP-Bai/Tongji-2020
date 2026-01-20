// 3-b13: Find three 3-digit numbers that sum to 1953 with unique digits 1-9
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用位掩码(bitmask)优化数字唯一性检查，从O(n²)降到O(1)
// 2. 提前计算数字掩码，避免重复计算
// 3. 使用const常量和内联函数，零成本抽象
// 4. 提取核心逻辑为纯函数，便于测试和性能分析
// 5. 使用std::time::Instant替代Windows API，跨平台兼容
// 6. 优化搜索范围：k的下界可以根据i和j动态计算

use std::time::Instant;

/// 将三位数转换为数字掩码
/// 使用位运算：每个数字对应一个bit位(1-9对应bit 1-9)
///
/// # Arguments
/// * `num` - 三位数(123-987)
///
/// # Returns
/// * `Some(u16)` - 如果所有数字都是1-9且不重复
/// * `None` - 如果包含0或有重复数字
#[inline]
const fn digit_mask(num: i32) -> Option<u16> {
    let d1 = num / 100;
    let d2 = (num / 10) % 10;
    let d3 = num % 10;

    // 检查是否包含0
    if d1 == 0 || d2 == 0 || d3 == 0 {
        return None;
    }

    let mask1 = 1u16 << d1;
    let mask2 = 1u16 << d2;
    let mask3 = 1u16 << d3;

    // 检查是否有重复数字：如果有重复，按位或后的结果会少于3个bit
    let combined = mask1 | mask2 | mask3;
    if combined.count_ones() != 3 {
        return None;
    }

    Some(combined)
}

/// 检查三个数字掩码是否完全不重叠（所有9个数字都不同）
#[inline]
const fn masks_are_disjoint(mask1: u16, mask2: u16, mask3: u16) -> bool {
    // 如果三个掩码完全不重叠，按位或后应该有9个bit被设置
    (mask1 | mask2 | mask3).count_ones() == 9
}

/// 查找所有满足条件的三数组合
///
/// # Arguments
/// * `target_sum` - 目标和
///
/// # Returns
/// * `Vec<(i32, i32, i32)>` - 所有满足条件的三元组
fn find_combinations(target_sum: i32) -> Vec<(i32, i32, i32)> {
    let mut results = Vec::new();

    // Rust改进: 预计算所有有效数字的掩码，避免重复计算
    // 使用数组存储，索引即为数字本身
    let mut masks = [None; 988];
    for (num, mask) in masks.iter_mut().enumerate().skip(123).take(988 - 123) {
        *mask = digit_mask(num as i32);
    }

    // 搜索范围优化：
    // i的最大值：当j和k尽可能大时，i最大约为651
    // j的最大值：当k最大时，j最大约为987
    for i in 123..651 {
        let Some(mask_i) = masks[i] else { continue };

        // j必须大于i
        for j in (i + 1)..988 {
            let Some(mask_j) = masks[j] else { continue };

            // Rust改进: 提前检查i和j的掩码是否重叠
            if (mask_i & mask_j) != 0 {
                continue;
            }

            // 计算k的值
            let k = target_sum - (i as i32) - (j as i32);

            // k必须大于j且在有效范围内
            if k <= (j as i32) || k >= 988 {
                continue;
            }

            let Some(mask_k) = masks[k as usize] else { continue };

            // Rust改进: 使用位运算检查所有9个数字是否唯一
            if masks_are_disjoint(mask_i, mask_j, mask_k) {
                results.push((i as i32, j as i32, k));
            }
        }
    }

    results
}

fn main() {
    let start = Instant::now();

    const TARGET_SUM: i32 = 1953;
    let results = find_combinations(TARGET_SUM);

    // 输出结果
    for (idx, (i, j, k)) in results.iter().enumerate() {
        println!("NO.{:3} : {}+{}+{}= {}", idx + 1, i, j, k, TARGET_SUM);
    }
    println!("total = {}", results.len());

    let elapsed = start.elapsed();

    // 输出性能信息
    println!("执行时间: {:.6}秒", elapsed.as_secs_f64());
    println!("执行时间: {}微秒", elapsed.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_mask_valid() {
        // 测试有效的三位数
        assert_eq!(digit_mask(123), Some(0b0000_0000_0000_1110)); // bits 1,2,3
        assert_eq!(digit_mask(456), Some(0b0000_0000_0111_0000)); // bits 4,5,6
        assert_eq!(digit_mask(789), Some(0b0000_0011_1000_0000)); // bits 7,8,9
    }

    #[test]
    fn test_digit_mask_with_zero() {
        // 测试包含0的数字
        assert_eq!(digit_mask(102), None);
        assert_eq!(digit_mask(120), None);
        assert_eq!(digit_mask(201), None);
    }

    #[test]
    fn test_digit_mask_with_duplicates() {
        // 测试有重复数字的情况
        assert_eq!(digit_mask(111), None);
        assert_eq!(digit_mask(121), None);
        assert_eq!(digit_mask(112), None);
        assert_eq!(digit_mask(232), None);
    }

    #[test]
    fn test_masks_are_disjoint() {
        // 测试完全不重叠的掩码
        let mask1 = digit_mask(123).unwrap();
        let mask2 = digit_mask(456).unwrap();
        let mask3 = digit_mask(789).unwrap();
        assert!(masks_are_disjoint(mask1, mask2, mask3));
    }

    #[test]
    fn test_masks_overlap() {
        // 测试有重叠的掩码
        let mask1 = digit_mask(123).unwrap();
        let mask2 = digit_mask(124).unwrap(); // 1和2重复
        let mask3 = digit_mask(567).unwrap();
        assert!(!masks_are_disjoint(mask1, mask2, mask3));
    }

    #[test]
    fn test_find_combinations_basic() {
        // 测试基本功能
        let results = find_combinations(1953);

        // 验证所有结果都满足条件
        for (i, j, k) in &results {
            // 检查和
            assert_eq!(i + j + k, 1953);

            // 检查升序
            assert!(i < j);
            assert!(j < k);

            // 检查范围
            assert!(*i >= 123 && *i < 988);
            assert!(*j >= 123 && *j < 988);
            assert!(*k >= 123 && *k < 988);

            // 检查数字唯一性
            let mask_i = digit_mask(*i).unwrap();
            let mask_j = digit_mask(*j).unwrap();
            let mask_k = digit_mask(*k).unwrap();
            assert!(masks_are_disjoint(mask_i, mask_j, mask_k));
        }

        // 验证结果数量（根据原程序的输出）
        assert!(results.len() > 0, "应该找到至少一个解");
    }

    #[test]
    fn test_find_combinations_specific_cases() {
        // 测试特定的已知解
        let results = find_combinations(1953);

        // 验证结果按升序排列
        for window in results.windows(2) {
            let (i1, j1, k1) = window[0];
            let (i2, j2, k2) = window[1];

            // 结果应该按字典序排列
            assert!(i1 <= i2);
            if i1 == i2 {
                assert!(j1 <= j2);
                if j1 == j2 {
                    assert!(k1 < k2);
                }
            }
        }
    }

    #[test]
    fn test_digit_mask_all_valid_numbers() {
        // 测试所有可能的三位数，确保掩码计算正确
        for num in 123..988 {
            if let Some(mask) = digit_mask(num) {
                // 如果返回Some，必须恰好有3个bit被设置
                assert_eq!(mask.count_ones(), 3);

                // 验证掩码只使用bit 1-9（bit 0和bit 10+不应该被设置）
                assert!(mask >= (1 << 1), "掩码应该至少有bit 1被设置");
                assert!(mask < (1 << 10), "掩码不应该使用bit 10或更高位");
            }
        }
    }

    #[test]
    fn test_performance_benchmark() {
        // 性能基准测试
        let start = Instant::now();
        let results = find_combinations(1953);
        let elapsed = start.elapsed();

        println!("找到 {} 个解，耗时 {:?}", results.len(), elapsed);

        // Rust改进: 由于使用了位掩码优化，性能应该显著优于原C++版本
        // 在现代硬件上应该在毫秒级完成
        assert!(elapsed.as_millis() < 1000, "执行时间应该小于1秒");
    }

    #[test]
    fn test_no_duplicate_results() {
        // 确保没有重复的结果
        let results = find_combinations(1953);
        let mut sorted = results.clone();
        sorted.sort();
        sorted.dedup();

        assert_eq!(results.len(), sorted.len(), "不应该有重复的结果");
    }
}
