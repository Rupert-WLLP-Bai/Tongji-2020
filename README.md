# Tongji-2020 课程作业存档

## 项目简介

本项目是 **2020 年同济大学高级语言程序设计课程** 的作业存档，包含原始 C/C++ 实现以及现代化的 Rust 版本迁移。

## 项目结构

```
Tongji-2020/
├── cpp-original/          # 原始 C/C++ 课程作业代码
│   ├── Project1/ ~ Project4/  # 早期项目
│   ├── 3-b*/ ~ 9-b*/          # 各类练习题目
│   ├── 90-b*/                 # 综合项目
│   └── 高程.sln               # Visual Studio 解决方案
├── tongji-rust/           # Rust 现代化重写版本
│   ├── src/                   # Rust 源代码
│   ├── tauri-app/             # Tauri 前端界面
│   └── tests/                 # 单元测试
└── crates/                # 共享 Rust crates
```

### 原始 C/C++ 代码 (`cpp-original/`)
- 包含所有原始课程作业的 C/C++ 实现
- 保留原始代码结构和 Visual Studio 项目文件
- 用于参考和对比学习

### Rust 迁移版本 (`tongji-rust/`)
- 完整的 Rust 重写，包含 60+ 个项目
- 改进的类型安全和错误处理
- 完善的单元测试覆盖（80+ 测试用例）
- 现代化的代码组织结构
- Tauri 前端界面支持，提供可视化交互

## 主要内容

### Tier 1 项目 (基础编程)
- 几何计算、数字处理、日期计算等基础算法实现

### Tier 2 项目 (数据结构与算法)
- 数组操作、排序算法等进阶内容

### Tier 3 项目 (高级应用)
- 文件操作、复杂算法等高级主题

### 游戏项目
- 汉诺塔 (Tower of Hanoi)
- 扫雷 (Minesweeper)

## 技术栈

- **原始版本**: C/C++, Visual Studio
- **迁移版本**: Rust, Tauri, React, TypeScript

## 作者

原作者: 2052526 信15 白俊豪

---

*本项目用于学习交流和技术展示*
