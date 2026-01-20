// 7-b2: KFC Ordering System with Combo Discounts
// Original: 2052526 信15 白俊豪
//
// 问题描述: KFC点餐系统，支持单品点餐和优惠套餐自动计算
// 输入字母代表菜品（A-Z），系统自动识别最优惠的套餐组合并计算折扣
//
// Rust改进:
// 1. 使用结构体和常量数组替代C风格结构体，类型安全且零成本抽象
// 2. 使用HashMap<char, usize>计数替代固定26长度数组，更清晰表达意图
// 3. 使用迭代器和函数式编程替代手动循环，代码更简洁
// 4. 提取核心逻辑为纯函数，便于单元测试和复用
// 5. 使用Result类型处理输入验证，比C++的返回码更类型安全
// 6. 避免使用unsafe代码和原始指针，完全内存安全
// 7. 使用String和&str替代char数组，避免缓冲区溢出风险
// 8. 贪心算法计算最大折扣：优先使用折扣最大的套餐
// 9. 使用Vec和迭代器构建输出字符串，避免手动格式化
// 10. 添加全面的单元测试覆盖所有边界情况

use std::collections::HashMap;
use std::io::{self, Write};

/// 菜单项结构体
#[derive(Debug, Clone)]
struct MenuItem {
    choice: char,
    name: &'static str,
    price: f64,
}

/// 优惠套餐结构体
#[derive(Debug, Clone)]
struct SpecialCombo {
    combo: &'static str,
    name: &'static str,
    price: f64,
}

/// KFC 2020冬季菜单
const MENU: &[MenuItem] = &[
    MenuItem {
        choice: 'A',
        name: "香辣鸡腿堡",
        price: 18.0,
    },
    MenuItem {
        choice: 'B',
        name: "劲脆鸡腿堡",
        price: 18.0,
    },
    MenuItem {
        choice: 'C',
        name: "新奥尔良烤鸡腿堡",
        price: 18.5,
    },
    MenuItem {
        choice: 'D',
        name: "鸡肉火腿帕尼尼",
        price: 14.0,
    },
    MenuItem {
        choice: 'E',
        name: "老北京鸡肉卷",
        price: 16.5,
    },
    MenuItem {
        choice: 'F',
        name: "川辣嫩牛卷",
        price: 19.0,
    },
    MenuItem {
        choice: 'G',
        name: "吮指原味鸡(1块)",
        price: 11.5,
    },
    MenuItem {
        choice: 'H',
        name: "热辣薯片脆皮鸡",
        price: 12.5,
    },
    MenuItem {
        choice: 'I',
        name: "新奥尔良烤翅(2块)",
        price: 12.0,
    },
    MenuItem {
        choice: 'J',
        name: "劲爆鸡米花",
        price: 10.5,
    },
    MenuItem {
        choice: 'K',
        name: "香辣鸡翅(2块)",
        price: 11.0,
    },
    MenuItem {
        choice: 'L',
        name: "热辣香骨鸡(3块)",
        price: 11.0,
    },
    MenuItem {
        choice: 'M',
        name: "鲜蔬色拉",
        price: 12.5,
    },
    MenuItem {
        choice: 'N',
        name: "薯条(小)",
        price: 8.0,
    },
    MenuItem {
        choice: 'O',
        name: "薯条(中)",
        price: 11.0,
    },
    MenuItem {
        choice: 'P',
        name: "薯条(大)",
        price: 13.0,
    },
    MenuItem {
        choice: 'Q',
        name: "芙蓉蔬荟汤",
        price: 8.0,
    },
    MenuItem {
        choice: 'R',
        name: "原味花筒冰激凌",
        price: 6.0,
    },
    MenuItem {
        choice: 'S',
        name: "醇香土豆泥",
        price: 6.5,
    },
    MenuItem {
        choice: 'T',
        name: "香甜粟米棒",
        price: 8.0,
    },
    MenuItem {
        choice: 'U',
        name: "葡式蛋挞",
        price: 7.5,
    },
    MenuItem {
        choice: 'V',
        name: "百事可乐(小)",
        price: 7.0,
    },
    MenuItem {
        choice: 'W',
        name: "百事可乐(中)",
        price: 9.5,
    },
    MenuItem {
        choice: 'X',
        name: "百事可乐(大)",
        price: 11.5,
    },
    MenuItem {
        choice: 'Y',
        name: "九珍果汁饮料",
        price: 12.0,
    },
    MenuItem {
        choice: 'Z',
        name: "纯纯玉米饮",
        price: 11.0,
    },
];

/// 优惠套餐列表
const SPECIALS: &[SpecialCombo] = &[
    SpecialCombo {
        combo: "ANV",
        name: "香辣鸡腿堡工作日午餐",
        price: 22.0,
    },
    SpecialCombo {
        combo: "BMV",
        name: "劲脆鸡腿堡超值套餐",
        price: 24.0,
    },
    SpecialCombo {
        combo: "ABCGGIIKKOUWWW",
        name: "超值全家桶",
        price: 100.0,
    },
    SpecialCombo {
        combo: "KIIRRJUWW",
        name: "缤纷小吃桶",
        price: 65.0,
    },
    SpecialCombo {
        combo: "JJ",
        name: "劲爆鸡米花(2份小)",
        price: 9.5,
    },
];

/// 验证输入是否只包含字母
///
/// # Arguments
/// * `input` - 用户输入的字符串
///
/// # Returns
/// * `Ok(())` - 输入有效
/// * `Err(String)` - 输入无效，返回错误信息
fn validate_input(input: &str) -> Result<(), String> {
    if input.is_empty() {
        return Err("输入为空".to_string());
    }

    // Rust改进: 使用chars().all()验证，比C风格循环更简洁
    if !input.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("输入包含非字母字符".to_string());
    }

    Ok(())
}

/// 统计每个字母出现的次数
///
/// # Arguments
/// * `input` - 输入字符串（已转大写）
///
/// # Returns
/// * HashMap<char, usize> - 每个字母的出现次数
///
/// # Examples
/// ```
/// let count = count_items("AABBC");
/// assert_eq!(count.get(&'A'), Some(&2));
/// assert_eq!(count.get(&'B'), Some(&2));
/// assert_eq!(count.get(&'C'), Some(&1));
/// ```
fn count_items(input: &str) -> HashMap<char, usize> {
    // Rust改进: 使用HashMap替代固定26长度数组，更清晰表达意图
    let mut count = HashMap::new();
    for ch in input.chars() {
        *count.entry(ch).or_insert(0) += 1;
    }
    count
}

/// 尝试从订单中扣除一个套餐的物品
///
/// # Arguments
/// * `order_count` - 当前订单计数
/// * `combo` - 套餐组合字符串
///
/// # Returns
/// * `true` - 成功扣除（订单包含该套餐）
/// * `false` - 扣除失败（订单不包含该套餐）
fn try_subtract_combo(order_count: &mut HashMap<char, usize>, combo: &str) -> bool {
    // 先检查是否所有物品都足够
    let combo_count = count_items(combo);
    for (ch, &needed) in &combo_count {
        if order_count.get(ch).copied().unwrap_or(0) < needed {
            return false;
        }
    }

    // 扣除物品，如果数量为0则移除该项
    for (ch, needed) in combo_count {
        let entry = order_count.get_mut(&ch).unwrap();
        *entry -= needed;
        if *entry == 0 {
            order_count.remove(&ch);
        }
    }

    true
}

/// 计算套餐的原价（不使用优惠）
///
/// # Arguments
/// * `combo` - 套餐组合字符串
///
/// # Returns
/// * f64 - 套餐原价
fn calc_combo_original_price(combo: &str) -> f64 {
    // Rust改进: 使用迭代器sum()，比手动循环累加更简洁
    combo
        .chars()
        .filter_map(|ch| MENU.iter().find(|item| item.choice == ch))
        .map(|item| item.price)
        .sum()
}

/// 计算最大折扣金额（贪心算法）
///
/// # Arguments
/// * `input` - 用户输入的订单字符串（大写）
///
/// # Returns
/// * f64 - 总折扣金额
///
/// # Algorithm
/// 贪心策略：按折扣金额从大到小排序，优先使用折扣最大的套餐
/// 对每个套餐，尽可能多次使用直到订单中物品不足
fn calc_discount(input: &str) -> f64 {
    let mut order_count = count_items(input);
    let mut total_discount = 0.0;

    // 计算每个套餐的折扣金额
    let mut discounts: Vec<(usize, f64)> = SPECIALS
        .iter()
        .enumerate()
        .map(|(idx, special)| {
            let original = calc_combo_original_price(special.combo);
            let discount = original - special.price;
            (idx, discount)
        })
        .collect();

    // Rust改进: 贪心算法 - 按折扣金额降序排序
    discounts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // 尽可能多次使用每个套餐
    for (idx, discount_amount) in discounts {
        let special = &SPECIALS[idx];
        while try_subtract_combo(&mut order_count, special.combo) {
            total_discount += discount_amount;
        }
    }

    total_discount
}

/// 计算订单总价（不含折扣）
///
/// # Arguments
/// * `order_count` - 订单计数
///
/// # Returns
/// * f64 - 总价
fn calc_total_price(order_count: &HashMap<char, usize>) -> f64 {
    // Rust改进: 使用迭代器链式调用，比嵌套循环更清晰
    order_count
        .iter()
        .filter_map(|(ch, &count)| {
            MENU.iter()
                .find(|item| item.choice == *ch)
                .map(|item| item.price * count as f64)
        })
        .sum()
}

/// 格式化订单输出字符串
///
/// # Arguments
/// * `order_count` - 订单计数
///
/// # Returns
/// * String - 格式化的订单字符串
///
/// # Examples
/// ```
/// let mut count = HashMap::new();
/// count.insert('A', 2);
/// count.insert('B', 1);
/// let result = format_order(&count);
/// assert_eq!(result, "香辣鸡腿堡*2+劲脆鸡腿堡");
/// ```
fn format_order(order_count: &HashMap<char, usize>) -> String {
    // Rust改进: 使用Vec收集结果，然后join，避免手动拼接字符串
    let mut items: Vec<String> = Vec::new();

    for item in MENU {
        if let Some(&count) = order_count.get(&item.choice) {
            if count > 0 {
                if count == 1 {
                    items.push(item.name.to_string());
                } else {
                    items.push(format!("{}*{}", item.name, count));
                }
            }
        }
    }

    items.join("+")
}

/// 打印菜单
fn print_menu() {
    println!("=============================================================");
    println!("                      KFC 2020冬季菜单                       ");
    println!("=============================================================");

    // 两列显示菜单
    for i in (0..MENU.len()).step_by(2) {
        print!(
            "{} {:20} {:>7.1}",
            MENU[i].choice, MENU[i].name, MENU[i].price
        );
        if i + 1 < MENU.len() {
            println!(
                " | {} {:20} {:>7.1}",
                MENU[i + 1].choice,
                MENU[i + 1].name,
                MENU[i + 1].price
            );
        } else {
            println!();
        }
    }

    println!();
    print_discount_info();
    print_usage_info();
}

/// 打印优惠信息
fn print_discount_info() {
    println!("【优惠信息】：");
    for special in SPECIALS {
        print!("{}=", special.name);

        // 统计套餐中每个物品的数量
        let combo_count = count_items(special.combo);
        let mut items: Vec<String> = Vec::new();

        for item in MENU {
            if let Some(&count) = combo_count.get(&item.choice) {
                if count > 0 {
                    if count == 1 {
                        items.push(item.name.to_string());
                    } else {
                        items.push(format!("{}*{}", item.name, count));
                    }
                }
            }
        }

        println!("{}={}", items.join("+"), special.price);
        println!();
    }
}

/// 打印使用说明
fn print_usage_info() {
    println!("【输入规则说明】：");
    println!("ANV = 香辣鸡腿堡 + 薯条(小) + 百事可乐(小) / akaak = 香辣鸡腿堡 * 3 + 香辣鸡翅 * 2");
    println!("字母不分大小写，不限顺序，单独输入0则退出程序");
    println!();
}

/// 处理订单并打印结果
///
/// # Arguments
/// * `input` - 用户输入的订单字符串
///
/// # Returns
/// * Result<(), String> - 成功或错误信息
fn process_order(input: &str) -> Result<(), String> {
    // 验证输入
    validate_input(input)?;

    // 转换为大写
    let input_upper = input.to_uppercase();

    // 统计订单
    let order_count = count_items(&input_upper);

    // 计算折扣
    let discount = calc_discount(&input_upper);

    // 计算总价
    let total = calc_total_price(&order_count);
    let final_price = total - discount;

    // 打印结果
    println!("您的点餐 = {}", format_order(&order_count));
    println!("总计:{:.1}元", final_price);
    println!();

    Ok(())
}

fn main() {
    loop {
        // 清屏（跨平台）
        print!("\x1B[2J\x1B[1;1H");

        print_menu();
        print!("请点单:");
        io::stdout().flush().unwrap();

        // 读取输入
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("读取输入失败: {}", e);
            continue;
        }

        let input = input.trim();

        // 检查退出
        if input == "0" {
            break;
        }

        // 检查空输入
        if input.is_empty() {
            continue;
        }

        // 处理订单
        match process_order(input) {
            Ok(()) => {
                println!("点餐完成，按回车键继续");
                let mut dummy = String::new();
                let _ = io::stdin().read_line(&mut dummy);
            }
            Err(e) => {
                println!("错误: {}，按回车键继续", e);
                let mut dummy = String::new();
                let _ = io::stdin().read_line(&mut dummy);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        // 测试有效输入
        assert!(validate_input("ABC").is_ok());
        assert!(validate_input("abc").is_ok());
        assert!(validate_input("AaBbCc").is_ok());
        assert!(validate_input("ABCDEFGHIJKLMNOPQRSTUVWXYZ").is_ok());

        // 测试无效输入
        assert!(validate_input("").is_err());
        assert!(validate_input("ABC123").is_err());
        assert!(validate_input("AB C").is_err());
        assert!(validate_input("AB-C").is_err());
        assert!(validate_input("AB+C").is_err());
    }

    #[test]
    fn test_count_items() {
        let count = count_items("AABBC");
        assert_eq!(count.get(&'A'), Some(&2));
        assert_eq!(count.get(&'B'), Some(&2));
        assert_eq!(count.get(&'C'), Some(&1));
        assert_eq!(count.get(&'D'), None);

        let count = count_items("AAA");
        assert_eq!(count.get(&'A'), Some(&3));

        let count = count_items("ABCDEFG");
        assert_eq!(count.len(), 7);
    }

    #[test]
    fn test_calc_combo_original_price() {
        // ANV套餐: A(18) + N(8) + V(7) = 33
        let price = calc_combo_original_price("ANV");
        assert!((price - 33.0).abs() < 0.01);

        // BMV套餐: B(18) + M(12.5) + V(7) = 37.5
        let price = calc_combo_original_price("BMV");
        assert!((price - 37.5).abs() < 0.01);

        // JJ套餐: J(10.5) + J(10.5) = 21
        let price = calc_combo_original_price("JJ");
        assert!((price - 21.0).abs() < 0.01);
    }

    #[test]
    fn test_try_subtract_combo() {
        let mut count = count_items("AANVV");

        // 可以扣除ANV套餐
        assert!(try_subtract_combo(&mut count, "ANV"));
        assert_eq!(count.get(&'A'), Some(&1));
        assert_eq!(count.get(&'N'), None);
        assert_eq!(count.get(&'V'), Some(&1));

        // 不能再扣除ANV套餐（N不足）
        assert!(!try_subtract_combo(&mut count, "ANV"));
    }

    #[test]
    fn test_calc_discount_simple() {
        // ANV套餐: 原价33，优惠价22，折扣11
        let discount = calc_discount("ANV");
        assert!((discount - 11.0).abs() < 0.01);

        // BMV套餐: 原价37.5，优惠价24，折扣13.5
        let discount = calc_discount("BMV");
        assert!((discount - 13.5).abs() < 0.01);

        // JJ套餐: 原价21，优惠价9.5，折扣11.5
        let discount = calc_discount("JJ");
        assert!((discount - 11.5).abs() < 0.01);
    }

    #[test]
    fn test_calc_discount_multiple() {
        // 两个ANV套餐
        let discount = calc_discount("AANVNV");
        assert!((discount - 22.0).abs() < 0.01);

        // 两个JJ套餐
        let discount = calc_discount("JJJJ");
        assert!((discount - 23.0).abs() < 0.01);
    }

    #[test]
    fn test_calc_discount_no_combo() {
        // 没有套餐，折扣为0
        let discount = calc_discount("A");
        assert!((discount - 0.0).abs() < 0.01);

        let discount = calc_discount("AB");
        assert!((discount - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_calc_total_price() {
        let mut count = HashMap::new();
        count.insert('A', 1);
        count.insert('B', 1);

        // A(18) + B(18) = 36
        let total = calc_total_price(&count);
        assert!((total - 36.0).abs() < 0.01);

        let mut count = HashMap::new();
        count.insert('A', 2);
        count.insert('N', 1);
        count.insert('V', 1);

        // A*2(36) + N(8) + V(7) = 51
        let total = calc_total_price(&count);
        assert!((total - 51.0).abs() < 0.01);
    }

    #[test]
    fn test_format_order() {
        let mut count = HashMap::new();
        count.insert('A', 2);
        count.insert('B', 1);

        let result = format_order(&count);
        assert!(result.contains("香辣鸡腿堡*2"));
        assert!(result.contains("劲脆鸡腿堡"));
        assert!(result.contains("+"));

        let mut count = HashMap::new();
        count.insert('A', 1);

        let result = format_order(&count);
        assert_eq!(result, "香辣鸡腿堡");
    }

    #[test]
    fn test_process_order_simple() {
        // 测试单个物品
        assert!(process_order("A").is_ok());
        assert!(process_order("a").is_ok());
    }

    #[test]
    fn test_process_order_combo() {
        // 测试套餐
        assert!(process_order("ANV").is_ok());
        assert!(process_order("anv").is_ok());
        assert!(process_order("BMV").is_ok());
    }

    #[test]
    fn test_process_order_invalid() {
        // 测试无效输入
        assert!(process_order("").is_err());
        assert!(process_order("123").is_err());
        assert!(process_order("A B").is_err());
    }

    #[test]
    fn test_case_insensitive() {
        // 测试大小写不敏感（通过process_order处理）
        let discount1 = calc_discount("ANV");
        let discount2 = calc_discount(&"anv".to_uppercase());
        assert!((discount1 - discount2).abs() < 0.01);

        let count1 = count_items("ABC");
        let count2 = count_items("abc".to_uppercase().as_str());
        assert_eq!(count1, count2);
    }

    #[test]
    fn test_order_independence() {
        // 测试顺序无关性
        let discount1 = calc_discount("ANV");
        let discount2 = calc_discount("VNA");
        let discount3 = calc_discount("NVA");
        assert!((discount1 - discount2).abs() < 0.01);
        assert!((discount1 - discount3).abs() < 0.01);
    }

    #[test]
    fn test_greedy_algorithm() {
        // 测试贪心算法选择最优套餐
        // 如果有多个套餐可选，应该选择折扣最大的

        // 超值全家桶的折扣应该很大
        let input = "ABCGGIIKKOUWWW";
        let discount = calc_discount(input);
        assert!(discount > 0.0);
    }

    #[test]
    fn test_complex_order() {
        // 测试复杂订单
        let input = "AANVBMVJJ";
        assert!(process_order(input).is_ok());

        let discount = calc_discount(input);
        // 应该包含ANV(11) + BMV(13.5) + JJ(11.5) = 36
        assert!(discount > 30.0);
    }

    #[test]
    fn test_all_menu_items() {
        // 测试所有菜单项都能正确处理
        for item in MENU {
            let input = item.choice.to_string();
            assert!(process_order(&input).is_ok());
        }
    }

    #[test]
    fn test_all_special_combos() {
        // 测试所有优惠套餐
        for special in SPECIALS {
            assert!(process_order(special.combo).is_ok());
            let discount = calc_discount(special.combo);
            assert!(discount > 0.0);
        }
    }
}
