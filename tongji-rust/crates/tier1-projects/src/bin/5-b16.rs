// 5-b16: Student grade management system
// Original: 2052526 信15 白俊豪
//
// Rust改进:
// 1. 使用结构体Student代替字符串解析，类型安全
// 2. 使用Vec<Student>代替固定大小数组，更灵活
// 3. 使用sort_by_key代替手写冒泡排序，O(n log n)复杂度
// 4. 使用filter和collect进行函数式过滤，更简洁
// 5. 使用Result处理解析错误，避免panic
// 6. 分离数据结构和业务逻辑，便于测试

use std::io::{self, Write};

/// 学生信息结构体
#[derive(Debug, Clone, PartialEq, Eq)]
struct Student {
    id: String,
    name: String,
    score: u32,
}

impl Student {
    /// 从输入字符串解析学生信息
    ///
    /// # Arguments
    /// * `input` - 格式为 "学号 姓名 成绩" 的字符串
    ///
    /// # Returns
    /// * `Ok(Student)` - 解析成功
    /// * `Err(String)` - 解析失败，返回错误信息
    fn parse(input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 3 {
            return Err("输入格式错误，应为：学号 姓名 成绩".to_string());
        }

        let score = parts[2]
            .parse::<u32>()
            .map_err(|_| "成绩必须是有效的数字".to_string())?;

        Ok(Student {
            id: parts[0].to_string(),
            name: parts[1].to_string(),
            score,
        })
    }

    /// 判断是否及格
    fn is_passing(&self) -> bool {
        self.score >= 60
    }
}

/// 输入学生信息
///
/// # Arguments
/// * `count` - 学生数量
///
/// # Returns
/// * `Vec<Student>` - 学生列表
fn input_students(count: usize) -> Vec<Student> {
    let mut students = Vec::with_capacity(count);

    for i in 0..count {
        loop {
            print!("请输入第{}个人的学号、姓名、成绩\n", i + 1);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match Student::parse(&input) {
                Ok(student) => {
                    students.push(student);
                    break;
                }
                Err(e) => {
                    println!("错误: {}", e);
                    continue;
                }
            }
        }
    }

    students
}

/// 按成绩降序排序学生列表
///
/// # Arguments
/// * `students` - 学生列表的可变引用
///
/// Rust改进: 使用标准库的sort_by_key，时间复杂度O(n log n)
/// 比原C++的冒泡排序O(n²)更高效
fn sort_by_score(students: &mut [Student]) {
    students.sort_by_key(|s| std::cmp::Reverse(s.score));
}

/// 获取不及格学生列表
///
/// # Arguments
/// * `students` - 学生列表
///
/// # Returns
/// * `Vec<&Student>` - 不及格学生的引用列表
///
/// Rust改进: 使用函数式编程filter，避免手动循环
fn get_failing_students(students: &[Student]) -> Vec<&Student> {
    students.iter().filter(|s| !s.is_passing()).collect()
}

/// 打印不及格学生名单
fn print_failing_students(students: &[&Student]) {
    println!("\n不及格名单:");
    for student in students {
        println!("{} {} {}", student.id, student.name, student.score);
    }
}

fn main() {
    const STUDENT_COUNT: usize = 10;

    // 输入学生信息
    let mut students = input_students(STUDENT_COUNT);

    // 按成绩降序排序
    sort_by_score(&mut students);

    // 获取并打印不及格学生
    let failing = get_failing_students(&students);
    print_failing_students(&failing);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_parse_valid() {
        let student = Student::parse("2052526 张三 85").unwrap();
        assert_eq!(student.id, "2052526");
        assert_eq!(student.name, "张三");
        assert_eq!(student.score, 85);
    }

    #[test]
    fn test_student_parse_with_extra_spaces() {
        let student = Student::parse("  2052526   张三   85  ").unwrap();
        assert_eq!(student.id, "2052526");
        assert_eq!(student.name, "张三");
        assert_eq!(student.score, 85);
    }

    #[test]
    fn test_student_parse_invalid_format() {
        assert!(Student::parse("2052526 张三").is_err());
        assert!(Student::parse("2052526").is_err());
        assert!(Student::parse("").is_err());
    }

    #[test]
    fn test_student_parse_invalid_score() {
        assert!(Student::parse("2052526 张三 abc").is_err());
        assert!(Student::parse("2052526 张三 85.5").is_err());
    }

    #[test]
    fn test_is_passing() {
        let passing = Student {
            id: "001".to_string(),
            name: "张三".to_string(),
            score: 60,
        };
        assert!(passing.is_passing());

        let failing = Student {
            id: "002".to_string(),
            name: "李四".to_string(),
            score: 59,
        };
        assert!(!failing.is_passing());
    }

    #[test]
    fn test_sort_by_score() {
        let mut students = vec![
            Student {
                id: "001".to_string(),
                name: "A".to_string(),
                score: 70,
            },
            Student {
                id: "002".to_string(),
                name: "B".to_string(),
                score: 90,
            },
            Student {
                id: "003".to_string(),
                name: "C".to_string(),
                score: 80,
            },
        ];

        sort_by_score(&mut students);

        assert_eq!(students[0].score, 90);
        assert_eq!(students[1].score, 80);
        assert_eq!(students[2].score, 70);
    }

    #[test]
    fn test_get_failing_students() {
        let students = vec![
            Student {
                id: "001".to_string(),
                name: "A".to_string(),
                score: 70,
            },
            Student {
                id: "002".to_string(),
                name: "B".to_string(),
                score: 50,
            },
            Student {
                id: "003".to_string(),
                name: "C".to_string(),
                score: 80,
            },
            Student {
                id: "004".to_string(),
                name: "D".to_string(),
                score: 45,
            },
        ];

        let failing = get_failing_students(&students);

        assert_eq!(failing.len(), 2);
        assert_eq!(failing[0].name, "B");
        assert_eq!(failing[1].name, "D");
    }

    #[test]
    fn test_get_failing_students_all_passing() {
        let students = vec![
            Student {
                id: "001".to_string(),
                name: "A".to_string(),
                score: 70,
            },
            Student {
                id: "002".to_string(),
                name: "B".to_string(),
                score: 80,
            },
        ];

        let failing = get_failing_students(&students);
        assert_eq!(failing.len(), 0);
    }

    #[test]
    fn test_get_failing_students_all_failing() {
        let students = vec![
            Student {
                id: "001".to_string(),
                name: "A".to_string(),
                score: 50,
            },
            Student {
                id: "002".to_string(),
                name: "B".to_string(),
                score: 40,
            },
        ];

        let failing = get_failing_students(&students);
        assert_eq!(failing.len(), 2);
    }

    #[test]
    fn test_boundary_score_60() {
        let student = Student {
            id: "001".to_string(),
            name: "边界".to_string(),
            score: 60,
        };
        assert!(student.is_passing());

        let students = vec![student.clone()];
        let failing = get_failing_students(&students);
        assert_eq!(failing.len(), 0);
    }

    #[test]
    fn test_boundary_score_59() {
        let student = Student {
            id: "001".to_string(),
            name: "边界".to_string(),
            score: 59,
        };
        assert!(!student.is_passing());

        let students = vec![student.clone()];
        let failing = get_failing_students(&students);
        assert_eq!(failing.len(), 1);
    }
}
