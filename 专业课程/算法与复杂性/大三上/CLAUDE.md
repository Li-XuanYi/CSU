# CLAUDE.md

以 2022 年高晓沨期末考试为蓝本，命制风格一致的练习题。难度以讲义和实验题为准。

## 仓库结构

```
算法与复杂性/
├── Slides/         # 课程幻灯片 PDF（21个）
├── Lab/            # 实验题与解答（含算法应用场景素材）
│   ├── pdf/        # 解答 PDF
│   └── Lab*.tex    # LaTeX 源文件
├── Final/          # 期末考试
│   ├── 2022-高晓沨/2022-高晓沨.pdf  # 主蓝本（命制风格、题型的核心依据）
│   └── 其他年份/（陈翌佳老师班，可参考出题思路）
├── Note.md         # 复习笔记
└── .claude/skills/
```

## 蓝本分析

### 总体结构（参考蓝本）

| 部分 | 题型 | 题量 | 分值 |
|------|------|------|------|
| Part 1 | 单选题 | 10 × 3.5' | 35' |
| Part 2 | 问答题 | 4 大题（含子题） | 65' |
| **合计** | | | **100'** |

### 选择题（MC1~MC10）

| 题号 | 知识点 | 章节 | 考核能力 |
|------|--------|------|----------|
| MC1 | 嵌套循环时间复杂度 | Slide03 | 计算复杂度 |
| MC2 | 排序算法稳定性与最坏情况复杂度 | Slide03/Slide05 | 概念辨析 |
| MC3 | 拟阵与贪心算法对应关系 | Slide07 | 概念对应 |
| MC4 | 排序网络组件性质（选 FALSE） | Slide05 | 概念辨析 |
| MC5 | 钢条切割 DP 递推式（选 FALSE） | Slide08 | 递推式理解 |
| MC6 | 硬币找零贪心最优性条件 | Slide06 | 贪心条件分析 |
| MC7 | 线性规划对偶转换 | Slide09 | 对偶规则 |
| MC8 | Floyd-Warshall 循环顺序 | Slide13 | 算法细节 |
| MC9 | DFS/BFS 遍历顺序 | Slide12 | 搜索过程 |
| MC10 | 多栈 push 摊还分析（最好/平均/最坏） | Slide10 | 复杂度分析 |

**风格要点**：4 选 1 单选。

### 问答题（QA1~QA4）

| 题号 | 主题 | 分值 | 子题结构 |
|------|------|------|----------|
| QA1 | Divide-and-Conquer | 15' | (a) 默写 Master Theorem 5' → (b) 判断能否用 5' → (c) 设计算法 5' |
| QA2 | Greedy & DP | 18' | (a) 活动选择贪心 9' → (b) 0/1 背包 DP 9' |
| QA3 | Network Flow | 17' | (a) 边不相交路径 8' → (b) 顶点分裂 9' |
| QA4 | NP & Reduction | 15' | (a) certificate/certifier 5' → (b) NP-complete 5' → (c) 变体分析 5' |

**风格要点**：子题递进（基础→深入）；问答题分为算法设计题和证明题；算法设计题如分治法、贪心算法、动态规划、线性规划等等，证明题含正确类证明、效率类证明。

### 难度比例

难度以讲义（Slides）和实验题（Lab）中的知识深度为准，蓝本仅约束题型和分值分配。

| 层级 | 占比 | 考核要求 |
|------|------|----------|
| 基础概念 | ~30% | 记忆、识别、对应 |
| 方法应用 | ~40% | 计算、推导、应用算法 |
| 综合证明 | ~30% | 设计、证明、归约 |

## 工作流

### 1. 加载参考材料

出题前先读取蓝本试卷参考题型和分值分配：

```bash
python -c "
import fitz, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
doc = fitz.open('final/2022-高晓沨/2022-高晓沨.pdf')
for i in range(doc.page_count):
    print(f'--- Page {i+1} ---')
    print(doc[i].get_text())
doc.close()
"
```

同时读取对应章节的 Lab 实验题来校准难度：

```bash
python -c "
import fitz, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
doc = fitz.open('Lab/pdf/LabXX.pdf')
for i in range(doc.page_count):
    print(f'--- Page {i+1} ---')
    print(doc[i].get_text())
doc.close()
"
```

### 2. 选择题出题

**步骤一**：读取对应 Slide 全面提取知识点，可以直接将讲义中的定义、定理、例子、代码等作为题目素材。也可参考 Lab/ 中对应章节的实验题来校准难度。

```bash
python -c "
import fitz, sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
doc = fitz.open('Slides/SlideXX-xxxx.pdf')
for i in range(doc.page_count):
    print(f'--- Page {i+1} ---')
    print(doc[i].get_text())
doc.close()
"
```

**步骤二**：出题规则：
- 4 选 1 单选题，每题 3.5'
- 中文命制，部分可问"选 FALSE"
- 可给代码分析复杂度、概念辨析、条件判断、推导计算
- 在题号后标注考点

示例：
```
3.5 分
**1.（Nested Loop Complexity, Slide03）** 题目描述
O (A) ...
O (B) ...
O (C) ...
O (D) ...
```

### 3. 问答题出题

**步骤一**：读取对应 Slide 提取知识点，可以直接将讲义中的定义、定理、例子、代码等作为题目素材。也可参考 Lab/ 中对应章节的实验题来校准难度（同上）。

**步骤二**：出题规则：
- 每大题含 2~3 个子题，从基础到深入
- 分值参考蓝本（如 5'+5'+5' 或 8'+9'）
- 必含伪代码 + 复杂度分析 + 正确性证明
- 中文命制，专业术语保留英文
- 每道子题在题号后标注考点

子题结构参考蓝本四种模式：

| 模式 | 示例 | 结构 |
|------|------|------|
| 定理+应用+设计 | QA1 分治 | (a)默写定理 → (b)判断能否用 → (c)设计算法 |
| 场景算法+变体 | QA2 贪心&DP | (a)基础贪心 → (b)加约束用 DP |
| 基础流+加强流 | QA3 网络流 | (a)边不相交 → (b)顶点+边均不相交 |
| 概念+证明+变体 | QA4 NP | (a)certificate → (b)NP-complete → (c)变体分析 |

### 4. 生成专题练习

1. 用户指定专题（如"分治"、"动态规划"、"网络流"等）
2. 加载蓝本参考题型风格
3. 读取对应 Slide，结合 Lab 校准难度，可以直接提取 Slide 中的内容作为题目素材，全面覆盖该专题的所有核心知识点
4. 为该专题出题：选择题 3~5 题 + 问答题 1~2 题
5. 选择题答案与解析合并；问答题给出完整解答（伪代码+复杂度+证明）

### 5. 输出格式

以 Markdown 交付，包含题目和答案。排版美观，不照搬蓝本格式。

```
# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：xxx

---

## 一、选择题

**1.（Nested Loop Complexity, Slide03）** 题目描述

- (A) ...
- (B) ...
- (C) ...
- (D) ...

**答案**：... **解析**：...

---

## 二、问答题

### 1. 标题（xx 分）

#### （1）子问题（Master Theorem, Slide04）（x'）

...

**答案**：...
```

### 6. 子智能体

允许使用 Agent/Workflow 加速：
- 并行读取多个 Slide
- 分章同时出题
- 交叉核验答案

### 7. Slide 章节表

| Slide | 主题 |
|-------|------|
| Slide01 | Prologue |
| Slide02 | Symbols |
| Slide03 | Algorithm Analysis |
| Slide04 | Divide & Conquer |
| Slide05 | Sorting Network |
| Slide06 | Greedy |
| Slide07 | Matroid |
| Slide08 | Dynamic Programming |
| Slide09 | Linear Programming |
| Slide10 | Amortized Analysis |
| Slide11 | Graph |
| Slide12 | DFS/BFS |
| Slide13 | Shortest Path |
| Slide14 | Network Flow |
| Slide15 | Turing Machine |
| Slide16 | NP Reduction |
| Slide17-18 | Approximation |
| Slide19 | Randomized Algorithm |
| Slide20 | Online Algorithm |

### 8. 参考依据

| 依据 | 路径 | 用途 |
|------|------|------|
| 讲义 | Slides/ | 知识范围与深度（难度来源） |
| 实验题 | Lab/（含 pdf/ 解答和 Lab*.tex 源文件） | 算法应用深度和计算难度（难度来源） |
| 主蓝本 | final/2022-高晓沨/2022-高晓沨.pdf | 题型、分值分配、出题风格（核心依据） |
| 标准答案 | final/2022-高晓沨/2022-高晓沨-答案.md | 解答深度和表述方式 |
| 其他年份卷 | final/（陈翌佳老师班） | 参考题目命制方式，虽出自不同老师但出题思路可借鉴 |
