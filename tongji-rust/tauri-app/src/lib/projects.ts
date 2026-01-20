import { Project } from '../types';

// All projects from the Rust migration
export const projects: Project[] = [
  // Tier 1 Projects (3-b*, 4-b*)
  { id: '3-b2', name: '3-b2', tier: 'tier1', description: '几何计算 (Geometric Calculations) - 根据输入的半径和高度，计算圆周长、圆面积、圆球表面积、圆球体积和圆柱体积 | Input: 半径 r, 高度 h | Output: 保留2位小数 | Rust改进: 修复原C++整数除法bug (4/3), 使用Result类型处理错误, 提取纯函数便于测试', category: 'Mathematics' },
  { id: '3-b3', name: '3-b3', tier: 'tier1', description: '整数位数提取 (Integer Digit Extraction) - 输入1到30000之间的整数，使用取模和除法运算提取各个数位 | Input: [1, 30000] | Output: 万位、千位、百位、十位、个位 | Rust改进: 使用数组存储各位数字避免多个临时变量, 提取核心逻辑为纯函数便于单元测试, 使用迭代器和enumerate()优雅处理位数提取', category: 'Fundamentals' },
  { id: '3-b4', name: '3-b4', tier: 'tier1', description: '大数位数提取 (Large Number Digit Extraction) - 提取数字的每一位（十亿位至分位），使用中文货币单位输出 | Input: [0, 100亿), 最多2位小数 | Output: 十亿/亿/千万/百万/十万/万/千/百/十/圆/角/分 | Rust改进: 使用整数运算避免浮点精度问题, 结构体封装提高类型安全, 纯函数设计便于单元测试', category: 'Mathematics' },
  { id: '3-b5', name: '3-b5', tier: 'tier1', description: '三角形面积计算 (Triangle Area Calculation) - 输入两边及夹角（度），计算面积 | Input: 边a, 边b, 夹角度数 | Formula: S = (1/2)×a×b×sin(θ) | Output: 保留3位小数 | Rust改进: 使用f64高精度类型和std::f64::consts::PI常量, 添加输入验证(边长为正/角度0-180度), 使用Result类型优雅处理错误', category: 'Mathematics' },
  { id: '3-b6', name: '3-b6', tier: 'tier1', description: '日期计算器 (Date Calculator) - 计算给定日期是该年的第几天，支持闰年判断 | Rust改进: 使用const数组存储月份天数, 提取独立函数便于测试, 使用迭代器sum()计算累计天数', category: 'Fundamentals' },
  { id: '3-b7', name: '3-b7', tier: 'tier1', description: '人民币大写转换器 (RMB Converter) - 将数字金额转换为中文货币大写格式 | Rust改进: 使用结构体封装数字各位, 使用数组替代switch语句, String::with_capacity预分配内存', category: 'Fundamentals' },
  { id: '3-b8', name: '3-b8', tier: 'tier1', description: '找零计算器 (Change Calculator) - 使用贪心算法计算最少纸币和硬币的找零方案 | Rust改进: 使用整数运算(分为单位)避免浮点精度问题, 使用结构体封装找零结果, 使用迭代器简化输出', category: 'Fundamentals' },
  { id: '3-b9', name: '3-b9', tier: 'tier1', description: '完全数查找器 (Perfect Number Finder) - 查找1000以内的完全数及其因子 | Rust改进: Iterator链式调用, 返回结构化数据, 优化因子查找算法', category: 'Fundamentals' },
  { id: '3-b10', name: '3-b10', tier: 'tier1', description: '输入验证器 (Input Validator) - 读取并验证0-100之间的整数输入 | Rust改进: loop返回值避免未初始化变量, Result链式调用, 提取验证逻辑为独立函数', category: 'Fundamentals' },
  { id: '3-b11', name: '3-b11', tier: 'tier1', description: '九九乘法表生成器 (Multiplication Table Generator) - 生成格式化的9×9乘法表，使用迭代器优化输出 | Rust改进: 迭代器链式调用简化代码、提取格式化逻辑为独立函数、format!宏替代setw', category: 'Fundamentals' },
  { id: '3-b12', name: '3-b12', tier: 'tier1', description: '月历打印器 (Calendar Printer) - 显示指定年月的日历，支持闰年判断和星期对齐 | Rust改进: 消除24个重复switch case、const数组存储月份天数、独立函数提高可测试性', category: 'Fundamentals' },
  { id: '3-b13', name: '3-b13', tier: 'tier1', description: '三数求和问题 (Three-Number Sum Puzzle) - 查找和为1953且使用1-9不重复数字的三位数组合，使用位掩码优化 | Rust改进: 位掩码优化从O(n²)到O(1)、预计算掩码避免重复、动态计算搜索边界', category: 'Fundamentals' },
  { id: '4-b11', name: '4-b11', tier: 'tier1', description: 'Fibonacci数列性能计时 (Fibonacci Sequence with Performance Timing) - 计算Fibonacci数列第n项，对比递归、迭代和记忆化三种算法的性能差异 | Input: 项数[1-46] | Output: Fibonacci值和算法耗时 | Rust改进: std::time::Instant跨平台高精度计时、记忆化优化避免重复计算、checked_add检测溢出、提供递归/迭代/记忆化三种实现展示性能差异', category: 'Algorithms' },
  { id: '4-b12', name: '4-b12', tier: 'tier1', description: '幂次检测器 (Power Base Checker) - 使用递归检查一个数是否是指定基数的幂 | Input: 整数num, 基数base(≥2) | Output: 是否为base的幂 | Rust改进: bool类型替代int返回值、尾递归优化、边界条件检查避免无效输入、Result处理输入错误、match表达式清晰逻辑分支', category: 'Algorithms' },
  { id: '4-b13', name: '4-b13', tier: 'tier1', description: '递归三角塔打印器 (Recursive Triangle Tower Printer) - 使用递归打印正/倒三角形字母塔 | Input: 结束字符(A-Z) | Output: 正三角塔(中间为A)和倒三角塔(两边为A) | Rust改进: enum表示正/倒三角形类型安全、String缓冲区避免多次IO、迭代器和collect()函数式风格、独立函数增强可测试性', category: 'Algorithms' },
  { id: '4-b14', name: '4-b14', tier: 'tier1', description: '一元二次方程求解器 (Quadratic Equation Solver) - 求解ax²+bx+c=0的根，支持实根和复根 | Input: 系数a,b,c | Output: 两个相等实根/两个不等实根/两个共轭复根 | Rust改进: enum表示根的类型、Complex结构体表示复数、Display trait统一格式化输出、const EPSILON浮点数比较、match表达式清晰控制流', category: 'Mathematics' },
  { id: '4-b15', name: '4-b15', tier: 'tier1', description: '一元二次方程求解器 (Quadratic Equation Solver) - 求解ax²+bx+c=0，支持实根和复根 | Input: 系数a,b,c | Output: 两个不等实根/两个相等实根/两个虚根/不是二次方程 | Rust改进: enum表示解类型避免多个输出函数、match表达式替代if-else链、Display trait优雅格式化、const EPSILON替代魔数、避免重复计算判别式', category: 'Mathematics' },
  { id: '4-b16', name: '4-b16', tier: 'tier1', description: '一元二次方程求解器-模块化版本 (Quadratic Equation Solver - Modular) - 模块化设计的方程求解器，每个解类型独立格式化 | Input: 系数a,b,c | Output: 格式化的方程解 | Rust改进: enum+match替代多个独立函数、Result类型处理输入错误、闭包简化复数格式化、提取可测试纯函数、避免重复判别式计算', category: 'Mathematics' },
  { id: '4-b17', name: '4-b17', tier: 'tier1', description: '一元二次方程求解器-复数版本 (Quadratic Equation Solver - Complex Numbers) - 使用Complex结构体表示复数的方程求解器 | Input: 系数a,b,c | Output: 实根或复数根 | Rust改进: Complex结构体+Display trait统一复数格式化、enum避免全局变量、Result处理输入错误、match表达式清晰控制流、const EPSILON提高可读性', category: 'Mathematics' },

  // Tier 2 Projects (5-b*, 6-b*)
  { id: '5-b1', name: '5-b1', tier: 'tier2', description: '有序数组插入 (Insert into Sorted Array) - 将新数字插入到已排序数组中并保持有序 | Input: 升序正整数数组(最多20个), 待插入正整数 | Output: 插入后的有序数组 | Rust改进: 使用Vec<i32>替代固定数组, binary_search()二分查找插入位置O(log n), insert()方法直接插入避免冒泡排序O(n²), 提取纯函数便于单元测试', category: 'Data Structures' },
  { id: '5-b2', name: '5-b2', tier: 'tier2', description: '灯泡开关问题 (Light Bulb Toggle Problem) - 100个灯泡初始全灭，第i个人切换所有i的倍数编号的灯泡，求最后哪些灯泡亮着 | Input: 无 | Output: 最终亮着的灯泡编号 | Rust改进: 使用bool类型表示状态更清晰, 数学优化发现只有完全平方数的灯泡会亮(因数个数为奇数), 时间复杂度从O(n²)降到O(√n), 提供模拟和优化两种算法实现', category: 'Algorithms' },
  { id: '5-b7', name: '5-b7', tier: 'tier2', description: '汉诺塔可视化 (Hanoi Tower Visualization) - 使用控制台图形演示汉诺塔求解过程，支持自定义起始柱、目标柱和移动速度 | Input: 层数(1-10), 起始柱(A-C), 目标柱(A-C), 速度(0-5), 是否显示内部数组 | Output: 垂直显示柱子状态和移动步骤 | Rust改进: 枚举类型Tower代替字符提供类型安全, Vec<Vec<u8>>表示三个柱子的栈避免全局可变状态, 状态封装在HanoiState结构体中所有权清晰, console-tools库实现跨平台控制台操作, match表达式替代大量if-else', category: 'Visualization' },
  { id: '5-b8', name: '5-b8', tier: 'tier2', description: '凸多边形面积计算器 (Convex Polygon Area Calculator) - 使用叉积法判断凸多边形并计算面积，支持4-7边形 | Input: 顶点数量(4-7), 各顶点坐标(x,y) | Output: 是否为凸多边形及面积 | Rust改进: Vec<(f64,f64)>存储坐标点类型安全, 迭代器和窗口操作避免手动索引, 核心算法提取为纯函数便于测试, Result类型处理边界情况, 所有权系统避免数据拷贝', category: 'Mathematics' },
  { id: '5-b9', name: '5-b9', tier: 'tier2', description: '数独验证器 (Sudoku Validator) - 验证9×9矩阵是否为有效的数独解，检查行、列和3×3宫格 | Input: 9×9矩阵，值为1-9 | Output: 是否为有效数独解 | Rust改进: HashSet检测重复比bool数组更符合Rust习惯, 迭代器和函数式编程避免嵌套循环, 验证逻辑提取为独立函数便于单元测试, Result类型处理输入错误, chunks方法处理3×3宫格更简洁', category: 'Algorithms' },
  { id: '5-b10', name: '5-b10', tier: 'tier2', description: '年历生成器 (Calendar Generator) - 显示指定年份的完整日历，按季度排列（每季度3个月并排显示） | Input: 年份[1900-2100] | Output: 全年12个月日历，按季度分组显示 | Rust改进: Calendar结构体封装日历数据和逻辑, const数组和迭代器避免重复代码, Vec<Vec<u8>>存储每月每天的星期避免固定大小数组, 消除全局可变状态所有数据通过参数传递, Zeller公式计算星期', category: 'Fundamentals' },
  { id: '5-b11', name: '5-b11', tier: 'tier2', description: '数字转中文货币 (Number to Chinese Currency) - 将数字转换为中文大写金额格式（壹贰叁肆伍陆柒捌玖拾佰仟万亿圆角分整） | Input: [0, 100亿), 最多2位小数 | Output: 中文大写金额 | Rust改进: ChineseNumber结构体封装转换逻辑, const静态字符串数组避免运行时拼接, 数字分解和格式化逻辑分离提高可测试性, String::with_capacity预分配内存提高性能, 消除全局可变状态, builder模式构建结果字符串', category: 'Fundamentals' },
  { id: '5-b12', name: '5-b12', tier: 'tier2', description: '字符串操作库 (String Manipulation Library) - 实现tj_str系列函数：strlen/strcat/strncat/strcpy/strncpy/strcmp/strcasecmp/strncmp/strcasencmp/strupr/strlwr/strchr/strstr/strrchr/strrstr/strrev | Input: 字符串和操作参数 | Output: 操作结果 | Rust改进: &str和String类型自动处理UTF-8编码和内存安全, 标准库字符串方法避免手动索引和越界风险, Option<usize>返回查找结果比C++的0/-1更清晰, 迭代器和函数式编程代码更简洁高效, 所有函数都是纯函数无副作用, 完整的单元测试覆盖所有边界情况', category: 'Data Structures' },
  { id: '5-b3', name: '5-b3', tier: 'tier2', description: '日期验证与天数计算 (Date Validation and Day-of-Year Calculation) - 验证日期合法性并计算是一年中的第几天，支持闰年判断 | Input: 年(2000-2030), 月(1-12), 日 | Output: 一年中的第几天或错误信息 | Rust改进: 使用Result<T, E>类型进行错误处理避免多层嵌套if-else, 使用const数组和match表达式更简洁, 使用迭代器sum()方法避免手动循环累加, 自定义错误类型提供更好的错误信息', category: 'Fundamentals' },
  { id: '5-b4', name: '5-b4', tier: 'tier2', description: '成绩频率统计 (Score Frequency Counter) - 统计每个分数出现的次数并按分数降序输出 | Input: 成绩数组(最多1000个), 以-1结尾 | Output: 分数与人数的对应关系(降序) | Rust改进: 使用Vec<i32>动态数组避免固定大小限制, 使用BTreeMap统计频率比数组更高效(O(n log n)而非O(n*101)), BTreeMap自动排序避免手动排序, 使用fold迭代器方法统计频率', category: 'Data Structures' },
  { id: '5-b5', name: '5-b5', tier: 'tier2', description: '成绩排名系统 (Score Ranking System) - 对成绩进行排名，相同分数并列且占用排名位置 | Input: 成绩数组(最多1000个), 以-1结尾 | Output: 每个分数对应的排名(降序) | Rust改进: 使用Vec<i32>替代固定数组, sort_unstable_by降序排序性能优于冒泡排序(O(n log n) vs O(n²)), 提取排名计算为独立函数便于测试, 使用Result处理输入错误', category: 'Algorithms' },
  { id: '5-b6', name: '5-b6', tier: 'tier2', description: '汉诺塔可视化 (Tower of Hanoi with Visualization) - 汉诺塔递归求解并显示每一步的移动过程和柱子状态 | Input: 层数(1-10), 起始柱(A-C), 目标柱(A-C) | Output: 每步移动详情和三个柱子的实时状态 | Rust改进: 使用enum Tower替代字符提供类型安全, 使用Vec<i32>作为栈替代固定数组, 使用struct HanoiState封装状态避免全局变量, match表达式替代switch强制穷尽匹配, 提取格式化逻辑为独立函数', category: 'Algorithms' },
  { id: '5-b13', name: '5-b13', tier: 'tier2', description: '扫雷网格生成器 (Minesweeper Grid Generator) - 生成10×26扫雷网格，随机放置50个地雷并计算周围地雷数 | Input: 无 | Output: 10行26列网格，*表示地雷，数字表示周围地雷数 | Rust改进: HashSet确保不重复放置地雷避免while循环性能问题, checked_sub安全处理边界避免溢出, 函数式编程风格提高代码简洁性', category: 'Algorithms' },
  { id: '5-b14', name: '5-b14', tier: 'tier2', description: '扫雷验证器 (Minesweeper Validator) - 验证扫雷网格的地雷数量(50个)和数字正确性 | Input: 10行26列网格 | Output: "正确"或"错误" | Rust改进: enum Cell类型安全表示格子状态, Result<(), String>返回明确错误信息, checked_sub安全处理边界, 迭代器filter统计地雷数', category: 'Algorithms' },
  { id: '5-b15', name: '5-b15', tier: 'tier2', description: '字符分类统计 (Character Classification) - 统计3行输入中大写、小写、数字、空格和其他字符的数量 | Input: 3行任意字符 | Output: 各类字符计数 | Rust改进: chars()迭代器自动处理UTF-8无需手动处理多字节字符, match表达式替代if-else链, 结构体CharCounts封装计数提供类型安全', category: 'Fundamentals' },
  { id: '5-b16', name: '5-b16', tier: 'tier2', description: '学生成绩管理 (Student Grade Management) - 输入10个学生信息，按成绩降序排序并输出不及格名单 | Input: 学号 姓名 成绩(×10) | Output: 不及格学生列表 | Rust改进: 结构体Student代替字符串解析提供类型安全, sort_by_key O(n log n)替代冒泡排序O(n²), filter函数式过滤更简洁', category: 'Data Structures' },
  { id: '5-b17', name: '5-b17', tier: 'tier2', description: '密码生成器 (Password Generator) - 根据配置生成10个随机密码，满足大小写字母、数字和特殊字符的最小数量要求 | Input: 长度(12-16), 大写数(≥2), 小写数(≥2), 数字数(≥2), 特殊字符数(≥2) | Output: 10个随机密码 | Rust改进: thread_rng()线程安全随机数, shuffle() Fisher-Yates算法O(n)替代手动交换, Result类型优雅处理输入验证', category: 'Algorithms' },
  { id: '5-b18', name: '5-b18', tier: 'tier2', description: '密码验证器 (Password Validator) - 验证10个密码是否都符合长度和字符类型要求 | Input: 长度, 大写数, 小写数, 数字数, 特殊字符数, 10个密码 | Output: "正确"或"错误" | Rust改进: 结构体PasswordRequirements封装验证规则, 迭代器filter统计字符类型避免手动循环, Result<bool, String>处理验证逻辑错误信息更清晰', category: 'Algorithms' },
  { id: '6-b1', name: '6-b1', tier: 'tier2', description: '整数提取器 (Integer Extractor) - 从混合字符串中提取所有整数，非数字字符作为分隔符 | Input: 混合字符串(如"abc123def456") | Output: 提取的整数列表 | Rust改进: 使用迭代器和peekable()优雅处理字符流, Vec<i32>自动扩展避免溢出, checked_mul/checked_add防止整数溢出, 零unsafe代码保证内存安全', category: 'String Processing' },
  { id: '6-b2', name: '6-b2', tier: 'tier2', description: '回文串判断器 (Palindrome Checker) - 判断字符串是否为回文串(正读反读相同) | Input: 字符串(长度<80) | Output: yes/no | Rust改进: 使用chars()处理Unicode字符(支持中文等多字节字符), 使用zip+rev双端迭代器比较, 提取纯函数便于测试, 时间复杂度O(n/2)', category: 'String Processing' },
  { id: '6-b3', name: '6-b3', tier: 'tier2', description: '二进制转十进制 (Binary to Decimal Converter) - 将二进制字符串(0/1组成)转换为十进制数 | Input: 二进制串(长度≤32) | Output: 十进制数 | Rust改进: 使用u32::from_str_radix()标准库函数避免手动计算, Result类型处理解析错误, 添加边界检查防止溢出, 使用fold迭代器实现手动转换展示函数式编程', category: 'Number Systems' },
  { id: '6-b4', name: '6-b4', tier: 'tier2', description: '字符串操作函数库 (String Operations Library) - 实现自定义字符串操作函数(strlen/strcat/strcmp/strchr/strrev等) | Input: 各函数测试用例 | Output: 函数执行结果 | Rust改进: 使用&str和String替代C风格字符串提供内存安全, 返回Option处理空值避免空指针, 使用Ordering枚举替代整数比较, 迭代器和函数式编程提高可读性', category: 'String Processing' },

  // Tier 3 Projects (7-b*, 8-b*, 9-b*)
  { id: '7-b1', name: '7-b1', tier: 'tier3', description: 'Unix时间戳转换器 (Unix Timestamp Converter) - 将Unix时间戳转换为UTC+8北京时间，支持负时间戳和闰年处理 | Input: Unix时间戳(秒) | Output: YYYY-M-D H:M:S格式 | Rust改进: 使用chrono库避免手动闰年计算, DateTime<Utc>类型安全, 零unsafe代码, Result错误处理, 时区API正确处理UTC+8', category: 'Advanced' },
  { id: '7-b2', name: '7-b2', tier: 'tier3', description: 'KFC点餐系统 (KFC Ordering System) - 支持单品点餐和优惠套餐自动计算，使用贪心算法选择最优折扣组合 | Input: 字母代表菜品(A-Z) | Output: 订单明细和折后总价 | Rust改进: HashMap计数替代固定数组, 结构体和常量数组零成本抽象, 贪心算法优先最大折扣, 迭代器函数式编程, Result类型输入验证', category: 'Advanced' },
  { id: '8-b1', name: '8-b1', tier: 'tier3', description: '随机数据生成器 (Random Data Generator) - 为测试生成指定大小的随机数据文件，支持多种生成策略 | Input: 数据大小, 生成策略 | Output: 多个随机数据文件 | Rust改进: rand crate线程安全RNG, 配置化生成策略(任意字节/ASCII/数字/可见字符), Result错误处理, 迭代器生成数据, RAII自动资源管理', category: 'File Operations' },
  { id: '8-b2', name: '8-b2', tier: 'tier3', description: 'Hex文件转二进制 (Hex to Binary Converter) - 将hex格式文本文件转换为二进制文件，解析特定格式的十六进制数据 | Input: hex格式文件 | Output: 二进制文件 | Rust改进: BufReader/BufWriter提高I/O性能, char::to_digit()安全解析, 提取纯函数便于测试, 迭代器处理行, 自动资源管理无需手动关闭', category: 'File Operations' },
  { id: '9-b1', name: '9-b1', tier: 'tier3', description: '最大值查找器 (Maximum Value Finder) - 查找2-4个正整数中的最大值 | Input: 数量num(2-4), num个正整数 | Output: max=最大值 | Rust改进: 使用泛型和Iterator::max()替代函数重载, Result<T,E>错误处理替代cin.fail(), 零unsafe代码完全内存安全, 提取核心逻辑为纯函数便于单元测试', category: 'Advanced' },
  { id: '9-b2', name: '9-b2', tier: 'tier3', description: '最小值查找器 (Minimum Value Finder) - 查找2-4个正整数中的最小值 | Input: 数量num(2-4), num个正整数 | Output: min=最小值 | Rust改进: 使用泛型和切片代替默认参数避免sentinel模式, Iterator::min()提供零成本抽象, Result<T,E>类型安全错误处理, 提取验证逻辑为纯函数消除重复代码', category: 'Advanced' },
  { id: '9-b3-1', name: '9-b3-1', tier: 'tier3', description: '类型大小比较器 (Type Size Comparator) - 使用泛型函数比较两个类型的内存大小 | Input: 两个不同类型的参数 | Output: 参数1所占空间 </>=/== 参数2所占空间 | Rust改进: 使用std::mem::size_of和泛型trait bounds实现类型安全, std::cmp::Ordering枚举提供类型安全的比较结果, 零unsafe代码依赖编译时类型信息, 利用零成本抽象编译时确定所有大小', category: 'Advanced' },
  { id: '9-b3-2', name: '9-b3-2', tier: 'tier3', description: '泛型求和溢出演示 (Generic Sum Overflow Demo) - 演示不同整数类型的求和溢出行为 | Input: 整数n | Output: 1到n的和(溢出时环绕) | Rust改进: 使用wrapping_add明确表达溢出行为避免未定义行为, 泛型trait bounds提供类型安全, 明确区分checked和wrapping两种语义, 添加comprehensive单元测试覆盖各种溢出场景', category: 'Advanced' },
  { id: '9-b4-1', name: '9-b4-1', tier: 'tier3', description: '三角形面积计算器-叉积法 (Triangle Area Calculator - Cross Product) - 使用向量叉积公式计算三角形面积 | Input: 三个顶点坐标(x1,y1), (x2,y2), (x3,y3) | Output: 面积或-1(共线) | Formula: Area = |AB × AC| / 2 | Rust改进: 使用Copy trait的Point结构体避免克隆, Option<f64>代替-1表示错误, const fn构造函数支持编译期计算, 使用i64避免i32乘法溢出', category: 'Advanced' },
  { id: '9-b4-2', name: '9-b4-2', tier: 'tier3', description: '三角形面积计算器-叉积法v2 (Triangle Area Calculator - Cross Product v2) - 使用向量叉积公式计算三角形面积(改进版) | Input: 三个顶点坐标(x1,y1), (x2,y2), (x3,y3) | Output: 面积或-1(共线) | Formula: Area = |AB × AC| / 2 | Rust改进: 强类型Point和Triangle结构体保证类型安全, Option<f64>代替-1错误码, 零unsafe代码完全内存安全(原C++使用friend), derive宏自动实现Debug/Clone/Copy等trait', category: 'Advanced' },

  // Games
  { id: 'hanoi', name: 'Hanoi Tower', tier: 'game', description: '汉诺塔 (Tower of Hanoi) - 经典汉诺塔益智游戏，将所有圆盘从起始柱移动到目标柱 | Rust改进: 类型安全的Pillar枚举避免无效柱子, 完整的单元测试覆盖(75+ tests), Result类型的错误处理替代异常', category: 'Games' },
  { id: 'minesweeper', name: 'Minesweeper', tier: 'game', description: '扫雷 (Minesweeper) - 经典扫雷游戏，标记所有地雷并揭开所有安全格子 | Rust改进: 首次点击安全区保证(3x3区域无雷), 模块化设计(board/mine_gen/flood_fill/validation分离), 完整的单元测试覆盖(80+ tests)', category: 'Games' },
];

export const getTierColor = (tier: Project['tier']): string => {
  switch (tier) {
    case 'tier1': return 'bg-blue-500';
    case 'tier2': return 'bg-green-500';
    case 'tier3': return 'bg-purple-500';
    case 'game': return 'bg-red-500';
    default: return 'bg-gray-500';
  }
};

export const getTierLabel = (tier: Project['tier']): string => {
  switch (tier) {
    case 'tier1': return 'Tier 1';
    case 'tier2': return 'Tier 2';
    case 'tier3': return 'Tier 3';
    case 'game': return 'Game';
    default: return 'Unknown';
  }
};
