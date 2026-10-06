# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：NP 归约与 NP-完备性（Slide16）

---

## 一、选择题（3.5' × 9 = 31.5'）

---

**1.（Certificate & Certifier, Slide16）** 关于 NP 及 certificate/certifier，下列说法正确的是：

- (A) NP 中的问题都可以被确定性图灵机（DTM）在多项式时间内求解
- (B) 若问题 X 存在一个多项式时间的 certifier C(s, t)，则 X ∈ NP
- (C) COMPOSITES 问题的 certificate 是合数的全部素因子
- (D) 一个问题的 certificate 长度必须与该问题的输入规模无关

**答案：B**

**解析：**
- A 错误：NP 中的问题可被**非**确定性图灵机（NTM）在多项式时间内求解。DTM 多项式时间内可解的是 P 类问题。
- **B 正确**：这正是 NP 的定义——存在多项式时间 certifier 的判定问题构成 NP。
- C 错误：COMPOSITES 的 certificate 是**一个**非平凡因子，而非全部素因子。例如 437669 = 541 × 809，certificate 可以是 541 或 809。
- D 错误：certificate 的长度必须受输入规模的多项式限制（|t| ≤ p(|s|)），但并非与输入规模无关。

---

**2.（Polynomial-Time Reduction, Slide16）** 关于多项式时间归约（polynomial-time reduction），下列描述中正确的是：

- (A) 若 X ≤<sub>P</sub> Y 且 Y 是 NP 完全的，则 X 一定是 NP 完全的
- (B) Cook 归约允许在求解 X 的过程中多次调用求解 Y 的 oracle，而 Karp 归约只允许调用一次且必须在算法末尾
- (C) 若 X ≤<sub>P</sub> Y 且 Y 不能在多项式时间内求解，则 X 也不能在多项式时间内求解
- (D) 多项式归约的传递性是指：若 X ≤<sub>P</sub> Y 则 Y ≤<sub>P</sub> X

**答案：B**

**解析：**
- A 错误：还需要 Y ∈ NP。仅有 X ≤<sub>P</sub> Y 只能说明 Y 是 NP-难的（NP-hard）。
- **B 正确**：这正是 Cook 归约与 Karp 归约的核心区别。Cook 归约允许多次调用 oracle，Karp 归约（多项式变换）只允许在末尾调用一次。讲义 p22 指出这两者是否等价仍是一个开放问题。
- C 错误：若 X ≤<sub>P</sub> Y 且 X 不能在多项式时间内求解，则 Y 也不能。
- D 错误：传递性是指若 X ≤<sub>P</sub> Y 且 Y ≤<sub>P</sub> Z，则 X ≤<sub>P</sub> Z。

---

**3.（NP-Completeness, Slide16）** 关于 NP-完备（NP-Complete）问题，下列说法中正确的是：

- (A) 若问题 Y 是 NP-完全的，且存在多项式时间算法求解 Y，则 P = NP
- (B) 要证明一个问题 Y 是 NP-完全的，只需证明某个已知 NP-完全问题 X 满足 X ≤<sub>P</sub> Y
- (C) 所有 NP 问题都可以在指数时间内求解，因此 NP ⊆ EXP 不成立
- (D) 若 P ≠ NP，则不存在任何确定性图灵机（DTM）可以求解 NP-完全问题

**答案：A**

**解析：**
- **A 正确**：若某个 NP-完全问题 Y 有多项式时间算法，则所有 NP 问题都可多项式归约到 Y 然后求解，故 P = NP。这是 NP-完全问题的核心性质。
- B 错误：还需证明 Y ∈ NP。只满足前者说明 Y 是 NP-难的（NP-hard）。
- C 错误：NP ⊆ EXP 成立。对 NP 中的任意问题，枚举所有可能的 certificate（指数多个）并用多项式时间 certifier 验证，即可在指数时间内解决。
- D 错误：即使 P ≠ NP，DTM 仍然可以用**指数时间**求解 NP-完全问题（如穷举搜索）。NP-完全问题的困难在于不存在**多项式时间**算法，而非完全不可解。

---

**4.（Reduction Strategies, Slide16）** 关于 Slide16 中介绍的基本归约策略，下列说法中错误的是（选 FALSE）：

- (A) INDEPENDENT-SET 与 VERTEX-COVER 之间存在简单等价归约，具体构造是保持图不变，将参数 k 替换为 n − k
- (B) VERTEX-COVER 可以归约到 SET-COVER，构造方式是将每条边作为一个元素，每个顶点对应一个覆盖与其关联的边的子集
- (C) 从 3-SAT 到 INDEPENDENT-SET 的归约中，每个 clause 构造一个三角形（3 个顶点对应 3 个 literal），同时将每个 literal 与其否定相连，设置 k = 子句数
- (D) 从 3-SAT 到 3-COLOR 的归约中，只需为每个 literal 创建 1 个节点，创建 T、F 两个特殊节点

**答案：D**

**解析：**
- A 正确：IS ≡<sub>P</sub> VC 的构造。
- B 正确：VC ≤<sub>P</sub> SC 的构造：全集 U = E，每个顶点 v 对应 S<sub>v</sub> = {e ∈ E : e 与 v 关联}。
- C 正确：3-SAT ≤<sub>P</sub> IS 的标准构造。
- **D 错误**：需创建 T、F、B **三个**特殊节点（三者连成三角形），并将每个 literal 节点连接到 B。仅两个节点无法完成构造。

---

**5.（NP-Complete Problem Genres, Slide16）** Slide16 将 NP-完全问题分为六大基本类型（genres），以下分类中正确的是：

- (A) 覆盖问题（Covering problems）：SET-COVER、INDEPENDENT SET
- (B) 约束满足问题（Constraint satisfaction）：SAT、3-SAT
- (C) 划分问题（Partitioning problems）：SUBSET-SUM、KNAPSACK
- (D) 排序问题（Sequencing problems）：3D-MATCHING、TSP

**答案：B**

**解析：**
- A 错误：INDEPENDENT SET 属于**打包问题（Packing problems）**。
- **B 正确**：SAT 和 3-SAT 属于约束满足问题。
- C 错误：SUBSET-SUM 和 KNAPSACK 属于**数值问题（Numerical problems）**，划分问题是 3D-MATCHING 和 3-COLOR。
- D 错误：3D-MATCHING 属于**划分问题**，排序问题的代表是 HAM-CYCLE 和 TSP。

---

**6.（NTM & NP Definition, Slide16）** 关于非确定性图灵机（NTM）与 NP 的关系，下列说法正确的是：

- (A) 若一个问题可以被 NTM 在多项式时间内求解，则该问题一定属于 P
- (B) NP 的定义只能通过"存在多项式时间 certifier"来刻画，NTM 无法用于定义 NP
- (C) 任何一个 NTM 都可以被一个 DTM 模拟，且模拟的时间复杂度是多项式的
- (D) NP 可以被定义为那些可被 NTM 在多项式时间内求解的判定问题的集合，该定义与"存在多项式时间 certifier"的定义等价

**答案：D**

**解析：**
- A 错误：P 要求 DTM 多项式时间求解。NTM 多项式时间求解是 NP 的定义之一。
- B 错误：NP 有两种等价定义：(i) 可被 NTM 在多项式时间内求解；(ii) 存在多项式时间 certifier。"NP" 即 Nondeterministic Polynomial-time。
- C 错误：任意 NTM 可被 DTM 模拟，但模拟开销是指数级的（枚举所有分支）。但 NTM 和 DTM 识别的语言集合是相同的（递归可枚举语言），区别在于效率。
- **D 正确**：讲义 p11（certifier 定义）和 p16（NTM 定义）分别给出了这两种等价定义。

---

**7.（CIRCUIT-SAT, Slide16）** 关于第一个 NP-完全问题 CIRCUIT-SAT，下列说法错误的是（选 FALSE）：

- (A) CIRCUIT-SAT 是 Cook 和 Levin 分别独立证明的第一个 NP-完全问题
- (B) 在 CIRCUIT-SAT ≤<sub>P</sub> 3-SAT 的归约中，对于 AND 门 x<sub>0</sub> = x<sub>1</sub> ∧ x<sub>2</sub>，需添加 3 个子句：(¬x<sub>0</sub> ∨ x<sub>1</sub>)、(¬x<sub>0</sub> ∨ x<sub>2</sub>)、(x<sub>0</sub> ∨ ¬x<sub>1</sub> ∨ ¬x<sub>2</sub>)
- (C) CIRCUIT-SAT NP-完全性的证明思路是将任意 NP 问题 X 的 certifier 转化为多项式规模的电路
- (D) 在证明 3-SAT 是 NP-完全时，只需证明 3-SAT ∈ NP 即可

**答案：D**

**解析：**
- A 正确：Cook (1971) 和 Levin (1973) 分别独立证明 CIRCUIT-SAT 是 NP-完全的。
- B 正确：AND 门 x₀ = x₁ ∧ x₂ 等价于 (¬x₀ ∨ x₁) ∧ (¬x₀ ∨ x₂) ∧ (x₀ ∨ ¬x₁ ∨ ¬x₂)，恰好 3 个子句。
- C 正确：这是 Cook-Levin 定理的核心——对 certifier 的计算过程进行电路编码。
- **D 错误**：还需证明某个已知 NP-完全问题（如 CIRCUIT-SAT）可归约到 3-SAT，即 CIRCUIT-SAT ≤<sub>P</sub> 3-SAT。

---

**8.（Sequencing Problems, Slide16）** 关于排序问题（Sequencing problems）及其 NP-完全性归约，下列说法正确的是：

- (A) HAM-CYCLE 到 TSP 的归约中，若将 TSP 距离函数定义为 d(u, v) = 1（若 (u, v) ∈ E）和 d(u, v) = 2（若 (u, v) ∉ E），则 G 存在哈密顿环当且仅当 TSP 实例存在长度 ≤ n 的旅行
- (B) DIR-HAM-CYCLE 到 HAM-CYCLE 的归约中，将原图每个顶点替换为 2 个新节点
- (C) LONGEST-PATH 与 SHORTEST-PATH 都是 NP-完全的
- (D) 3-SAT 到 DIR-HAM-CYCLE 的归约中，每个变量对应 1 条路径

**答案：A**

**解析：**
- **A 正确**：讲义 p64 的构造。若 G 有哈密顿环，则 TSP 总距离 = n；反之，若 TSP 有长度 ≤ n 的旅行，则每段距离必须为 1（因 ≥ 2 时总距离 ≥ n+1 > n），故每条边在 E 中。
- B 错误：每个顶点替换为 **3** 个节点（蓝 B、绿 G、红 R），而非 2 个。
- C 错误：SHORTEST-PATH 是 P 类问题（如 Dijkstra 算法），只有 LONGEST-PATH 是 NP-完全的。
- D 错误：每个变量对应**一行路径**（多个节点组成），共 n 行对应 n 个变量。讲义构造中每条路径有 2 个方向（左→右表示 xᵢ = 1，右→左表示 xᵢ = 0），共有 2ⁿ 种可能的哈密顿环对应 2ⁿ 种真值赋值。

---

**9.（Partitioning & Numerical Problems, Slide16）** 关于划分问题和数值问题的 NP-完全性归约，下列说法正确的是：

- (A) 3-SAT 到 3D-MATCHING 的归约中，每个变量 x<sub>i</sub> 的 gadget 包含 k 个核元素（core）和 2k 个尖元素（tip），其中 k = 子句数
- (B) SUBSET-SUM 问题的输入数字通常用一元编码（unary）表示
- (C) 在 3-SAT 到 SUBSET-SUM 的归约中，构造的十进制整数共 n + k 位（n 为变量数，k 为子句数），且加法过程中可能产生进位
- (D) 3-SAT 到 3D-MATCHING 的归约中，cleanup gadget 用于消耗未被变量 gadget 和 clause gadget 使用的元素

**答案：D**

**解析：**
- A 错误：每个变量 gadget 包含 **2k** 个核元素和 **2k** 个尖元素。
- B 错误：SUBSET-SUM 的输入数字通常用**二进制**编码。若用一元编码，动态规划可在多项式时间内求解。
- C 错误：正是在**无进位**（carry-free）的设计下归约才成立。每列和最多为 6（小于进制 10），不会产生进位。
- **D 正确**：cleanup gadget 消耗未被选择的尖元素，确保所有元素被恰好匹配一次。

---

## 二、问答题

---

### 1. Certificate、Certifier 与 NP-完备性证明（20'）

#### （1）Certificate 与 Certifier（Certificate & Certifier, Slide16 p11-p14）（5'）

根据讲义中的定义，回答以下问题：

- a) 什么是 certificate？什么是 certifier？请给出 NP 的形式化定义。
- b) 对于 HAM-CYCLE 问题（给定无向图 G，判断是否存在经过每个顶点恰好一次的简单环），请设计一个 certificate 和对应的 certifier，证明 HAM-CYCLE ∈ NP。
- c) 已知 SAT ∈ NP。若某个问题 Y 满足 SAT ≤<sub>P</sub> Y 且 Y ∈ NP，能否断定 Y 是 NP-完全的？为什么？

**答案**

**a) 定义：**

- **Certificate（证书）**：对于判定问题 X，若 s ∈ X，则存在一个字符串 t（称为 certificate），能够证明 s ∈ X。Certificate 的长度必须受输入规模的多项式限制，即 |t| ≤ p(|s|) 对某个多项式 p(·) 成立。
- **Certifier（验证器）**：算法 C(s, t) 是问题 X 的 certifier，如果对任意字符串 s，s ∈ X 当且仅当存在某个字符串 t 使得 C(s, t) = yes。Certifier 必须在多项式时间内运行。
- **NP 的形式化定义**：NP 是那些存在多项式时间 certifier 的判定问题的集合。

**b) HAM-CYCLE ∈ NP 的证明：**

Certificate：n 个顶点的一个排列 (v<sub>1</sub>, v<sub>2</sub>, ..., v<sub>n</sub>)。

Certifier：
```
CERTIFIER(G, π):
    检查 π 是否包含 V 中每个顶点恰好一次
    for i = 1 to n-1:
        if (π[i], π[i+1]) ∉ E:
            return false
    if (π[n], π[1]) ∉ E:
        return false
    return true
```

- 若 G 存在哈密顿环，则存在排列作为 certificate，certifier 接受。
- 若 certifier 接受某个排列，则该排列给出 G 中的一个哈密顿环。
- 时间复杂度 O(n)，certificate 长度 O(n)，满足多项式限制。

**c)** 可以断定 Y 是 NP-完全的。

理由：要证明 Y 是 NP-完全的，需要 Y ∈ NP（已知）且对所有 X ∈ NP，X ≤<sub>P</sub> Y。由于 SAT 是 NP-完全的（Cook-Levin），对所有 X ∈ NP 有 X ≤<sub>P</sub> SAT。又由 SAT ≤<sub>P</sub> Y，由传递性得 X ≤<sub>P</sub> Y。故 Y 满足 NP-完全的两个条件。

---

#### （2）证明 NP-完全性（Proving NP-Completeness, Slide16 p42）（5'）

给出证明一个问题 Y 是 NP-完全的标准化步骤。然后以 3-COLOR 为例，说明如何应用这些步骤——即，证明 3-COLOR 是 NP-完全的需要哪些关键步骤？

**答案**

**标准化步骤（三步法）：**

1. **证明 Y ∈ NP**：设计多项式时间 certifier，说明存在长度受多项式限制的 certificate 可验证 s ∈ Y。
2. **选择一个已知 NP-完全问题 X**（如 CIRCUIT-SAT、3-SAT）。
3. **证明 X ≤<sub>P</sub> Y**：构造多项式时间变换 f，使得 x ∈ X ⇔ f(x) ∈ Y。

**以 3-COLOR 为例：**

- **Step 1 - 3-COLOR ∈ NP**：Certificate 是每个顶点的颜色分配（3 色之一）。Certifier 检查每条边的两端颜色是否不同，O(|E|) 时间完成。
- **Step 2 - 选择 X**：选择 3-SAT（已知 NP-完全）。
- **Step 3 - 3-SAT ≤<sub>P</sub> 3-COLOR**（讲义 p75-p79）：
  - 为每个 literal 创建节点；
  - 创建 T、F、B **三个**特殊节点，三者连成三角形；
  - 每个 literal 节点连接到 B；
  - 每个 literal 与其 negation 连边；
  - 对每个 clause 添加 6 节点 13 边的专用 gadget；
  - 证明图是 3-可着色的当且仅当 Φ 可满足。

---

#### （3）自归约性（Self-Reducibility, Slide16 p36）（5'）

讲义中讨论了 decision problem 与 search problem 之间的自归约性（self-reducibility）。以 VERTEX-COVER 为例：

- a) 什么是问题 X 的自归约性？为什么自归约性在研究 NP-完全问题时很重要？
- b) 请描述如何利用 VERTEX-COVER 的判定版本（是否存在大小不超过 k 的顶点覆盖）来求解其搜索版本（找出最小顶点覆盖的具体顶点集合），写出算法伪代码并分析时间复杂度。

**答案**

**a) 定义与重要性：**

- **自归约性**：问题 X 的搜索版本（search problem）可以多项式时间归约到 X 的判定版本（decision problem）。即，若能回答"是否存在"的问题，就可利用它找出具体解。
- **重要性**：表明对 NP-完全问题只需关注判定版本——判定版本和搜索版本在多项式时间内等价。这为 NP-完全理论提供了坚实基础，使研究可聚焦于判定问题。

**b) 顶点覆盖的自归约算法：**

```
FIND-VC(G):
    // 二分搜索最小顶点覆盖的大小
    lo = 0, hi = |V|
    while lo < hi:
        mid = ⌊(lo + hi) / 2⌋
        if DECIDE-VC(G, mid):    // 判定版本
            hi = mid
        else:
            lo = mid + 1
    k* = lo                       // 最小顶点覆盖的大小

    // 用判定版本来构造具体解
    S = ∅
    G' = G
    for each vertex v ∈ V:
        if DECIDE-VC(G' - {v}, k* - 1):
            S = S ∪ {v}           // v 属于某个最小顶点覆盖
            G' = G' - {v}
            k* = k* - 1
    return S
```

**时间复杂度**：设判定版本 DECIDE-VC 的时间为 O(f(n))，则二分搜索调用 O(log n) 次，构造阶段调用 O(n) 次，总时间 O((n + log n)·f(n)) = O(n·f(n))。由于 f(n) 为多项式，总时间仍为多项式。

---

### 2. 归约设计与分析（15'）

#### （1）简单等价归约（Reduction by Simple Equivalence, Slide16 p24-p28）（5'）

证明 INDEPENDENT-SET 与 VERTEX-COVER 之间存在多项式时间归约关系（即 INDEPENDENT-SET ≡<sub>P</sub> VERTEX-COVER）。请给出完整的双向证明。

**答案**

**方向 1：INDEPENDENT-SET ≤<sub>P</sub> VERTEX-COVER**

给定 IS 的实例 G = (V, E)，整数 k，构造 VC 的实例：同一张图 G，整数 k' = n − k。

- **⇒**：设 S 是 G 中大小 ≥ k 的独立集。对任意边 (u, v) ∈ E，由独立集定义，u ∉ S 或 v ∉ S，即至少一个在 V − S 中。故 V − S 是顶点覆盖，且 |V − S| = n − |S| ≤ n − k = k'。
- **⇐**：设 T 是 G 中大小 ≤ k' 的顶点覆盖，令 S = V − T。若存在 u, v ∈ S 且 (u, v) ∈ E，则 T 不覆盖 (u, v)，矛盾。故 S 是独立集，|S| = n − |T| ≥ n − k' = k。

**方向 2：VERTEX-COVER ≤<sub>P</sub> INDEPENDENT-SET**

给定 VC 的实例 G = (V, E)，整数 k'，构造 IS 的实例：同一张图 G，整数 k = n − k'。证明与方向 1 完全对称。

因此 INDEPENDENT-SET ≡<sub>P</sub> VERTEX-COVER。归约仅需计算 n − k，O(1) 时间。

---

#### （2）Gadget 归约（Reduction via Gadgets, Slide16 p33-p34）（5'）

考虑 3-SAT 到 INDEPENDENT-SET 的归约。给定如下的 3-SAT 实例：

$$\Phi = (x_1 \lor \overline{x_2} \lor x_3) \land (\overline{x_1} \lor x_2 \lor x_4) \land (x_1 \lor x_2 \lor \overline{x_4})$$

- a) 按照讲义中的构造方法，画出对应的图 G 并标出 k 的值。
- b) 若取真值赋值 $x_1=1, x_2=0, x_3=1, x_4=1$，请指出该赋值对应的独立集。
- c) 简述该归约的正确性证明思路（⇒ 和 ⇐ 两个方向）。

**答案**

**a) 图 G 的构造：**

$$\Phi = (x_1 \lor \overline{x_2} \lor x_3) \land (\overline{x_1} \lor x_2 \lor x_4) \land (x_1 \lor x_2 \lor \overline{x_4})$$

共有 3 个子句，k = 3。每个子句构造一个三角形：

```
    Clause 1       Clause 2       Clause 3
      x₁             ¬x₁            x₁
     /  \           /  \           /  \
   ¬x₂——x₃        x₂——x₄        x₂——¬x₄
   
   否定边（跨三角形连接 literal 与其 negation）：
   x₁ ——— ¬x₁
   ¬x₂ ——— x₂
   x₄ ——— ¬x₄
   （x₃ 无否定出现）
```

**b) 取赋值 x₁=1, x₂=0, x₃=1, x₄=1：**

检查每个子句中为 true 的 literal：
- C1：(x₁ ∨ ¬x₂ ∨ x₃) — x₁ = 1, ¬x₂ = 1, x₃ = 1，均可选（选 x₁）
- C2：(¬x₁ ∨ x₂ ∨ x₄) — ¬x₁ = 0, x₂ = 0, x₄ = 1 ✓（选 x₄）
- C3：(x₁ ∨ x₂ ∨ ¬x₄) — x₁ = 1 ✓，x₂ = 0, ¬x₄ = 0（选 x₁）

独立集 S = {C1 中的 x₁, C2 中的 x₄, C3 中的 x₁}。C1 和 C3 的两个 x₁ 是不同顶点（属不同三角形），之间无否定关系，可同时存在。

**c) 正确性证明思路：**

**⇒（可满足 → 独立集）**：给定满足赋值，从每个子句选一个 true literal。这些 literal 构成独立集：(i) 三角形内最多选一个（因三角形内任意两点相邻）；(ii) 不会同时选中 literal 及其否定（若一个为 true，另一个必为 false）。选中顶点数 = 子句数 = k。

**⇐（独立集 → 可满足）**：设 S 是大小为 k 的独立集。每个三角形最多选一个顶点，共 k 个三角形，故 S 从每个三角形中恰好选一个。将这些 literal 设为 true，其余变量任意赋值。独立集中不含 literal 及其否定，故赋值一致。每个子句至少有一个 true literal，Φ 可满足。

---

#### （3）归约的传递性（Transitivity of Reductions, Slide16 p35）（5'）

已知以下归约关系成立（部分已在讲义中证明）：

- 3-SAT ≤<sub>P</sub> INDEPENDENT-SET
- INDEPENDENT-SET ≤<sub>P</sub> VERTEX-COVER
- VERTEX-COVER ≤<sub>P</sub> SET-COVER

- a) 利用归约的传递性（transitivity），可以推出什么结论？请写出完整的归约链。
- b) 假设某天有人发现了 SET-COVER 的多项式时间算法，请根据归约链分析这一发现对 3-SAT、INDEPENDENT-SET、VERTEX-COVER 等问题的影响。
- c) 请从讲义介绍的 NP-完全问题类型中再举出一个不同的归约链的例子（至少包含 3 个问题），并简要说明各步归约的基本思路。

**答案**

**a) 归约链：**

由传递性：若 X ≤<sub>P</sub> Y 且 Y ≤<sub>P</sub> Z，则 X ≤<sub>P</sub> Z。因此：

$$\text{3-SAT} \leq_P \text{INDEPENDENT-SET} \leq_P \text{VERTEX-COVER} \leq_P \text{SET-COVER}$$

故 3-SAT ≤<sub>P</sub> SET-COVER，即 SAT 问题可归约到集合覆盖问题。

**b) 若 SET-COVER 有多项式时间算法：**

- 由归约链，3-SAT、INDEPENDENT-SET、VERTEX-COVER 也都在多项式时间内可解。
- 由于 3-SAT 是 NP-完全的，所有 NP 问题都可归约到 3-SAT，进而归约到 SET-COVER。
- 这意味着**所有 NP 问题**都有多项式时间算法，即 **P = NP**。

**c) 另一归约链示例：**

**3-SAT ≤<sub>P</sub> 3-COLOR ≤<sub>P</sub> k-REGISTER-ALLOCATION**

1. **3-SAT ≤<sub>P</sub> 3-COLOR**（讲义 p75-p79）：
   - 为每个 literal 创建节点，创建 T、F、B 三个特殊节点（三角形）
   - 连接 literal 到 B，literal 与其否定间连边
   - 每个 clause 添加 6 节点 13 边的 gadget
   - 可满足 ⇔ 3-可着色

2. **3-COLOR ≤<sub>P</sub> k-REGISTER-ALLOCATION**（讲义 p74）：
   - 寄存器分配中，干涉图的 k-可着色性对应能否用 k 个寄存器
   - 将 3-COLOR 的图直接作为干涉图，k = 3
   - 由 Chaitin (1982) 结果，该归约对任意 k ≥ 3 成立

---

### 3. 重要归约的综合分析（15'）

#### （1）Sequencing Problem 归约链：DIR-HAM-CYCLE ≤<sub>P</sub> HAM-CYCLE（5'）

讲义中给出了 DIR-HAM-CYCLE ≤<sub>P</sub> HAM-CYCLE 的归约：

- a) 描述该归约的具体构造方法：给定有向图 G = (V, E)，如何构造无向图 G'？G' 的顶点数和边数各是多少？
- b) 证明：G 存在有向哈密顿环当且仅当 G' 存在无向哈密顿环。
- c) 若将每个顶点替换为 2 个节点而非 3 个，该归约是否仍然正确？为什么？

**答案**

**a) 构造方法：**

给定有向图 G = (V, E)，构造无向图 G' = (V', E')：

- 对每个 v ∈ V，在 G' 中创建 **3 个节点**：v<sub>in</sub>、v<sub>mid</sub>、v<sub>out</sub>
- 在 G' 中添加边：(v<sub>in</sub>, v<sub>mid</sub>)、(v<sub>mid</sub>, v<sub>out</sub>)
- 对每条有向边 (u, v) ∈ E，添加无向边：(u<sub>out</sub>, v<sub>in</sub>)

G' 的规模：顶点数 3|V|，边数 2|V| + |E|。

**b) 双向证明：**

**⇒**：设 G 有有向哈密顿环 (v₁, v₂, ..., vₙ, v₁)。在 G' 中构造：
```
v₁,in → v₁,mid → v₁,out → v₂,in → v₂,mid → v₂,out → ... → vₙ,in → vₙ,mid → vₙ,out → v₁,in
```
该环访问 G' 所有 3n 个顶点各一次，边均在 E' 中，故为无向哈密顿环。

**⇐**：设 G' 有无向哈密顿环 Γ'。Γ' 访问 3 色节点时只能按以下顺序：
- ..., B, G, R, B, G, R, B, G, R, B, ...
- ..., B, R, G, B, R, G, B, R, G, B, ...

其中 B = in, G = mid, R = out。无论哪种顺序，Γ' 中的蓝色节点（v<sub>in</sub>）恰好对应 G 中每个顶点一次，且顺序构成 G 的一个有向哈密顿环。

**c) 若替换为 2 个节点：**

归约将**不正确**。3 节点构造的三色模式强制了方向顺序（in → mid → out），保证了有向路径的一致性。2 节点无法施加这种方向约束，可能导致 G' 的无向哈密顿环在 G 中对应一条不按有向边方向行进的"回头路"。

---

#### （2）HAM-CYCLE 到 TSP 的归约（Sequencing: HAM-CYCLE ≤<sub>P</sub> TSP）（5'）

- a) 给定一个 HAM-CYCLE 实例 G = (V, E)，构造对应的 TSP 实例（城市集合和距离函数），要求归约在多项式时间内完成。
- b) 证明归约正确性：G 是哈密顿的当且仅当 TSP 实例存在不超过某个阈值的旅行。
- c) 该归约中构造的 TSP 距离函数是否满足三角不等式（△-inequality）？为什么这个性质值得注意？

**答案**

**a) 构造：**

给定 G = (V, E)，|V| = n，构造 TSP 实例：
- 城市集：C = V（n 个城市）
- 距离函数：
  $$d(u, v) = \begin{cases} 1 & \text{if } (u, v) \in E \\ 2 & \text{if } (u, v) \notin E \end{cases}$$
- 阈值：D = n

**b) 正确性证明：**

**⇒**：设 G 有哈密顿环 (v₁, v₂, ..., vₙ, v₁)。每对相邻顶点在 G 中都有边，故 d(vᵢ, vᵢ₊₁) = 1，总距离 = n ≤ D。

**⇐**：设 TSP 有总距离 ≤ n 的旅行。共 n 段行程，每段 ≥ 1，故每段必须恰好为 1。由定义，d(u, v) = 1 当且仅当 (u, v) ∈ E，因此旅行对应 G 的一个哈密顿环。

**c) 三角不等式：**

该构造满足三角不等式：d(u, w) ≤ d(u, v) + d(v, w) 对任意 u, v, w 成立。

- 若 d(u, w) = 1：右边 ≥ 1 + 1 = 2 ≥ 1 ✓
- 若 d(u, w) = 2：右边最小为 1 + 1 = 2 = d(u, w) ✓

值得注意：即使对 TSP 添加三角不等式限制（即度量 TSP），问题仍是 NP-完全的。这表明即便是具有良好几何性质的 TSP 实例，精确求解仍然困难。

---

#### （3）3-SAT 到 SUBSET-SUM 的归约（Numerical: 3-SAT ≤<sub>P</sub> SUBSET-SUM）（5'）

假设有 3-SAT 实例：

$$\Phi = (x_1 \lor x_2 \lor \overline{x_3}) \land (\overline{x_1} \lor \overline{x_2} \lor x_3)$$

- a) 按照讲义中的构造方法，列出所有数字（十进制，n + k 位）和目标值 W。
- b) 解释为什么构造时要确保每一列求和时不产生进位（carry-free）。
- c) 该归约能否推广到一般的 SAT 问题（不限制子句大小为 3）？请说明理由。

**答案**

**a) 具体的数字构造：**

n = 3 个变量，k = 2 个子句，每个数字是 n + k = 5 位十进制数。

| 数字 | 说明 | x₁ 列 | x₂ 列 | x₃ 列 | C₁ 列 | C₂ 列 |
|------|------|-------|-------|-------|-------|-------|
| v₁   | x₁ = 1 | 1 | 0 | 0 | 1 | 0 |
| v₁'  | x₁ = 0 | 1 | 0 | 0 | 0 | 1 |
| v₂   | x₂ = 1 | 0 | 1 | 0 | 1 | 0 |
| v₂'  | x₂ = 0 | 0 | 1 | 0 | 0 | 1 |
| v₃   | x₃ = 1 | 0 | 0 | 1 | 0 | 1 |
| v₃'  | x₃ = 0 | 0 | 0 | 1 | 1 | 0 |
| s₁   | C₁ slack 1 | 0 | 0 | 0 | 1 | 0 |
| s₁'  | C₁ slack 2 | 0 | 0 | 0 | 1 | 0 |
| s₂   | C₂ slack 1 | 0 | 0 | 0 | 0 | 1 |
| s₂'  | C₂ slack 2 | 0 | 0 | 0 | 0 | 1 |
| **W** | 目标值 | **1** | **1** | **1** | **3** | **3** |

变量行中，xᵢ 列放 1；若该变量的 true 赋值满足子句 Cⱼ，在对应子句列放 1。每个子句列有 2 个 slack（值为 1），补足与目标值 3 的差值。

**b) 无进位设计的原因：**

无进位保证可从子集和的目标值唯一反推每个变量列和子句列的选择：

- **变量列**：目标值 = 1，意味着恰好选择一个变量值（true 或 false）。若两个都选得 2 > 1，都不选得 0 < 1。
- **子句列**：目标值 = 3。最多 3 个 true literal + 2 个 slack 各贡献 1，总和 ≤ 5 < 10，不会进位。
- 无进位使得子集和问题的解与 3-SAT 的可满足赋值一一对应。

**c) 能否推广到一般 SAT：**

**可以**推广。对于长度为 m 的子句：
- 若 m = 3：保持原构造不变。
- 若 m < 3：减少 slack 数量。
- 若 m > 3：最多 m 个 true literal 可出现在该列，目标值需调整为 m（并将进制加大以确保无进位），或增加 slack 数量。

但实际中一般通过 Tseitin 变换先将 SAT 转化为 3-SAT，再做 3-SAT ≤<sub>P</sub> SUBSET-SUM，无需直接构造 SAT 到 SUBSET-SUM 的归约。
