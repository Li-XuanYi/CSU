# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：Turing Machine & Computability（Slide15）

---

## 一、选择题

**1.（Turing Machine Composition, Slide15）** 关于 Turing Machine 的组成部分，以下哪项描述是**错误**的？

- (A) Turing Machine 包含一条无限长的 tape，tape 被划分为若干 cell，每个 cell 可存储一个 symbol
- (B) Turing Machine 包含一个有限的状态集合，其中包含 start state 和 halt state
- (C) Turing Machine 的指令形式为 $\langle q_i, s_j \rangle \to \langle q_l, s_k, L/R/S \rangle$，含义为：处于状态 $q_i$ 读到符号 $s_j$，则转向状态 $q_l$，写入 $s_k$，并移动读头
- (D) Turing Machine 的 tape 只能向右移动，不能向左移动

**答案**：D **解析**：Turing Machine 的读头可以向左（L）、向右（R）或停在原地（S），因此选项 (D) 说"只能向右移动"是错误的。

---

**2.（Multi-Tape TM, Slide15）** 关于 Multi-Tape Turing Machine，以下说法正确的是：

- (A) k-tape TM 的计算能力弱于 Single-Tape TM
- (B) k-tape TM 的计算能力强于 Single-Tape TM
- (C) 若 $f$ 在 $T(n)$ 时间内可被 k-tape TM 计算，则在 $O(5kT(n)^2)$ 时间内可被 Single-Tape TM 计算
- (D) 若 $f$ 在 $T(n)$ 时间内可被 k-tape TM 计算，则在 $O(kT(n))$ 时间内可被 Single-Tape TM 计算

**答案**：C **解析**：根据讲义内容（Slide15 p.32），Multi-Tape TM 可以通过 tape interleaving 技术模拟为 Single-Tape TM，时间开销为 $5kT(n)^2$。虽然 Multi-Tape 更高效，但两者计算能力等价（选项 A、B 均错误）。选项 D 的时间上界不正确。

---

**3.（Alphabet Size, Slide15）** 关于 Turing Machine 字母表大小的比较，以下说法正确的是：

- (A) 使用字母表 $\Gamma$ 的 TM 比使用 $\{0,1,2,\sqcup\}$ 的 TM 计算能力更强
- (B) 若 $f$ 在 $T(n)$ 时间内可被使用字母表 $\Gamma$ 的 TM $M$ 计算，则在 $O(4\log|\Gamma| \cdot T(n))$ 时间内可被使用字母表 $\{0,1,2,\sqcup\}$ 的 TM $\tilde{M}$ 计算
- (C) 字母表大小不影响模拟的时间开销
- (D) 使用字母表 $\{0,1,2,\sqcup\}$ 的 TM 不能模拟使用更大字母表的 TM

**答案**：B **解析**：根据讲义内容（Slide15 p.28-30），使用字母表 $\Gamma$ 的 TM 可以被使用 $\{0,1,2,\sqcup\}$ 的 TM 以 $O(\log|\Gamma|)$ 的因子模拟：每个符号编码为 $\log|\Gamma|$ 个比特，每次模拟需要 $4\log|\Gamma|$ 步。两者的计算能力是等价的（选项 A、D 错误），但模拟时有额外的时间开销（选项 C 错误）。

---

**4.（Bidirectional Tape, Slide15）** 关于双向 tape（Bidirectional Tape）和单向 tape（Unidirectional Tape）的关系，以下说法正确的是：

- (A) 双向 tape 的 TM 计算能力强于单向 tape 的 TM
- (B) 若 $f$ 在 $T(n)$ 时间内可被双向 tape 的 TM $M$ 计算，则在 $4T(n)$ 时间内可被单向 tape 的 TM $\tilde{M}$ 计算
- (C) 双向 tape 的 TM 无法被单向 tape 的 TM 模拟
- (D) 单向 tape 的 TM 无法访问负半轴的内容

**答案**：B **解析**：根据讲义内容（Slide15 p.35-37），双向 tape 的 TM 可以被单向 tape 的 TM 以 $4T(n)$ 的时间因子模拟。模拟思路是将双向 tape 映射到单向 tape 上，使用两个 symbol 叠加（$\Gamma \times \Gamma$）的字母表。两者计算能力等价。

---

**5.（Effective Procedure, Slide15）** 关于有效过程（Effective Procedure），以下说法正确的是：

- (A) 任何良定义的数学函数都是可计算的
- (B) 定理证明（Theorem Proving）是有效的/可计算的
- (C) 证明验证（Proof Verification）是有效的/可计算的
- (D) Church-Turing Thesis 是一个严格的数学定理

**答案**：C **解析**：根据讲义内容（Slide15 p.6），定理证明在一般情况下不是有效的（算法不可解），但证明验证是有效的（可以机械验证一个证明的正确性）。Church-Turing Thesis 是一个论题（thesis）而非定理，它断言各种计算模型等价的观察结论。并非所有函数都是可计算的（例如 $g(n)$ 判断 $\pi$ 的十进制展开中是否有连续 $n$ 个 7，该函数不可计算）。

---

**6.（Decidable vs Computable, Slide15）** 关于判定性问题（Decision Problem）和可判定性（Decidability），以下说法正确的是：

- (A) 谓词 $P(x)$ 是可判定的当且仅当其特征函数 $c_P(x)$ 是可计算的
- (B) 所有关于自然数的谓词都是可判定的
- (C) 判定性问题与函数计算问题完全无关
- (D) 如果一个函数是可计算的，那么它对应的判定性问题一定是可判定的

**答案**：A **解析**：根据讲义内容（Slide15 p.41-42），谓词 $P(x)$ 是可判定的，当且仅当其特征函数 $c_P(x)$（输出 1 表示真，0 表示假）是可计算的。并非所有关于自然数的谓词都是可判定的（如停机问题）。判定性和函数计算是紧密相关的。

---

**7.（Coding of Domain, Slide15）** 设 $\alpha: \mathbb{Z} \to \mathbb{N}$ 为整数域 $\mathbb{Z}$ 的编码：
$$\alpha(n) = \begin{cases} 2n, & n \ge 0 \\ -2n-1, & n < 0 \end{cases}$$
考虑函数 $f(x) = x - 1$ 在 $\mathbb{Z}$ 上，则其对应的数值函数 $f^*: \mathbb{N} \to \mathbb{N}$ 在 $x = 3$ 处的值为：

- (A) $f^*(3) = 5$
- (B) $f^*(3) = 2$
- (C) $f^*(3) = 3$
- (D) $f^*(3) = 4$

**答案**：A **解析**：$f^* = \alpha \circ f \circ \alpha^{-1}$

---

## 二、问答题

### 1. Turing Machine 设计与模拟（20 分）

#### （1）Multi-Tape TM 概念（Effective Procedure, Slide15）（5'）

**问题**：请说明什么是有效过程（Effective Procedure / Algorithm）。举一个可计算（computable）函数和不可计算（non-computable）函数的例子，并简要解释原因。

**答案**：

**有效过程（Algorithm）**：一个由有限条指令组成的过程，对某个输入集合中的任意输入，通过系统执行这些指令，能在有限步内终止并产生输出。

**可计算函数示例**：$HCF(x, y)$（最大公约数）—— Euclid 算法可在有限步内终止。

**不可计算函数示例**：$g(n) = \begin{cases} 1, & \text{若 } \pi \text{ 的十进制展开中存在连续 } n \text{ 个 7} \\ 0, & \text{否则} \end{cases}$——我们无法设计一个总能终止的算法来判断这一性质。

---

#### （2）Single-Tape TM 设计（Turing Machine, Slide15）（8'）

**问题**：设计一个单带 Turing Machine，其功能为：给定一个由 0 和 1 组成的二进制字符串，判断该字符串是否为回文（palindrome，即正读反读相同）。输出要求在 tape 上写入 1 后停机（是回文）或写入 0 后停机（不是回文）。

请给出：
1. 状态集合 $Q$ 的设计
2. 指令集（至少 6 条核心指令）
3. 用 $(q, tape\_content)$ 序列简要展示对于输入 $\triangleright 0110 \triangleleft$ 的执行过程

**答案**：

**状态集合**：
- $q_S$：起始状态
- $q_0$：读到 0，标记并向右移动，准备检查末尾
- $q_1$：读到 1，标记并向右移动，准备检查末尾
- $q_R$：向右移动到 tape 末尾
- $q_{L0}$：从右端向左回退（期望读到 0）
- $q_{L1}$：从右端向左回退（期望读到 1）
- $q_{back}$：回退到左侧已标记位置之后
- $q_{accept}$：接受状态（输出 1）
- $q_{reject}$：拒绝状态（输出 0）
- $q_H$：停机状态

其中 $q_{accept}$ 和 $q_{reject}$ 都转移到 $q_H$。

**指令集**（核心部分）：

- 起始和工作阶段：
1. $\langle q_S, \triangleright \rangle \to \langle q_S, \triangleright, R\rangle$
2. $\langle q_S, 0 \rangle \to \langle q_0, \#, R\rangle$（标记 0 为 $\#$）
3. $\langle q_S, 1 \rangle \to \langle q_1, \#, R\rangle$（标记 1 为 $\#$）
4. $\langle q_S, \triangleleft \rangle \to \langle q_{accept}, \triangleleft, S\rangle$（空串或只剩单个字符）

- 右移到末尾：
5. $\langle q_0, 0/1 \rangle \to \langle q_0, 0/1, R\rangle$
6. $\langle q_0, \triangleleft \rangle \to \langle q_{L0}, \triangleleft, L\rangle$（到末尾，回退检查是否匹配 0）
7. $\langle q_1, 0/1 \rangle \to \langle q_1, 0/1, R\rangle$
8. $\langle q_1, \triangleleft \rangle \to \langle q_{L1}, \triangleleft, L\rangle$（到末尾，回退检查是否匹配 1）

- 检查匹配：
9. $\langle q_{L0}, 0 \rangle \to \langle q_{back}, \#, L\rangle$（匹配上了，标记后回退）
10. $\langle q_{L0}, 1/\# \rangle \to \langle q_{L0}, 1/\#, L\rangle$（继续向左）
11. $\langle q_{L0}, \triangleright \rangle \to \langle q_{reject}, \triangleright, S\rangle$（没找到匹配的 0 → 拒绝）
12. $\langle q_{L1}, 1 \rangle \to \langle q_{back}, \#, L\rangle$
13. $\langle q_{L1}, 0/\# \rangle \to \langle q_{L1}, 0/\#, L\rangle$
14. $\langle q_{L1}, \triangleright \rangle \to \langle q_{reject}, \triangleright, S\rangle$

- 回退到左端：
15. $\langle q_{back}, 0/1/\# \rangle \to \langle q_{back}, 0/1/\#, L\rangle$
16. $\langle q_{back}, \triangleright \rangle \to \langle q_S, \triangleright, R\rangle$

- 接受/拒绝：
17. $\langle q_{accept}, \triangleright/0/1/\#/\triangleleft \rangle \to \langle q_H, 1, S\rangle$（输出 1 后停机）
18. $\langle q_{reject}, \triangleright/0/1/\#/\triangleleft \rangle \to \langle q_H, 0, S\rangle$（输出 0 后停机）

**执行过程示例**（输入 $\triangleright 0110 \triangleleft$，是回文）：

```
(q_S, ▷0110◁)
→ (q_S, ▷0110◁)  [起始，跳过▷]
→ (q_0, ▷#110◁)   [读0，标记为#]
→ (q_0, ▷#110◁)   [向右移动]
→ (q_0, ▷#110◁)   [向右移动]
→ (q_0, ▷#110◁)   [向右移动]
→ (q_{L0}, ▷#110◁) [到达◁，向左回退检查0]
→ (q_{L0}, ▷#11#◁) [读到0，匹配！标记为#]
→ (q_{back}, ▷#11#◁) [向左回退到左端]
→ ... (q_{back}, ▷#11#◁)
→ (q_S, ▷#11#◁)   [回到起始，重新开始]
→ (q_S, ▷#11#◁)   [跳过▷，遇到#继续右移]
→ (q_S, ▷#11#◁)   [跳过#]
→ (q_1, ▷##1#◁)    [读1，标记为#]
→ (q_1, ▷##1#◁)    [向右移动，跳过#]
→ (q_{L1}, ▷##1#◁) [到达◁]
→ (q_{L1}, ▷### #◁) [读到1，匹配！标记为#]
→ (q_{back}, ▷### #◁)
→ ...
→ (q_S, ▷####◁)   [所有字符都匹配]
→ (q_S, ▷####◁)   [跳过#]
→ (q_S, ▷####◁)   [再次跳过#]
→ ...              [到达◁]
→ (q_{accept}, ▷####◁) → (q_H, ▷####1◁) [输出1，停机]
```

---

#### （3）TM-Computable Function（Decidable, Slide15）（7'）

**问题**：令 $f: \mathbb{N} \to \mathbb{N}$ 是一个函数。定义 TM 计算 $f$ 的含义。接着说，对于一个谓词（predicate）$P(x_1, \dots, x_n)$，如何判断它是可判定的（decidable）？

进一步，证明：如果 $P(x)$ 和 $Q(x)$ 都是可判定的谓词，则 $P(x) \land Q(x)$ 也是可判定的。

**答案**：

**TM 计算函数的含义**：
设 $M$ 是一个 Turing Machine，$a_1, \dots, a_n, b \in \mathbb{N}$。当 $M(a_1, \dots, a_n)$ 收敛到 $b$（记作 $M(a_1, \dots, a_n) \downarrow b$）时，表示在初始配置 $a_1, \dots, a_n, 0, 0, \dots$ 上运行后，最终配置中第一个寄存器的值为 $b$。若对于所有 $a_1, \dots, a_n, b \in \mathbb{N}$，有 $M(a_1, \dots, a_n) \downarrow b \iff f(a_1, \dots, a_n) = b$，则称 $M$ **TM-计算** $f$。函数 $f$ 是 **TM-可计算的**（简称可计算的），如果存在一个 Turing Machine 能计算它。

**可判定性**：
谓词 $P(x_1, \dots, x_n)$ 的特征函数 $c_P$ 定义为：
$$c_P(x) = \begin{cases} 1, & P(x) \text{ 成立} \\ 0, & \text{否则} \end{cases}$$
$P$ 是 **可判定的** 当且仅当 $c_P$ 是可计算的；否则 $P$ 是不可判定的。

**证明 $P(x) \land Q(x)$ 可判定**：

设 $P$ 和 $Q$ 的可判定性分别由 TM $M_P$ 和 $M_Q$ 保证。构造 TM $M_{P\land Q}$ 如下：
1. 对输入 $x$，模拟 $M_P(x)$，得到输出 $r_1 \in \{0, 1\}$
2. 若 $r_1 = 0$，输出 0 并停机
3. 若 $r_1 = 1$，模拟 $M_Q(x)$，得到输出 $r_2 \in \{0, 1\}$
4. 输出 $r_2$ 并停机

由于 $M_P$ 和 $M_Q$ 都在有限步内停机，$M_{P\land Q}$ 也在有限步内停机，且输出为 1 当且仅当 $P(x)$ 和 $Q(x)$ 同时为真。因此 $P(x) \land Q(x)$ 是可判定的。

类似地，可以证明 $P(x) \lor Q(x)$、$\lnot P(x)$ 等逻辑组合也是可判定的。

---

### 2. TM 变体与计算理论（15 分）

#### （1）Church-Turing Thesis（TM Variations, Slide15）（5'）

**问题**：简述 Church-Turing 论题（Church-Turing Thesis）的内容。列出至少三种历史上提出的计算模型，它们如何支持这一论题？

**答案**：

**Church-Turing 论题**：任何直观意义上的"有效可计算的"函数都可以被 Turing Machine 计算。或者等价地说，所有合理的计算模型都刻画了同一类可计算函数。

**支持该论题的计算模型**（均为 1936 年前后提出）：
1. **Gödel-Kleene（1936）**：部分递归函数（Partial Recursive Functions）
2. **Turing（1936）**：Turing Machine
3. **Church（1936）**：$\lambda$-演算（$\lambda$-terms）
4. **Post（1943）**：Post 系统（Post Systems）
5. **Markov（1951）**：Markov 算法（Post 系统的变体）
6. **Shepherdson-Sturgis（1963）**：URM-可计算函数（Unlimited Register Machine）

这些模型虽然在形式上完全不同，但它们定义的**可计算函数类完全相同**，这为 Church-Turing 论题提供了强有力的支持。

---

#### （2）Single-Tape 模拟 Multi-Tape（Single-Tape vs Multi-Tape, Slide15）（5'）

**问题**：解释如何用 Single-Tape TM 模拟 k-Tape TM 的计算过程。说明核心思想、关键步骤和时间开销。

**答案**：

**核心思想**：将 k 条 tape 交错（interleave）存储到一条 tape 上。

**具体方法**：
1. **保留输入**：tape 的前 $n+1$ 个 cell 保留给输入
2. **符号扩展**：每个 symbol $a$ 变为两个 symbol $(a, \hat{a})$，其中 $\hat{a}$ 标记当前读头位置。每个 tape 占用一个"轨道"
3. **初始化**：在输入后放置 $\sqcup$ 标记，将输入比特复制到"虚拟输入 tape"上（复制时覆盖原符号为 $\sqcup$），标记第 $n+2$ 到第 $n+k$ 个 cell 为初始读头位置

**模拟步骤**：
1. 从第 $(n+1)$ 个 cell 向右扫描 $kT(n)$ 个 cell，在寄存器中记录 k 个带 hat 标记的符号（表示 k 个读头的位置和内容）
2. 利用 $M$ 的转移函数计算出新符号和移动方向
3. 从右向左扫描 $kT(n)$ 个 cell 更新 tape 内容，遇到带 hat 的符号时向右移动 k 个 cell 执行更新

**时间开销**：$O(5kT(n)^2)$，其中 $T(n)$ 是 k-tape TM 的运行时间。

---

#### （3）字母表大小对效率的影响（Larger Alphabets, Slide15）（5'）

**问题**：说明用字母表 $\{0, 1, 2, \sqcup\}$ 的 TM 模拟使用任意字母表 $\Gamma$ 的 TM 的方法和开销。

具体地，若一个使用字母表 $\Gamma$ 的 k-tape TM $M$ 在 $T(n)$ 时间内计算函数 $f$，请设计模拟 TM $\tilde{M}$ 每步模拟的五个阶段，并分析总时间复杂度。

**答案**：

**编码方法**：将 $\Gamma$ 中的每个符号编码为 $\{0,1\}$ 上的长度为 $\log|\Gamma|$ 的二进制串。

**状态扩展**：原状态 $q$ 扩展为多组状态：
- $\langle q \rangle$（初始状态）
- $\langle q, \sigma_1, \dots, \sigma_k \rangle$ 其中 $|\sigma_1| = \dots = |\sigma_k| = 1$（读取阶段）
- $\langle q, \sigma_1, \dots, \sigma_k \rangle$ 其中 $|\sigma_1| = \dots = |\sigma_k| = \log|\Gamma|$（已完整读取）

**模拟一步的五个阶段**：
1. **读取**：从每条 tape 上读取 $\log|\Gamma|$ 个比特（编码一个 $\Gamma$ 符号），用时 $O(\log|\Gamma|)$
2. **存储**：用状态寄存器存储读取的 k 个符号
3. **计算**：利用 $M$ 的转移函数计算 $M$ 写入的符号和新的状态
4. **暂存**：在状态寄存器中存储要写入的信息
5. **写入**：花费 $\log|\Gamma|$ 步将编码写入 tape

**总时间复杂度**：$O(4\log|\Gamma| \cdot T(n))$。每步模拟需 $O(\log|\Gamma|)$ 步读取和 $O(\log|\Gamma|)$ 步写入，加上状态转移的常数开销，总共 $4\log|\Gamma|$ 倍因子。

---
