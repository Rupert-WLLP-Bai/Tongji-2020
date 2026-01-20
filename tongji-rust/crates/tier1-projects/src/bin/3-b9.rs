// 3-b9: Find perfect numbers (numbers equal to sum of their proper divisors)
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 提取核心逻辑为纯函数，便于测试和复用
// 2. 使用Iterator链式调用，避免手动循环累加
// 3. 使用Vec收集因子，避免重复计算
// 4. 使用filter + collect简化因子查找
// 5. 返回结构化数据(PerfectNumber)而非直接打印，分离逻辑与展示

use std::fmt;

/// 完全数结构体，包含数值和其因子
#[derive(Debug, PartialEq, Eq)]
pub struct PerfectNumber {
    pub number: u32,
    pub factors: Vec<u32>,
}

impl fmt::Display for PerfectNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},its factors are ", self.number)?;
        for (i, factor) in self.factors.iter().enumerate() {
            if i < self.factors.len() - 1 {
                write!(f, "{},", factor)?;
            } else {
                write!(f, "{}", factor)?;
            }
        }
        Ok(())
    }
}

/// 查找一个数的所有真因子(proper divisors)
///
/// # Arguments
/// * `n` - 要查找因子的数
///
/// # Returns
/// * `Vec<u32>` - 所有小于n的因子
///
/// # Examples
/// ```
/// # use tier1_projects::*;
/// let factors = find_proper_divisors(6);
/// assert_eq!(factors, vec![1, 2, 3]);
/// ```
pub fn find_proper_divisors(n: u32) -> Vec<u32> {
    // Rust改进: 使用Iterator的filter和collect，代码更简洁
    // 只需遍历到sqrt(n)，然后同时添加i和n/i
    if n <= 1 {
        return Vec::new();
    }

    let mut factors = Vec::new();
    factors.push(1); // 1总是因子

    // 优化: 只遍历到sqrt(n)
    let sqrt_n = (n as f64).sqrt() as u32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            factors.push(i);
            let complement = n / i;
            // 避免重复添加(例如n=16时，4*4=16)
            if complement != i && complement != n {
                factors.push(complement);
            }
        }
    }

    // 排序因子以便输出
    factors.sort_unstable();
    factors
}

/// 检查一个数是否为完全数
///
/// # Arguments
/// * `n` - 要检查的数
///
/// # Returns
/// * `Option<PerfectNumber>` - 如果是完全数返回Some，否则返回None
///
/// # Examples
/// ```
/// # use tier1_projects::*;
/// assert!(is_perfect_number(6).is_some());
/// assert!(is_perfect_number(28).is_some());
/// assert!(is_perfect_number(10).is_none());
/// ```
pub fn is_perfect_number(n: u32) -> Option<PerfectNumber> {
    // 完全数必须大于1
    if n <= 1 {
        return None;
    }

    let factors = find_proper_divisors(n);
    // Rust改进: 使用Iterator的sum()方法，避免手动循环
    let sum: u32 = factors.iter().sum();

    if sum == n {
        Some(PerfectNumber {
            number: n,
            factors,
        })
    } else {
        None
    }
}

/// 查找指定范围内的所有完全数
///
/// # Arguments
/// * `start` - 起始值(包含)
/// * `end` - 结束值(包含)
///
/// # Returns
/// * `Vec<PerfectNumber>` - 范围内所有完全数
///
/// # Examples
/// ```
/// # use tier1_projects::*;
/// let perfects = find_perfect_numbers(1, 30);
/// assert_eq!(perfects.len(), 2); // 6 and 28
/// assert_eq!(perfects[0].number, 6);
/// assert_eq!(perfects[1].number, 28);
/// ```
pub fn find_perfect_numbers(start: u32, end: u32) -> Vec<PerfectNumber> {
    // Rust改进: 使用Iterator链式调用，代码更函数式
    // filter_map会自动过滤None值并解包Some值
    (start..=end)
        .filter_map(is_perfect_number)
        .collect()
}

fn main() {
    // 注意: 原C++代码注释说"这一类数不可能为奇数"是正确的
    // 已知的完全数都是偶数，但从2开始遍历更安全
    let perfect_numbers = find_perfect_numbers(2, 1000);

    for pn in perfect_numbers {
        println!("{}", pn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_proper_divisors_small() {
        // 测试小数字的因子
        assert_eq!(find_proper_divisors(1), vec![]);
        assert_eq!(find_proper_divisors(2), vec![1]);
        assert_eq!(find_proper_divisors(6), vec![1, 2, 3]);
    }

    #[test]
    fn test_find_proper_divisors_perfect_square() {
        // 测试完全平方数
        let factors = find_proper_divisors(16);
        assert_eq!(factors, vec![1, 2, 4, 8]);
    }

    #[test]
    fn test_find_proper_divisors_prime() {
        // 测试质数(只有1作为真因子)
        assert_eq!(find_proper_divisors(7), vec![1]);
        assert_eq!(find_proper_divisors(13), vec![1]);
    }

    #[test]
    fn test_is_perfect_number_6() {
        // 测试第一个完全数: 6 = 1 + 2 + 3
        let result = is_perfect_number(6);
        assert!(result.is_some());
        let pn = result.unwrap();
        assert_eq!(pn.number, 6);
        assert_eq!(pn.factors, vec![1, 2, 3]);
    }

    #[test]
    fn test_is_perfect_number_28() {
        // 测试第二个完全数: 28 = 1 + 2 + 4 + 7 + 14
        let result = is_perfect_number(28);
        assert!(result.is_some());
        let pn = result.unwrap();
        assert_eq!(pn.number, 28);
        assert_eq!(pn.factors, vec![1, 2, 4, 7, 14]);
    }

    #[test]
    fn test_is_perfect_number_496() {
        // 测试第三个完全数: 496
        let result = is_perfect_number(496);
        assert!(result.is_some());
        let pn = result.unwrap();
        assert_eq!(pn.number, 496);
        // 496的因子: 1, 2, 4, 8, 16, 31, 62, 124, 248
        assert_eq!(pn.factors.iter().sum::<u32>(), 496);
    }

    #[test]
    fn test_is_not_perfect_number() {
        // 测试非完全数
        assert!(is_perfect_number(10).is_none());
        assert!(is_perfect_number(12).is_none());
        assert!(is_perfect_number(100).is_none());
    }

    #[test]
    fn test_find_perfect_numbers_small_range() {
        // 测试小范围
        let perfects = find_perfect_numbers(1, 10);
        assert_eq!(perfects.len(), 1);
        assert_eq!(perfects[0].number, 6);
    }

    #[test]
    fn test_find_perfect_numbers_up_to_30() {
        // 测试到30的范围
        let perfects = find_perfect_numbers(1, 30);
        assert_eq!(perfects.len(), 2);
        assert_eq!(perfects[0].number, 6);
        assert_eq!(perfects[1].number, 28);
    }

    #[test]
    fn test_find_perfect_numbers_up_to_1000() {
        // 测试到1000的范围(应该找到3个: 6, 28, 496)
        let perfects = find_perfect_numbers(2, 1000);
        assert_eq!(perfects.len(), 3);
        assert_eq!(perfects[0].number, 6);
        assert_eq!(perfects[1].number, 28);
        assert_eq!(perfects[2].number, 496);
    }

    #[test]
    fn test_perfect_number_display() {
        // 测试Display trait实现
        let pn = PerfectNumber {
            number: 6,
            factors: vec![1, 2, 3],
        };
        assert_eq!(format!("{}", pn), "6,its factors are 1,2,3");
    }

    #[test]
    fn test_edge_cases() {
        // 测试边界情况
        assert_eq!(find_proper_divisors(0), vec![]);
        assert!(is_perfect_number(0).is_none());
        assert!(is_perfect_number(1).is_none());
    }

    #[test]
    fn test_factors_sum_correctness() {
        // 验证因子和的正确性
        for n in 2..=100 {
            let factors = find_proper_divisors(n);
            let sum: u32 = factors.iter().sum();
            if sum == n {
                assert!(is_perfect_number(n).is_some());
            } else {
                assert!(is_perfect_number(n).is_none());
            }
        }
    }
}
