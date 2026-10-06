# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：摊还分析（Amortized Analysis）
> 难度来源：Slide10

---

## 一、选择题（3.5' x 3 = 10.5'）

---

**1.（Multistack Push Amortized Analysis, Slide10）** 考虑一个多栈（Multistack）结构，由 $k+1$ 个栈 $S_0, S_1, \dots, S_k$ 组成。其中 $S_i$ 的容量为 $2^i$。执行 `PUSH(x)` 操作时，总是优先将元素压入 $S_0$；若 $S_0$ 已满，则依次将 $S_0$ 中所有元素弹出并压入 $S_1$；若 $S_1$ 也已满，则继续向上迁移，直至找到第一个有空位的栈。将上述多栈结构的 push 操作在不同情况下的时间复杂度进行配对，下列选项中**正确**的是：

- (A) 最好情况 $O(1)$，最坏情况 $O(n)$，平均/摊还情况 $O(\log n)$
- (B) 最好情况 $O(1)$，最坏情况 $O(n)$，平均/摊还情况 $O(1)$
- (C) 最好情况 $O(1)$，最坏情况 $O(n^2)$，平均/摊还情况 $O(n)$
- (D) 最好情况 $O(\log n)$，最坏情况 $O(n)$，平均/摊还情况 $O(1)$

**答案**：A **解析**：最好情况当 $S_0$ 未满时直接入栈，耗时 $O(1)$。最坏情况触发连续满栈迁移，需要迁移所有 $n$ 个元素，耗时 $O(n)$。摊还分析：$S_i$ 容量为 $2^i$，每 $2^i$ 次 push 触发一次迁移（代价 $2^i$），则 $m$ 次 push 中第 $i$ 级栈的总迁移代价为 $\frac{m}{2^i} \times 2^i = m$，共 $\log_2 m$ 级，总代价 $m\log_2 m$，摊还 $O(\log n)$。

---

**2.（Binary Counter Amortized Cost, Slide10）** 考虑一个 $k$ 位二进制计数器，初始为全 0。对该计数器执行 $n$ 次 `INCREMENT` 操作。关于使用摊还分析对该操作的分析，下列表述中**错误**的是：

- (A) 使用聚集分析（Aggregate Analysis）时，观察到位 $A[i]$ 每 $2^i$ 次 `INCREMENT` 翻转一次，$n$ 次操作的总翻转次数不超过 $2n$，因此摊还代价为 $O(1)$
- (B) 使用记账法（Accounting Method）时，可将每次 $0 \to 1$ 翻转的摊还代价设为 $2$（$1$ 用于本次翻转，$1$ 作为信用存储），$1 \to 0$ 翻转的摊还代价设为 $0$，信用总量等于计数器中 1 的个数
- (C) 使用势能法（Potential Method）时，可定义势能函数 $\Phi = \text{计数器中 0 的个数}$，则每次 `INCREMENT` 的摊还代价为 $2$
- (D) 使用势能法（Potential Method）时，可定义势能函数 $\Phi = \text{计数器中 1 的个数}$，则每次 `INCREMENT` 的摊还代价为 $2$

**答案**：C **解析**：

本题要求选出**错误**表述。逐一分析每个选项对 INCREMENT 的摊还分析：

**选项 (A) 正确——聚集分析**。

$k$ 位二进制计数器从 0 开始，执行 $n$ 次 INCREMENT。位 $A[i]$ 每 $2^i$ 次 INCREMENT 翻转一次（$A[0]$ 每次翻转，$A[1]$ 每 2 次翻转，$A[2]$ 每 4 次翻转……），因此 $n$ 次操作中 $A[i]$ 翻转 $\lfloor n/2^i \rfloor$ 次。总翻转次数：
$$T(n) = \sum_{i=0}^{k-1} \left\lfloor \frac{n}{2^i} \right\rfloor < n \sum_{i=0}^{\infty} \frac{1}{2^i} = 2n$$
摊还代价 $= T(n)/n < 2 = O(1)$。

**选项 (B) 正确——记账法**。

设置摊还代价：flip($0 \to 1$) 收 $2$，flip($1 \to 0$) 收 $0$。

每次 INCREMENT 中，恰好将一个 0 翻转为 1（从最低位开始连续翻转 1→0，然后将第一个 0 翻转为 1）。因此收取的摊还代价为 $2 \times 1 = 2$（翻转 $0 \to 1$ 的一次）+ $0 \times (\text{翻转 } 1 \to 0 \text{ 的次数})$ = 总共 $2$。

信用分配：每次 $0 \to 1$ 翻转时，$1$ 用于支付本次翻转本身，另外 $1$ 作为信用存入该位。当该位将来被翻转回 0 时，这 $1$ 信用正好支付 $1 \to 0$ 的翻转代价。因此信用总量 = 计数器中 1 的个数 $\ge 0$，信用永远不会为负。

总摊还代价 $= 2 \times (\#\text{flip}(0 \to 1))$。每次 INCREMENT 恰好产生一次 $0 \to 1$ 翻转，故 $\#\text{flip}(0 \to 1) = n$，总摊还代价 $\le 2n$。

**选项 (C) 错误——本题答案**。

势能法定义 $\hat{C}_i = C_i + \Phi(S_i) - \Phi(S_{i-1})$。若取 $\Phi = \text{计数器中 0 的个数}$，分析一次 INCREMENT 操作：

设第 $i$ 次 INCREMENT 翻转了 $t_i$ 个 $1 \to 0$ 和 $1$ 个 $0 \to 1$：
- 实际代价：$C_i = t_i + 1$
- 势能变化：$0 \to 1$ 翻转使 0 的个数**减少 1**，每个 $1 \to 0$ 翻转使 0 的个数**增加 1**，故 $\Phi(S_i) - \Phi(S_{i-1}) = 1 \cdot t_i - 1 \cdot 1 = t_i - 1$
- 摊还代价：$\hat{C}_i = (t_i + 1) + (t_i - 1) = 2t_i$

$t_i$ 可以是任意非负整数（例如从 $0111$ 到 $1000$ 时 $t_i = 3$），因此 $\hat{C}_i = 2t_i$ 不是常数 2，且当 $t_i$ 很大时摊还代价可以很大，无法得到 $O(1)$ 的结论。故 $\Phi = \text{0 的个数}$ 不是正确的势能函数。

**选项 (D) 正确——势能法**。

定义 $\Phi = \text{计数器中 1 的个数}$，分析同一次 INCREMENT：

设翻转了 $t_i$ 个 $1 \to 0$ 和 $1$ 个 $0 \to 1$：
- 实际代价：$C_i = t_i + 1$
- 势能变化：$0 \to 1$ 使 1 的个数**增加 1**，每个 $1 \to 0$ 使 1 的个数**减少 1**，故 $\Phi(S_i) - \Phi(S_{i-1}) = 1 - t_i$
- 摊还代价：$\hat{C}_i = (t_i + 1) + (1 - t_i) = 2$

每次 INCREMENT 的摊还代价恒为 2，且 $\Phi \ge 0 = \Phi_0$（初始全 0，1 的个数为 0），满足势能法条件。总摊还代价 $\le 2n$，摊还 $O(1)$。

**总结**：三种方法对 Binary Counter 的分析结论一致——INCREMENT 的摊还代价为 $O(1)$。$\Phi = \#1$ 是正确的势能函数，$\Phi = \#0$ 是错误的。

---

**3.（Accounting vs. Potential Method Concept, Slide10）** 关于摊还分析的三种方法——聚集分析（Aggregate Analysis）、记账法（Accounting Method）和势能法（Potential Method），下列表述中**正确**的是：

- (A) 聚集分析为每种操作分配不同的摊还代价，而记账法为所有操作分配相同的摊还代价
- (B) 在记账法中，不同操作的摊还代价可以不同，且要求在任何时刻信用总量不能为负（即 $\sum C_i \le \sum \hat{C}_i$）
- (C) 势能法要求势能函数 $\Phi$ 必须满足 $\Phi(S_n) = \Phi(S_0)$ 才能保证 $\sum \hat{C}_i \ge \sum C_i$
- (D) 三种方法中，只有聚集分析不涉及概率，而记账法和势能法都需要概率分析

**答案**：B **解析**：

本题要求选出**正确**说法。逐一分析每种方法的核心思想和适用场景：

**选项 (A) 错误——混淆了聚集分析和记账法的角色**。

聚集分析（Aggregate Analysis）对所有操作类型取**统一**的摊还代价：先算出 $n$ 次操作的总代价 $T(n)$，然后每个操作的摊还代价都设为 $T(n)/n$，不区分操作类型。例如对 Stack with MULTIPOP，三种操作（PUSH、POP、MULTIPOP）的摊还代价都是 $T(n)/n = O(1)$。

记账法（Accounting Method）恰恰相反，它允许为**不同**操作类型分配**不同**的摊还代价。例如对同一问题，记账法设 PUSH 摊还代价为 $2$，POP 为 $0$，MULTIPOP 为 $0$——三者各不相同。A 选项把两者的特征说反了。

**选项 (B) 正确——记账法的核心约束**。

记账法的要点：
1. **不同操作可分配不同摊还代价**：如 Binary Counter 中 flip($0\to1$) 收 $2$，flip($1\to0$) 收 $0$。
2. **信用总量不能为负**：设实际代价 $C_i$，摊还代价 $\hat{C}_i$，则每次操作后累积信用 $= \sum_{j=1}^{i} (\hat{C}_j - C_j) \ge 0$ 必须始终成立。这等价于对任意前缀 $k$ 有 $\sum_{j=1}^{k} C_j \le \sum_{j=1}^{k} \hat{C}_j$，从而保证 $\sum_{j=1}^{n} C_j \le \sum_{j=1}^{n} \hat{C}_j$。
3. **如果信用变为负**，意味着之前收取的摊还代价不足以支付后续的实际代价，摊还分析失败。

**选项 (C) 错误——势能法的条件过强**。

势能法中，摊还代价定义为 $\hat{C}_i = C_i + \Phi(S_i) - \Phi(S_{i-1})$。求和得：
$$\sum_{i=1}^{n} \hat{C}_i = \sum_{i=1}^{n} C_i + \Phi(S_n) - \Phi(S_0)$$
要保证 $\sum \hat{C}_i \ge \sum C_i$（即摊还代价是实际代价的上界），只需 $\Phi(S_n) \ge \Phi(S_0)$，即**终态势能不低于初态势能**即可，不需要 $\Phi(S_n) = \Phi(S_0)$。

$\Phi(S_n) > \Phi(S_0)$ 意味着势能有所增长，此时 $\sum \hat{C}_i$ 比 $\sum C_i$ 多出一个正项 $\Phi(S_n) - \Phi(S_0)$，上界仍然成立。例如 Binary Counter 的 $\Phi = \#1$，初态全 0 时 $\Phi_0 = 0$，终态有若干个 1 时 $\Phi_n > 0$，$\Phi(S_n) \ge \Phi(S_0)$ 显然成立。

**选项 (D) 错误——混淆了摊还分析与平均情况分析**。

摊还分析的三种方法（聚集、记账、势能）均**不涉及概率**。它们分析的是最坏情况下**操作序列**的平均代价，而不是"输入的概率分布"。区别如下：

| 方面 | 平均情况分析 (Average-case) | 摊还分析 (Amortized) |
|------|---------------------------|---------------------|
| 平均对象 | 对所有**输入**取平均 | 对**操作序列**取平均 |
| 是否涉及概率 | 是（假设输入服从某分布） | **否**（保证最坏情况下的平均性能） |
| 保证 | 概率意义下的"平均" | 确定性保证 |
| 例子 | 快速排序平均 $O(n\log n)$ | 动态表插入摊还 $O(1)$ |

三种摊还方法都给出确定性上界：即使操作序列是最坏情况的，摊还代价也是真实总代价的上界。

**总结**：摊还分析的核心思想是将昂贵操作的代价"分摊"到廉价操作上，三种方法的区别在于分摊方式——聚集法统一分摊、记账法按操作类型差异分摊、势能法通过状态函数间接分摊。

---

## 二、问答题（15'）

### 1. 动态表（Dynamic Table）的摊还分析（15'）

考虑一个动态表（Dynamic Table），支持 `TABLE_INSERT` 和 `TABLE_DELETE` 两种操作。表采用如下内存管理策略：

- **扩容策略**：当表满时（`num = size`），将表容量加倍为新表，复制所有元素。
- **缩容策略**：当删除导致负载因子 $\alpha = \text{num} / \text{size} < 1/4$ 时，将表容量减半为新表，复制所有元素。

#### （1）记账法分析 TABLE_INSERT（6'）

对于仅支持 `TABLE_INSERT`（无删除操作）的动态表，使用记账法为每次插入分配摊还代价 $\hat{C} = 3$（单位：$1$ 为一次基本操作的成本）。请解释：

- 这 $3$ 个单位的代价分别用于什么目的？
- 结合一个具体的扩容过程（例如表满时从容量 $4$ 扩容到 $8$），说明信用（credit）如何积累和消耗，并论证信用永远不会为负。

**答案**：

每次 `TABLE_INSERT` 收取 $\$3$：
- $\$1$ 用于支付本次插入本身的代价（将新元素写入表）。
- $\$2$ 作为信用存储，用于支付未来扩容时的复制代价。具体地，表中每个元素需要 $\$1$ 来支付其被复制到新表的代价。

扩容时，表中有 4 个旧元素需要复制到新表，复制代价为 $4$。信用余额中正好有此前为每个旧元素存储的 $\$1 \times 4 = \$4$ 来支付复制代价，再加上本次插入的 $\$2$ 中的一部分。信用始终为正，故 $\sum C_i \le \sum \hat{C}_i$，摊还代价 $O(1)$。

---

#### （2）势能法分析 TABLE_INSERT（5'）

对于同样仅支持 `TABLE_INSERT` 的动态表，定义势能函数 $\Phi(T) = 2 \times \text{num}[T] - \text{size}[T]$。请分别计算以下两种情况下 `TABLE_INSERT` 的摊还代价 $\hat{C}_i$：

- **情况 A**：第 $i$ 次插入未触发扩容
- **情况 B**：第 $i$ 次插入触发了扩容

要求写出完整的推导过程，并说明为什么该势能函数满足 $\Phi(S_n) \ge \Phi(S_0)$。

**答案**：

记 $\text{num}_i$ 和 $\text{size}_i$ 分别为第 $i$ 次操作后表中元素数和表容量。势能函数 $\Phi_i = 2 \cdot \text{num}_i - \text{size}_i$。

**情况 A：未触发扩容**

此时 $\text{size}_i = \text{size}_{i-1}$，$\text{num}_i = \text{num}_{i-1} + 1$。实际代价 $C_i = 1$（仅插入）。

$$
\begin{aligned}
\hat{C}_i &= C_i + \Phi_i - \Phi_{i-1} \\
&= 1 + (2\text{num}_i - \text{size}_i) - (2\text{num}_{i-1} - \text{size}_{i-1}) \\
&= 1 + (2(\text{num}_{i-1}+1) - \text{size}_{i-1}) - (2\text{num}_{i-1} - \text{size}_{i-1}) \\
&= 1 + 2 = 3
\end{aligned}
$$

**情况 B：触发扩容**

此时表满：$\text{num}_{i-1} = \text{size}_{i-1}$，且扩容后 $\text{size}_i = 2 \times \text{size}_{i-1}$，$\text{num}_i = \text{num}_{i-1} + 1$。实际代价 $C_i = 1 + \text{num}_{i-1}$（1 次插入 + 复制 $\text{num}_{i-1}$ 个旧元素）。

$$
\begin{aligned}
\hat{C}_i &= C_i + \Phi_i - \Phi_{i-1} \\
&= (1 + \text{num}_{i-1}) + (2\text{num}_i - \text{size}_i) - (2\text{num}_{i-1} - \text{size}_{i-1}) \\
&= (1 + \text{num}_{i-1}) + (2(\text{num}_{i-1}+1) - 2\text{size}_{i-1}) - (2\text{num}_{i-1} - \text{size}_{i-1}) \\
&= 1 + \text{num}_{i-1} + 2\text{num}_{i-1} + 2 - 2\text{size}_{i-1} - 2\text{num}_{i-1} + \text{size}_{i-1} \\
&= 3 + \text{num}_{i-1} - \text{size}_{i-1} \\
&= 3 + 0 = 3 \quad (\text{因为 } \text{num}_{i-1} = \text{size}_{i-1})
\end{aligned}
$$

**验证 $\Phi(S_n) \ge \Phi(S_0)$**：

初始空表：$\text{num}_0 = 0, \text{size}_0 = 0$，故 $\Phi_0 = 2 \times 0 - 0 = 0$。

动态表策略保证表至少半满（$\text{num} \ge \text{size}/2$），因此：

$$
\Phi = 2 \times \text{num} - \text{size} \ge 2 \times \frac{\text{size}}{2} - \text{size} = 0
$$

故 $\Phi_i \ge 0 = \Phi_0$ 对任意 $i$ 成立，满足 $\Phi(S_n) \ge \Phi(S_0)$，从而 $\sum \hat{C}_i = \sum C_i + \Phi_n - \Phi_0 \ge \sum C_i$。

---

#### （3）删除操作的摊还分析（4'）

考虑同时支持 `TABLE_DELETE` 的完整策略（负载因子低于 $1/4$ 时缩容）。以下是势能函数的定义：

$$
\Phi(T) = 
\begin{cases}
2 \cdot \text{num}[T] - \text{size}[T], & \text{若 } \alpha(T) \ge 1/2 \\[4pt]
\dfrac{1}{2}\text{size}[T] - \text{num}[T], & \text{若 } \alpha(T) < 1/2
\end{cases}
$$

其中 $\alpha(T) = \text{num}[T] / \text{size}[T]$ 为负载因子。

假设某次 `TABLE_DELETE` 操作前，负载因子 $\alpha_{i-1} \ge 1/2$，且删除后负载因子 $\alpha_i \ge 1/2$（即删除后未触发缩容且未进入低负载区域）。请计算此次删除操作的摊还代价 $\hat{C}_i$，并说明其含义。

**答案**：

**情况：$\alpha_{i-1} \ge 1/2$ 且 $\alpha_i \ge 1/2$**

此时使用势能函数 $\Phi = 2 \cdot \text{num} - \text{size}$。

删除操作：$\text{num}_i = \text{num}_{i-1} - 1$，$\text{size}_i = \text{size}_{i-1}$（未触发缩容）。实际代价 $C_i = 1$（仅删除）。

$$
\begin{aligned}
\hat{C}_i &= C_i + \Phi_i - \Phi_{i-1} \\
&= 1 + (2\text{num}_i - \text{size}_i) - (2\text{num}_{i-1} - \text{size}_{i-1}) \\
&= 1 + (2(\text{num}_{i-1}-1) - \text{size}_{i-1}) - (2\text{num}_{i-1} - \text{size}_{i-1}) \\
&= 1 + 2\text{num}_{i-1} - 2 - \text{size}_{i-1} - 2\text{num}_{i-1} + \text{size}_{i-1} \\
&= 1 - 2 = -1
\end{aligned}
$$

**含义**：摊还代价为 $-1$，表示此次删除操作不仅本身只需 $1$ 单位实际代价，还因减少了表中元素个数（从而降低了势能），释放了 $1$ 单位信用供后续可能的缩容使用。这说明删除操作在 $\alpha \ge 1/2$ 时是"赚钱"的，整体上保证了所有操作摊还代价均为常数。

#### （4）势能函数设计：1/3 与 2/3 阈值（Potential Design, Slide10）（4'）

假设动态表采用另一种策略：
- **扩容策略**：当负载因子 $\alpha > 2/3$ 时，将表容量加倍
- **缩容策略**：当负载因子 $\alpha < 1/3$ 时，将表容量减半

请设计一个势能函数 $\Phi(T)$，使其满足：
1. 在 $\alpha = 1/2$（均衡点）处 $\Phi = 0$
2. 在扩容阈值 $\alpha = 2/3$ 处 $\Phi$ 恰好等于复制所有元素所需的代价
3. 在缩容阈值 $\alpha = 1/3$ 处 $\Phi$ 恰好等于复制所有元素所需的代价
4. $\Phi(T) \ge 0$ 对所有状态成立

**答案**：

**势能函数设计**：

以 $\alpha = 1/2$ 为分界，分两段定义：

$$
\Phi(T) = \begin{cases}
4 \cdot \text{num}[T] - 2 \cdot \text{size}[T], & \alpha(T) \ge 1/2 \\[4pt]
\text{size}[T] - 2 \cdot \text{num}[T], & \alpha(T) < 1/2
\end{cases}
$$

**设计思路**：

对于 $\alpha \ge 1/2$ 区域，设 $\Phi = a \cdot \text{num} - b \cdot \text{size}$：
- 在 $\alpha = 1/2$：$\text{num} = \text{size}/2$，要求 $\Phi = 0$ ⇒ $a \cdot \frac{\text{size}}{2} - b \cdot \text{size} = 0$ ⇒ $a = 2b$
- 在 $\alpha = 2/3$（扩容点）：$\text{num} = 2 \cdot \text{size}/3$，扩容需复制 $\text{num}$ 个元素，要求 $\Phi = \text{num}$ ⇒ $a \cdot \frac{2 \cdot \text{size}}{3} - b \cdot \text{size} = \frac{2 \cdot \text{size}}{3}$ ⇒ $\frac{2a}{3} - b = \frac{2}{3}$
- 代入 $a = 2b$：$\frac{4b}{3} - b = \frac{2}{3}$ ⇒ $\frac{b}{3} = \frac{2}{3}$ ⇒ $b = 2, a = 4$
- 得 $\Phi = 4 \cdot \text{num} - 2 \cdot \text{size}$

对于 $\alpha < 1/2$ 区域，设 $\Phi = c \cdot \text{size} - d \cdot \text{num}$：
- 在 $\alpha = 1/2$：要求 $\Phi = 0$ ⇒ $c \cdot \text{size} - d \cdot \frac{\text{size}}{2} = 0$ ⇒ $c = d/2$
- 在 $\alpha = 1/3$（缩容点）：$\text{num} = \text{size}/3$，缩容需复制 $\text{num}$ 个元素，要求 $\Phi = \text{num}$ ⇒ $c \cdot \text{size} - d \cdot \frac{\text{size}}{3} = \frac{\text{size}}{3}$ ⇒ $c - \frac{d}{3} = \frac{1}{3}$
- 代入 $c = d/2$：$\frac{d}{2} - \frac{d}{3} = \frac{1}{3}$ ⇒ $\frac{d}{6} = \frac{1}{3}$ ⇒ $d = 2, c = 1$
- 得 $\Phi = \text{size} - 2 \cdot \text{num}$

**验证**：

| 状态 | $\alpha$ | num | $\Phi$ | 含义 |
|------|----------|-----|-------|------|
| 均衡点 | $1/2$ | $\text{size}/2$ | $4\cdot\frac{\text{size}}{2} - 2\cdot\text{size} = 0$ | 势能为 0 |
| 扩容前 | $2/3$ | $2\cdot\text{size}/3$ | $4\cdot\frac{2\cdot\text{size}}{3} - 2\cdot\text{size} = \frac{2\cdot\text{size}}{3} = \text{num}$ | 势能恰好支付复制代价 |
| 缩容前 | $1/3$ | $\text{size}/3$ | $\text{size} - 2\cdot\frac{\text{size}}{3} = \frac{\text{size}}{3} = \text{num}$ | 势能恰好支付复制代价 |

**非负性**：当 $\alpha \ge 1/2$ 时，$\Phi = 4\text{num} - 2\text{size} \ge 4 \cdot \frac{\text{size}}{2} - 2 \cdot \text{size} = 0$。当 $\alpha < 1/2$ 时，$\Phi = \text{size} - 2\text{num} > \text{size} - 2 \cdot \frac{\text{size}}{2} = 0$。故 $\Phi \ge 0$ 恒成立，满足势能法条件。

**INSERT 摊还代价**（$\alpha \ge 1/2$ 区域未触发扩容时）：$\hat{C}_i = 1 + \Delta\Phi = 1 + 4 = 5$。

**DELETE 摊还代价**（$\alpha < 1/2$ 区域未触发缩容时）：$\hat{C}_i = 1 + \Delta\Phi = 1 + 2 = 3$。

各操作摊还代价均为常数，故 $n$ 次操作总时间 $O(n)$。

### 2. 二进制计数器与多栈的摊还分析（15 分）

#### （1）二进制计数器：聚集分析（Binary Counter Aggregate, Slide10）（5'）

考虑一个 $k$ 位二进制计数器，初始为全 0。对其执行 $n$ 次 `INCREMENT` 操作。

**（a）** 使用聚集分析法，证明 $n$ 次 `INCREMENT` 操作的总时间复杂度为 $O(n)$，从而摊还代价为 $O(1)$。（3'）

**（b）** 如果计数器不是从 0 开始，而是从某个任意数 $m$ 开始计数到 $n$（$m < n$），聚集分析是否仍然成立？为什么？（2'）

---

**答案**：

**（a）** 关键观察：位 $A[i]$ 每 $2^i$ 次 `INCREMENT` 翻转一次，$n$ 次操作中 $A[i]$ 翻转 $\lfloor n/2^i \rfloor$ 次。

$$
T(n) = \sum_{i=0}^{k-1} \left\lfloor \frac{n}{2^i} \right\rfloor < n \cdot \sum_{i=0}^{\infty} \frac{1}{2^i} = 2n
$$

因此总时间复杂度 $T(n) = O(n)$，摊还代价 $= T(n)/n = O(1)$。

**（b）** 仍然成立。从 $m$ 计数到 $n$ 共 $L = n-m$ 次操作。对于任意位 $A[i]$，其翻转周期为 $2^{i+1}$（每 $2^i$ 次 INCREMENT 翻转一次）。在任何连续的 $L$ 次 INCREMENT 中，$A[i]$ 最多翻转 $\lceil L/2^i \rceil$ 次（不依赖于起始值 $m$），因为两次相邻翻转之间至少间隔 $2^i$ 次操作。

因此总翻转次数：
$$T = \sum_{i=0}^{k-1} \lceil L/2^i \rceil \le \sum_{i=0}^{k-1} \left( \frac{L}{2^i} + 1 \right) = L \sum_{i=0}^{k-1} \frac{1}{2^i} + k < 2L + k = O(L)$$

摊还代价 $= T/L = O(1)$。

---

#### （2）二进制计数器：记账法与势能法（Binary Counter Accounting & Potential, Slide10）（6'）

**（a）** 使用记账法分析 `INCREMENT` 操作：设置 $0 \to 1$ 翻转的摊还代价为 $2$，$1 \to 0$ 翻转的摊还代价为 $0$。解释为什么信用永远不会为负，并证明总摊还代价 $\le 2n$。（3'）

**（b）** 使用势能法分析 `INCREMENT` 操作：定义势能函数 $\Phi = \text{计数器中 1 的个数}$。推导每次 `INCREMENT` 的摊还代价为 $2$。（3'）

---

**答案**：

**（a）** 每次 `INCREMENT` 恰好将一个 $0$ 翻转为 $1$（可能翻转多个 $1 \to 0$），所以 $0 \to 1$ 翻转的次数 $\le n$。每次 $0 \to 1$ 翻转收取 $\$2$，其中 $\$1$ 用于本次翻转，$\$1$ 作为信用存入该位。该信用在将来该位被翻转回 $0$ 时支付。由于每个 $1$ 位都有 $\$1$ 信用，信用总量 $= \text{计数器中 1 的个数} \ge 0$，故信用永不值为负。

总摊还代价：

$$
\sum \hat{C}_i = 2 \times (\# 0 \to 1 \text{ 翻转次数}) \le 2n
$$

**（b）** 设第 $i$ 次 `INCREMENT` 翻转了 $t_i$ 个 $1 \to 0$ 和 $1$ 个 $0 \to 1$，实际代价 $C_i = t_i + 1$。

势能变化：

$$
\Phi_i - \Phi_{i-1} = 1 - t_i \quad (\text{增加了一个 }1\text{，减少了 }t_i\text{ 个 }1)
$$

摊还代价：

$$
\hat{C}_i = C_i + \Phi_i - \Phi_{i-1} = (t_i + 1) + (1 - t_i) = 2
$$

由于 $\Phi \ge 0 = \Phi_0$，满足势能法条件 $\Phi(S_n) \ge \Phi(S_0)$，故 $\sum C_i \le \sum \hat{C}_i = 2n$。

---

#### （3）多栈推入操作的摊还分析（Multistack Push, Slide10）（4'）

考虑一个多栈（Multistack）结构，由一系列普通栈 $S_0, S_1, S_2, \dots$ 组成，其中 $S_i$ 的容量为 $2^i$。执行 `PUSH(x)` 操作时：

1. 优先将元素压入 $S_0$；
2. 若 $S_0$ 已满，则将 $S_0$ 中所有元素弹出并压入 $S_1$；
3. 若 $S_1$ 也已满，则递归地将其元素全部迁移到 $S_2$；
4. 依此类推，直到找到第一个有空位的栈。

假设每个栈的 push/pop 操作耗时 $O(1)$。

**（a）** 分析一次 `PUSH` 操作在最好情况和最坏情况下的时间复杂度。（1'）

**（b）** 使用聚集分析或势能法，证明 $n$ 次 `PUSH` 操作的摊还代价为 $O(\log n)$。（3'）

---

**答案**：

**（a）** 最好情况：$S_0$ 未满，直接入栈 $O(1)$。

最坏情况：$S_0, S_1, \dots, S_{k-1}$ 全部满，需要迁移所有元素到 $S_k$，涉及迁移 $2^0 + 2^1 + \cdots + 2^{k-1} = 2^k - 1$ 个元素（接近 $n$ 个），耗时 $O(n)$。

**（b）** **聚集分析法**：

每次 push 本身代价 $1$，共 $n$。

$S_i$ 每 $2^i$ 次 push 触发一次迁移：将 $2^i$ 个元素从 $S_i$ 弹出并压入 $S_{i+1}$，迁移代价为 $2 \times 2^i = 2^{i+1}$（弹出 + 压入）。

$n$ 次 push 中，$S_i$ 的迁移触发次数为 $\lfloor n/2^i \rfloor$。

$$
\begin{aligned}
T(n) &= n + \sum_{i=0}^{\lfloor \log_2 n \rfloor} \left\lfloor \frac{n}{2^i} \right\rfloor \times 2^{i+1} \\
&\le n + \sum_{i=0}^{\lfloor \log_2 n \rfloor} \frac{n}{2^i} \times 2^{i+1} \\
&= n + \sum_{i=0}^{\lfloor \log_2 n \rfloor} 2n \\
&= n + 2n \cdot (\lfloor \log_2 n \rfloor + 1) \\
&= O(n \log n)
\end{aligned}
$$

因此摊还代价 $= T(n)/n = O(\log n)$。
