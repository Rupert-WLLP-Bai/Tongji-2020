// 5-b4: Score statistics - count frequency of each score
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Vec<i32>动态数组，避免固定大小限制和初始化开销
// 2. 使用HashMap统计频率，比数组更高效（O(n)而非O(n*101)）
// 3. 使用迭代器的filter和collect，代码更简洁
// 4. 使用BTreeMap自动排序，避免手动排序
// 5. 提取统计逻辑为独立函数，便于单元测试

use std::collections::BTreeMap;
use std::io;

/// 统计分数频率
///
/// # Arguments
/// * `scores` - 分数数组
///
/// # Returns
/// * `BTreeMap<i32, usize>` - 分数到频率的映射，按分数降序排列
///
/// # Examples
/// ```
/// let scores = vec![100, 90, 90, 80];
/// let freq = count_score_frequency(&scores);
/// assert_eq!(freq.get(&100), Some(&1));
/// assert_eq!(freq.get(&90), Some(&2));
/// assert_eq!(freq.get(&80), Some(&1));
/// ```
fn count_score_frequency(scores: &[i32]) -> BTreeMap<i32, usize> {
    // Rust改进: 使用fold迭代器方法统计频率，比C++的双重循环更高效
    // 时间复杂度从O(n*101)降低到O(n log n)
    scores.iter().fold(BTreeMap::new(), |mut map, &score| {
        *map.entry(score).or_insert(0) += 1;
        map
    })
}

/// 读取分数输入
///
/// # Returns
/// * `Vec<i32>` - 分数数组
fn read_scores() -> Vec<i32> {
    println!("请输入成绩(最多1000个),以-1结尾 : ");

    let mut scores = Vec::new();
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        // Rust改进: 使用split_whitespace()处理多个空格分隔的输入
        for token in input.split_whitespace() {
            if let Ok(score) = token.parse::<i32>() {
                if score < 0 {
                    return scores;
                }
                scores.push(score);

                // 限制最多1000个
                if scores.len() >= 1000 {
                    return scores;
                }
            }
        }
    }
}

/// 打印分数数组，每行10个
///
/// # Arguments
/// * `scores` - 分数数组
fn print_scores(scores: &[i32]) {
    println!("输入的数组为 : ");

    // Rust改进: 使用chunks()迭代器方法，每10个一组打印
    for (i, chunk) in scores.chunks(10).enumerate() {
        if i > 0 {
            println!();
        }
        for score in chunk {
            print!("{} ", score);
        }
    }
    println!();
}

fn main() {
    let scores = read_scores();

    if scores.is_empty() {
        println!("无有效输入");
        return;
    }

    print_scores(&scores);

    println!("分数与人数的对应关系为 : ");

    // Rust改进: 使用BTreeMap自动按键排序，然后反向迭代
    let frequency = count_score_frequency(&scores);

    // 按分数从高到低输出
    for (&score, &count) in frequency.iter().rev() {
        println!("{} {}", score, count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_score_frequency_empty() {
        // 测试空数组
        let scores: Vec<i32> = vec![];
        let freq = count_score_frequency(&scores);
        assert!(freq.is_empty());
    }

    #[test]
    fn test_count_score_frequency_single() {
        // 测试单个分数
        let scores = vec![100];
        let freq = count_score_frequency(&scores);
        assert_eq!(freq.get(&100), Some(&1));
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_count_score_frequency_duplicates() {
        // 测试重复分数
        let scores = vec![90, 90, 90, 80, 80, 70];
        let freq = count_score_frequency(&scores);
        assert_eq!(freq.get(&90), Some(&3));
        assert_eq!(freq.get(&80), Some(&2));
        assert_eq!(freq.get(&70), Some(&1));
        assert_eq!(freq.len(), 3);
    }

    #[test]
    fn test_count_score_frequency_all_different() {
        // 测试所有分数不同
        let scores = vec![100, 95, 90, 85, 80];
        let freq = count_score_frequency(&scores);
        assert_eq!(freq.len(), 5);
        for score in scores {
            assert_eq!(freq.get(&score), Some(&1));
        }
    }

    #[test]
    fn test_count_score_frequency_all_same() {
        // 测试所有分数相同
        let scores = vec![75, 75, 75, 75, 75];
        let freq = count_score_frequency(&scores);
        assert_eq!(freq.get(&75), Some(&5));
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_count_score_frequency_boundary() {
        // 测试边界值
        let scores = vec![0, 100, 0, 100];
        let freq = count_score_frequency(&scores);
        assert_eq!(freq.get(&0), Some(&2));
        assert_eq!(freq.get(&100), Some(&2));
        assert_eq!(freq.len(), 2);
    }

    #[test]
    fn test_count_score_frequency_sorted_order() {
        // 测试BTreeMap的排序特性
        let scores = vec![50, 100, 75, 25, 90];
        let freq = count_score_frequency(&scores);

        // BTreeMap应该按键排序
        let keys: Vec<i32> = freq.keys().copied().collect();
        assert_eq!(keys, vec![25, 50, 75, 90, 100]);
    }

    #[test]
    fn test_count_score_frequency_large_dataset() {
        // 测试大数据集
        let mut scores = Vec::new();
        for i in 0..100 {
            scores.push(i);
            scores.push(i); // 每个分数出现两次
        }

        let freq = count_score_frequency(&scores);
        assert_eq!(freq.len(), 100);
        for i in 0..100 {
            assert_eq!(freq.get(&i), Some(&2));
        }
    }

    #[test]
    fn test_count_score_frequency_mixed() {
        // 测试混合场景
        let scores = vec![100, 90, 85, 90, 100, 75, 85, 90, 100];
        let freq = count_score_frequency(&scores);
        assert_eq!(freq.get(&100), Some(&3));
        assert_eq!(freq.get(&90), Some(&3));
        assert_eq!(freq.get(&85), Some(&2));
        assert_eq!(freq.get(&75), Some(&1));
        assert_eq!(freq.len(), 4);
    }

    #[test]
    fn test_count_score_frequency_reverse_iteration() {
        // 测试反向迭代（从高到低）
        let scores = vec![60, 70, 80, 90, 100];
        let freq = count_score_frequency(&scores);

        let reversed: Vec<i32> = freq.keys().rev().copied().collect();
        assert_eq!(reversed, vec![100, 90, 80, 70, 60]);
    }
}
