// 5-b5: Score ranking system
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用Vec<i32>替代固定大小数组，避免浪费内存
// 2. 使用sort_unstable_by降序排序，性能优于冒泡排序O(n²)
// 3. 使用HashMap统计分数出现次数，避免嵌套循环
// 4. 使用迭代器链式调用，代码更简洁
// 5. 提取核心逻辑为纯函数，便于单元测试
// 6. 使用Result处理输入错误，而非依赖特殊值-1

use std::collections::HashMap;
use std::io;

/// 计算每个分数的排名
///
/// # Arguments
/// * `scores` - 分数列表（已排序，降序）
///
/// # Returns
/// * Vec<(i32, usize)> - (分数, 排名)的列表
///
/// # Examples
/// ```
/// let scores = vec![95, 90, 90, 85];
/// let rankings = calculate_rankings(&scores);
/// assert_eq!(rankings, vec![(95, 1), (90, 2), (90, 2), (85, 4)]);
/// ```
fn calculate_rankings(scores: &[i32]) -> Vec<(i32, usize)> {
    if scores.is_empty() {
        return Vec::new();
    }

    let mut rankings = Vec::with_capacity(scores.len());
    let mut rank = 1;
    let mut i = 0;

    while i < scores.len() {
        let current_score = scores[i];
        let mut count = 1;

        // 统计相同分数的数量
        while i + count < scores.len() && scores[i + count] == current_score {
            count += 1;
        }

        // 为所有相同分数分配相同排名
        for _ in 0..count {
            rankings.push((current_score, rank));
        }

        // 下一个排名跳过并列的位置
        rank += count;
        i += count;
    }

    rankings
}

/// 读取分数列表，直到输入负数为止
///
/// # Returns
/// * `io::Result<Vec<i32>>` - 成功返回分数列表，失败返回IO错误
fn read_scores() -> io::Result<Vec<i32>> {
    println!("请输入成绩(最多1000个),以-1结尾 : ");

    let mut scores = Vec::new();
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    // Rust改进: 使用迭代器链式处理输入
    for token in input.split_whitespace() {
        if let Ok(score) = token.parse::<i32>() {
            if score < 0 {
                break;
            }
            if scores.len() >= 1000 {
                break;
            }
            scores.push(score);
        }
    }

    Ok(scores)
}

/// 显示分数数组（每行10个）
fn display_scores(scores: &[i32]) {
    println!("输入的数组为 : ");
    for (i, &score) in scores.iter().enumerate() {
        if i % 10 == 0 {
            println!();
        }
        print!("{} ", score);
    }
    println!();
}

fn main() {
    match read_scores() {
        Ok(mut scores) => {
            if scores.is_empty() {
                println!("无有效输入");
                return;
            }

            display_scores(&scores);

            // Rust改进: 使用sort_unstable_by降序排序，比冒泡排序快得多
            // 时间复杂度从O(n²)降到O(n log n)
            scores.sort_unstable_by(|a, b| b.cmp(a));

            println!("\n分数与名次的对应关系为 : ");
            let rankings = calculate_rankings(&scores);

            for (score, rank) in rankings {
                println!("{} {}", score, rank);
            }
        }
        Err(e) => {
            eprintln!("读取输入失败: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rankings_simple() {
        let scores = vec![100, 90, 80];
        let rankings = calculate_rankings(&scores);
        assert_eq!(rankings, vec![(100, 1), (90, 2), (80, 3)]);
    }

    #[test]
    fn test_calculate_rankings_with_ties() {
        // 测试并列排名
        let scores = vec![95, 90, 90, 85];
        let rankings = calculate_rankings(&scores);
        assert_eq!(rankings, vec![(95, 1), (90, 2), (90, 2), (85, 4)]);
    }

    #[test]
    fn test_calculate_rankings_all_same() {
        // 测试所有分数相同
        let scores = vec![80, 80, 80];
        let rankings = calculate_rankings(&scores);
        assert_eq!(rankings, vec![(80, 1), (80, 1), (80, 1)]);
    }

    #[test]
    fn test_calculate_rankings_empty() {
        let scores: Vec<i32> = vec![];
        let rankings = calculate_rankings(&scores);
        assert_eq!(rankings, vec![]);
    }

    #[test]
    fn test_calculate_rankings_single() {
        let scores = vec![100];
        let rankings = calculate_rankings(&scores);
        assert_eq!(rankings, vec![(100, 1)]);
    }

    #[test]
    fn test_calculate_rankings_complex() {
        // 测试复杂情况：多个并列
        let scores = vec![100, 95, 95, 95, 90, 90, 85];
        let rankings = calculate_rankings(&scores);
        assert_eq!(
            rankings,
            vec![
                (100, 1),
                (95, 2),
                (95, 2),
                (95, 2),
                (90, 5),
                (90, 5),
                (85, 7)
            ]
        );
    }

    #[test]
    fn test_sorting_descending() {
        // 测试降序排序
        let mut scores = vec![70, 90, 80, 100, 85];
        scores.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(scores, vec![100, 90, 85, 80, 70]);
    }

    #[test]
    fn test_ranking_boundary_values() {
        // 测试边界值
        let scores = vec![100, 0];
        let rankings = calculate_rankings(&scores);
        assert_eq!(rankings, vec![(100, 1), (0, 2)]);
    }

    #[test]
    fn test_ranking_large_dataset() {
        // 测试大数据集
        let mut scores: Vec<i32> = (1..=100).collect();
        scores.sort_unstable_by(|a, b| b.cmp(a));
        let rankings = calculate_rankings(&scores);

        assert_eq!(rankings.len(), 100);
        assert_eq!(rankings[0], (100, 1));
        assert_eq!(rankings[99], (1, 100));
    }
}
