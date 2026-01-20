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

  // Tier 2 Projects (5-b*, 6-b*)
  { id: '5-b1', name: '5-b1', tier: 'tier2', description: 'Array operations', category: 'Data Structures' },
  { id: '5-b2', name: '5-b2', tier: 'tier2', description: 'Sorting algorithms', category: 'Algorithms' },
  { id: '6-b1', name: '6-b1', tier: 'tier2', description: 'Advanced algorithms', category: 'Algorithms' },
  { id: '6-b2', name: '6-b2', tier: 'tier2', description: 'Data processing', category: 'Data Structures' },

  // Tier 3 Projects (7-b*, 8-b*, 9-b*)
  { id: '7-b1', name: '7-b1', tier: 'tier3', description: 'Date/time calculations', category: 'Advanced' },
  { id: '7-b2', name: '7-b2', tier: 'tier3', description: 'Time handling', category: 'Advanced' },
  { id: '8-b1', name: '8-b1', tier: 'tier3', description: 'File I/O', category: 'File Operations' },
  { id: '8-b2', name: '8-b2', tier: 'tier3', description: 'File processing', category: 'File Operations' },
  { id: '9-b1', name: '9-b1', tier: 'tier3', description: 'Complex algorithms', category: 'Advanced' },

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
