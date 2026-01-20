import { Project } from '../types';

// All projects from the Rust migration
export const projects: Project[] = [
  // Tier 1 Projects (3-b*, 4-b*)
  { id: '3-b2', name: '3-b2', tier: 'tier1', description: '几何计算 (Geometric Calculations) - 根据输入的半径和高度，计算圆周长、圆面积、圆球表面积、圆球体积和圆柱体积 | Input: 半径 r, 高度 h | Output: 保留2位小数', category: 'Mathematics' },
  { id: '3-b3', name: '3-b3', tier: 'tier1', description: '整数位数提取 (Integer Digit Extraction) - 输入1到30000之间的整数，使用取模和除法运算提取各个数位 | Input: [1, 30000] | Output: 万位、千位、百位、十位、个位', category: 'Fundamentals' },
  { id: '3-b4', name: '3-b4', tier: 'tier1', description: '大数位数提取 (Large Number Digit Extraction) - 提取数字的每一位（十亿位至分位），使用中文货币单位输出 | Input: [0, 100亿), 最多2位小数 | Output: 十亿/亿/千万/百万/十万/万/千/百/十/圆/角/分', category: 'Mathematics' },
  { id: '3-b5', name: '3-b5', tier: 'tier1', description: '三角形面积计算 (Triangle Area Calculation) - 输入两边及夹角（度），计算面积 | Input: 边a, 边b, 夹角度数 | Formula: S = (1/2)×a×b×sin(θ) | Output: 保留3位小数', category: 'Mathematics' },
  { id: '3-b6', name: '3-b6', tier: 'tier1', description: '日期计算器 (Date Calculator) - 计算给定日期是该年的第几天，支持闰年判断', category: 'Fundamentals' },
  { id: '3-b7', name: '3-b7', tier: 'tier1', description: '人民币大写转换器 (RMB Converter) - 将数字金额转换为中文货币大写格式', category: 'Fundamentals' },
  { id: '3-b8', name: '3-b8', tier: 'tier1', description: '找零计算器 (Change Calculator) - 使用贪心算法计算最少纸币和硬币的找零方案', category: 'Fundamentals' },
  { id: '3-b9', name: '3-b9', tier: 'tier1', description: '完全数查找器 (Perfect Number Finder) - 查找1000以内的完全数及其因子', category: 'Fundamentals' },
  { id: '3-b10', name: '3-b10', tier: 'tier1', description: '输入验证器 (Input Validator) - 读取并验证0-100之间的整数输入', category: 'Fundamentals' },
  { id: '3-b11', name: '3-b11', tier: 'tier1', description: '九九乘法表生成器 (Multiplication Table Generator) - 生成格式化的9×9乘法表，使用迭代器优化输出', category: 'Fundamentals' },
  { id: '3-b12', name: '3-b12', tier: 'tier1', description: '月历打印器 (Calendar Printer) - 显示指定年月的日历，支持闰年判断和星期对齐', category: 'Fundamentals' },
  { id: '3-b13', name: '3-b13', tier: 'tier1', description: '三数求和问题 (Three-Number Sum Puzzle) - 查找和为1953且使用1-9不重复数字的三位数组合，使用位掩码优化', category: 'Fundamentals' },
  { id: '4-b11', name: '4-b11', tier: 'tier1', description: 'Fibonacci数列性能计时 (Fibonacci Sequence with Performance Timing) - 计算Fibonacci数列第n项，对比递归、迭代和记忆化三种算法的性能差异 | Input: 项数[1-46] | Output: Fibonacci值和算法耗时', category: 'Algorithms' },
  { id: '4-b12', name: '4-b12', tier: 'tier1', description: '幂次检测器 (Power Base Checker) - 使用递归检查一个数是否是指定基数的幂 | Input: 整数num, 基数base(≥2) | Output: 是否为base的幂', category: 'Algorithms' },
  { id: '4-b13', name: '4-b13', tier: 'tier1', description: '递归三角塔打印器 (Recursive Triangle Tower Printer) - 使用递归打印正/倒三角形字母塔 | Input: 结束字符(A-Z) | Output: 正三角塔(中间为A)和倒三角塔(两边为A)', category: 'Algorithms' },
  { id: '4-b14', name: '4-b14', tier: 'tier1', description: '一元二次方程求解器 (Quadratic Equation Solver) - 求解ax²+bx+c=0的根，支持实根和复根 | Input: 系数a,b,c | Output: 两个相等实根/两个不等实根/两个共轭复根', category: 'Mathematics' },
  { id: '4-b15', name: '4-b15', tier: 'tier1', description: '一元二次方程求解器 (Quadratic Equation Solver) - 求解ax²+bx+c=0，支持实根和复根 | Input: 系数a,b,c | Output: 两个不等实根/两个相等实根/两个虚根/不是二次方程', category: 'Mathematics' },
  { id: '4-b16', name: '4-b16', tier: 'tier1', description: '一元二次方程求解器-模块化版本 (Quadratic Equation Solver - Modular) - 模块化设计的方程求解器，每个解类型独立格式化 | Input: 系数a,b,c | Output: 格式化的方程解', category: 'Mathematics' },
  { id: '4-b17', name: '4-b17', tier: 'tier1', description: '一元二次方程求解器-复数版本 (Quadratic Equation Solver - Complex Numbers) - 使用Complex结构体表示复数的方程求解器 | Input: 系数a,b,c | Output: 实根或复数根', category: 'Mathematics' },

  // Tier 2 Projects (5-b*, 6-b*)
  { id: '5-b1', name: '5-b1', tier: 'tier2', description: '有序数组插入 (Insert into Sorted Array) - 将新数字插入到已排序数组中并保持有序 | Input: 升序正整数数组(最多20个), 待插入正整数 | Output: 插入后的有序数组', category: 'Data Structures' },
  { id: '5-b2', name: '5-b2', tier: 'tier2', description: '灯泡开关问题 (Light Bulb Toggle Problem) - 100个灯泡初始全灭，第i个人切换所有i的倍数编号的灯泡，求最后哪些灯泡亮着 | Input: 无 | Output: 最终亮着的灯泡编号', category: 'Algorithms' },
  { id: '5-b7', name: '5-b7', tier: 'tier2', description: '汉诺塔可视化 (Hanoi Tower Visualization) - 使用控制台图形演示汉诺塔求解过程，支持自定义起始柱、目标柱和移动速度 | Input: 层数(1-10), 起始柱(A-C), 目标柱(A-C), 速度(0-5), 是否显示内部数组 | Output: 垂直显示柱子状态和移动步骤', category: 'Visualization' },
  { id: '5-b8', name: '5-b8', tier: 'tier2', description: '凸多边形面积计算器 (Convex Polygon Area Calculator) - 使用叉积法判断凸多边形并计算面积，支持4-7边形 | Input: 顶点数量(4-7), 各顶点坐标(x,y) | Output: 是否为凸多边形及面积', category: 'Mathematics' },
  { id: '5-b9', name: '5-b9', tier: 'tier2', description: '数独验证器 (Sudoku Validator) - 验证9×9矩阵是否为有效的数独解，检查行、列和3×3宫格 | Input: 9×9矩阵，值为1-9 | Output: 是否为有效数独解', category: 'Algorithms' },
  { id: '5-b10', name: '5-b10', tier: 'tier2', description: '年历生成器 (Calendar Generator) - 显示指定年份的完整日历，按季度排列（每季度3个月并排显示） | Input: 年份[1900-2100] | Output: 全年12个月日历，按季度分组显示', category: 'Fundamentals' },
  { id: '5-b11', name: '5-b11', tier: 'tier2', description: '数字转中文货币 (Number to Chinese Currency) - 将数字转换为中文大写金额格式（壹贰叁肆伍陆柒捌玖拾佰仟万亿圆角分整） | Input: [0, 100亿), 最多2位小数 | Output: 中文大写金额', category: 'Fundamentals' },
  { id: '5-b12', name: '5-b12', tier: 'tier2', description: '字符串操作库 (String Manipulation Library) - 实现tj_str系列函数：strlen/strcat/strncat/strcpy/strncpy/strcmp/strcasecmp/strncmp/strcasencmp/strupr/strlwr/strchr/strstr/strrchr/strrstr/strrev | Input: 字符串和操作参数 | Output: 操作结果', category: 'Data Structures' },
  { id: '5-b3', name: '5-b3', tier: 'tier2', description: '日期验证与天数计算 (Date Validation and Day-of-Year Calculation) - 验证日期合法性并计算是一年中的第几天，支持闰年判断 | Input: 年(2000-2030), 月(1-12), 日 | Output: 一年中的第几天或错误信息', category: 'Fundamentals' },
  { id: '5-b4', name: '5-b4', tier: 'tier2', description: '成绩频率统计 (Score Frequency Counter) - 统计每个分数出现的次数并按分数降序输出 | Input: 成绩数组(最多1000个), 以-1结尾 | Output: 分数与人数的对应关系(降序)', category: 'Data Structures' },
  { id: '5-b5', name: '5-b5', tier: 'tier2', description: '成绩排名系统 (Score Ranking System) - 对成绩进行排名，相同分数并列且占用排名位置 | Input: 成绩数组(最多1000个), 以-1结尾 | Output: 每个分数对应的排名(降序)', category: 'Algorithms' },
  { id: '5-b6', name: '5-b6', tier: 'tier2', description: '汉诺塔可视化 (Tower of Hanoi with Visualization) - 汉诺塔递归求解并显示每一步的移动过程和柱子状态 | Input: 层数(1-10), 起始柱(A-C), 目标柱(A-C) | Output: 每步移动详情和三个柱子的实时状态', category: 'Algorithms' },
  { id: '5-b13', name: '5-b13', tier: 'tier2', description: '扫雷网格生成器 (Minesweeper Grid Generator) - 生成10×26扫雷网格，随机放置50个地雷并计算周围地雷数 | Input: 无 | Output: 10行26列网格，*表示地雷，数字表示周围地雷数', category: 'Algorithms' },
  { id: '5-b14', name: '5-b14', tier: 'tier2', description: '扫雷验证器 (Minesweeper Validator) - 验证扫雷网格的地雷数量(50个)和数字正确性 | Input: 10行26列网格 | Output: "正确"或"错误"', category: 'Algorithms' },
  { id: '5-b15', name: '5-b15', tier: 'tier2', description: '字符分类统计 (Character Classification) - 统计3行输入中大写、小写、数字、空格和其他字符的数量 | Input: 3行任意字符 | Output: 各类字符计数', category: 'Fundamentals' },
  { id: '5-b16', name: '5-b16', tier: 'tier2', description: '学生成绩管理 (Student Grade Management) - 输入10个学生信息，按成绩降序排序并输出不及格名单 | Input: 学号 姓名 成绩(×10) | Output: 不及格学生列表', category: 'Data Structures' },
  { id: '5-b17', name: '5-b17', tier: 'tier2', description: '密码生成器 (Password Generator) - 根据配置生成10个随机密码，满足大小写字母、数字和特殊字符的最小数量要求 | Input: 长度(12-16), 大写数(≥2), 小写数(≥2), 数字数(≥2), 特殊字符数(≥2) | Output: 10个随机密码', category: 'Algorithms' },
  { id: '5-b18', name: '5-b18', tier: 'tier2', description: '密码验证器 (Password Validator) - 验证10个密码是否都符合长度和字符类型要求 | Input: 长度, 大写数, 小写数, 数字数, 特殊字符数, 10个密码 | Output: "正确"或"错误"', category: 'Algorithms' },
  { id: '6-b1', name: '6-b1', tier: 'tier2', description: '整数提取器 (Integer Extractor) - 从混合字符串中提取所有整数，非数字字符作为分隔符 | Input: 混合字符串(如"abc123def456") | Output: 提取的整数列表', category: 'String Processing' },
  { id: '6-b2', name: '6-b2', tier: 'tier2', description: '回文串判断器 (Palindrome Checker) - 判断字符串是否为回文串(正读反读相同) | Input: 字符串(长度<80) | Output: yes/no', category: 'String Processing' },
  { id: '6-b3', name: '6-b3', tier: 'tier2', description: '二进制转十进制 (Binary to Decimal Converter) - 将二进制字符串(0/1组成)转换为十进制数 | Input: 二进制串(长度≤32) | Output: 十进制数', category: 'Number Systems' },
  { id: '6-b4', name: '6-b4', tier: 'tier2', description: '字符串操作函数库 (String Operations Library) - 实现自定义字符串操作函数(strlen/strcat/strcmp/strchr/strrev等) | Input: 各函数测试用例 | Output: 函数执行结果', category: 'String Processing' },

  // Tier 3 Projects (7-b*, 8-b*, 9-b*)
  { id: '7-b1', name: '7-b1', tier: 'tier3', description: 'Unix时间戳转换器 (Unix Timestamp Converter) - 将Unix时间戳转换为UTC+8北京时间，支持负时间戳和闰年处理 | Input: Unix时间戳(秒) | Output: YYYY-M-D H:M:S格式', category: 'Advanced' },
  { id: '7-b2', name: '7-b2', tier: 'tier3', description: 'KFC点餐系统 (KFC Ordering System) - 支持单品点餐和优惠套餐自动计算，使用贪心算法选择最优折扣组合 | Input: 字母代表菜品(A-Z) | Output: 订单明细和折后总价', category: 'Advanced' },
  { id: '8-b1', name: '8-b1', tier: 'tier3', description: '随机数据生成器 (Random Data Generator) - 为测试生成指定大小的随机数据文件，支持多种生成策略 | Input: 数据大小, 生成策略 | Output: 多个随机数据文件', category: 'File Operations' },
  { id: '8-b2', name: '8-b2', tier: 'tier3', description: 'Hex文件转二进制 (Hex to Binary Converter) - 将hex格式文本文件转换为二进制文件，解析特定格式的十六进制数据 | Input: hex格式文件 | Output: 二进制文件', category: 'File Operations' },
  { id: '9-b1', name: '9-b1', tier: 'tier3', description: '最大值查找器 (Maximum Value Finder) - 查找2-4个正整数中的最大值 | Input: 数量num(2-4), num个正整数 | Output: max=最大值', category: 'Advanced' },
  { id: '9-b2', name: '9-b2', tier: 'tier3', description: '最小值查找器 (Minimum Value Finder) - 查找2-4个正整数中的最小值 | Input: 数量num(2-4), num个正整数 | Output: min=最小值', category: 'Advanced' },
  { id: '9-b3-1', name: '9-b3-1', tier: 'tier3', description: '类型大小比较器 (Type Size Comparator) - 使用泛型函数比较两个类型的内存大小 | Input: 两个不同类型的参数 | Output: 参数1所占空间 </>=/== 参数2所占空间', category: 'Advanced' },
  { id: '9-b3-2', name: '9-b3-2', tier: 'tier3', description: '泛型求和溢出演示 (Generic Sum Overflow Demo) - 演示不同整数类型的求和溢出行为 | Input: 整数n | Output: 1到n的和(溢出时环绕)', category: 'Advanced' },
  { id: '9-b4-1', name: '9-b4-1', tier: 'tier3', description: '三角形面积计算器-叉积法 (Triangle Area Calculator - Cross Product) - 使用向量叉积公式计算三角形面积 | Input: 三个顶点坐标(x1,y1), (x2,y2), (x3,y3) | Output: 面积或-1(共线) | Formula: Area = |AB × AC| / 2', category: 'Advanced' },
  { id: '9-b4-2', name: '9-b4-2', tier: 'tier3', description: '三角形面积计算器-叉积法v2 (Triangle Area Calculator - Cross Product v2) - 使用向量叉积公式计算三角形面积(改进版) | Input: 三个顶点坐标(x1,y1), (x2,y2), (x3,y3) | Output: 面积或-1(共线) | Formula: Area = |AB × AC| / 2', category: 'Advanced' },

  // Games
  { id: 'hanoi', name: 'Hanoi Tower', tier: 'game', description: '汉诺塔 (Tower of Hanoi) - 经典汉诺塔益智游戏，将所有圆盘从起始柱移动到目标柱', category: 'Games' },
  { id: 'minesweeper', name: 'Minesweeper', tier: 'game', description: '扫雷 (Minesweeper) - 经典扫雷游戏，标记所有地雷并揭开所有安全格子', category: 'Games' },
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
