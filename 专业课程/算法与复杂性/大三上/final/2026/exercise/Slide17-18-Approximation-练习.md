# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：Approximation Algorithm（Slides 17-18）

---

## 一、选择题

**1.（Approximation Ratio Definition, Slide17）** 关于近似比（Performance Ratio）的定义，以下说法正确的是？

- (A) 对于最小化问题，近似比为 $R(x,y) = \frac{opt(x)}{m(x,y)}$；对于最大化问题，近似比为 $R(x,y) = \frac{m(x,y)}{opt(x)}$
- (B) 近似比定义为 $R(x,y) = \max\left\{\frac{m(x,y)}{opt(x)}, \frac{opt(x)}{m(x,y)}\right\}$，对最大化和最小化问题统一处理
- (C) 近似比定义为 $R(x,y) = \frac{|m(x,y) - opt(x)|}{opt(x)}$
- (D) 近似比定义为 $R(x,y) = \frac{m(x,y)}{opt(x)}$，适用于所有优化问题

**答案**：B **解析**：根据 Slide17 定义，$R(x,y) = \max\left\{\frac{m(x,y)}{opt(x)}, \frac{opt(x)}{m(x,y)}\right\}$。这样统一处理了最大化和最小化问题，保证近似比始终 $\geq 1$。选项 A 对两种问题的定义反了；C 是绝对误差；D 对最小化问题会小于 1。

---

**2.（APX Class Hierarchy, Slide17）** 关于近似算法复杂度类的包含关系，以下哪个是正确的？（假设 $P \neq NP$）

- (A) $FPTAS \subseteq PTAS \subseteq APX \subseteq Log\text{-}APX \subseteq Poly\text{-}APX \subseteq Exp\text{-}APX \subseteq NPO$
- (B) $NPO \subseteq Exp\text{-}APX \subseteq Poly\text{-}APX \subseteq Log\text{-}APX \subseteq APX \subseteq PTAS \subseteq FPTAS$
- (C) $APX \subseteq PTAS \subseteq FPTAS \subseteq Log\text{-}APX \subseteq Poly\text{-}APX \subseteq NPO$
- (D) $PTAS \subseteq FPTAS \subseteq APX \subseteq Log\text{-}APX \subseteq Poly\text{-}APX \subseteq NPO$

**答案**：A **解析**：根据 Slide17，当 $P \neq NP$ 时，包含关系为 $FPTAS \subseteq PTAS \subseteq APX \subseteq Log\text{-}APX \subseteq Poly\text{-}APX \subseteq Exp\text{-}APX \subseteq NPO$。FPTAS 是最强的（时间关于 $1/\epsilon$ 多项式），越往右近似比越弱（常数 → 对数 → 多项式 → 指数）。

---

**3.（LP Rounding for Set Cover, Slide18）** 设 Set Cover 问题中每个元素最多出现在 $f$ 个集合中（即元素的最大频率为 $f$）。用 LP Rounding 确定性取整算法，给定 LP 松弛的最优解 $x^*_S$，取整策略为：当 $x^*_S \geq 1/f$ 时将 $x_S$ 取整为 1，否则取整为 0。关于该算法的近似比，以下说法正确的是？

- (A) 近似比为 $H_n$（调和数）
- (B) 近似比为 $f$
- (C) 近似比为 $2f$
- (D) 近似比为 $\log n$

**答案**：B **解析**：根据 Slide18，确定性 LP Rounding 的近似比为 $f$。原因：对于每个元素 $e$，由于 $\sum_{S:e \in S} x_S \geq 1$ 且 $e$ 最多出现在 $f$ 个集合中，必有某个 $S$ 满足 $x_S \geq 1/f$，所以 $e$ 被覆盖。取整后每个 $x_S$ 最多放大 $f$ 倍，故 $cost(S') \leq f \cdot OPT_f \leq f \cdot OPT$。

---

**4.（Local Search for Max Cut, Slide18）** 关于用 Local Search 求解 Maximum Cut 问题的近似算法，下列说法错误的是？

- (A) 邻域结构定义为将任一顶点从当前分区移动到另一分区
- (B) 局部最优解的割大小至少为总边数的一半
- (C) 该算法的近似比为 2
- (D) 该算法一定能在多项式时间内找到全局最优解

**答案**：D **解析**：A 正确——邻域定义为单顶点移动；B 正确——由 $2m_1 - m_N(G) \leq 0$ 和 $2m_2 - m_N(G) \leq 0$ 可推出 $m_N(G) \geq m/2$；C 正确——$m^*(G)/m_N(G) \leq 2$；D 错误——Local Search 只能保证找到局部最优解，不一定找到全局最优解。

---

**5.（Greedy Set Cover Approximation Ratio, Slide17）** Greedy Set Cover 算法的近似比为 $H_n$（$H_n = 1 + 1/2 + \cdots + 1/n$ 为调和数）。关于该界，以下说法正确的是？

- (A) 该界是紧的（tight），存在实例使得近似比任意接近 $H_n$
- (B) 该界不紧，实际近似比远好于 $H_n$
- (C) Greedy Set Cover 属于 PTAS
- (D) Greedy Set Cover 属于 FPTAS

**答案**：A **解析**：Slide17 给出了一个 tight example，最优覆盖代价为 $1+\epsilon$ 而贪心算法代价为 $H_n$，因此界是紧的。由于 $H_n = \Theta(\log n)$，Greedy Set Cover 属于 Log-APX，而非 PTAS 或 FPTAS。

---

## 二、问答题

### 1. Greedy Knapsack 与改进（18 分）

#### （1）Greedy Knapsack 算法与反例（Greedy Strategy, Slide17）（8'）

对于 Maximum Knapsack 问题，Greedy Knapsack 算法按价值密度 $p_i/a_i$（单位重量的价值）降序依次选取物品。请给出一个具体实例，说明该算法的解与最优解的比值可以任意大（即不是常数倍近似）。

**答案**：

考虑如下实例：共有 $n$ 个物品，背包容量 $b = kn$（$k$ 为任意大的正数）。

- 物品 $1$ 到 $n-1$：$p_i = a_i = 1$（价值 1，重量 1）
- 物品 $n$：$p_n = b - 1 = kn - 1$，$a_n = b = kn$

按 $p_i/a_i$ 排序，前 $n-1$ 个物品的密度为 $1$，物品 $n$ 的密度为 $(kn-1)/(kn) < 1$，因此贪心算法先选前 $n-1$ 个物品，总价值 $mg = n-1$，占用容量 $n-1$，剩余容量 $kn - (n-1)$ 无法装入物品 $n$。

最优解：只选物品 $n$，总价值 $m^* = kn - 1$。

$$
\frac{m^*}{mg} > \frac{kn - 1}{n - 1} > k
$$

由于 $k$ 可以任意大，比值可以任意大。

---

#### （2）改进的 2-近似算法（Improved Bound, Slide17）（10'）

证明：若令 $m_H(X) = \max\{p_{\max}, m_g(X)\}$，其中 $p_{\max}$ 为单个物品的最大价值，$m_g(X)$ 为 Greedy Knapsack 的解，则 $m_H(X)$ 满足 $\frac{m^*(X)}{m_H(X)} < 2$。

**答案**：

1. 假设我们在做分数背包时，按性价比排序后，恰好在第 $k$ 个物品时把背包装满（第 $k$ 个物品只装了切片）。
2. 此时分数背包的最优解为 $OPT_{fractional}$。因为分数背包是 0-1 背包的松弛问题，所以显然有：$OPT \le OPT_{fractional}$
3. 考虑前 $k-1$ 个完整装入的物品，它们的价值总和为 $S = \sum_{i=1}^{k-1} v_i$。显然常规贪心解 $V_{greedy} \ge S$。
4. 分数背包的解可以表示为：$OPT_{fractional} = S + \alpha \cdot v_k$（其中 $0 < \alpha < 1$）。因此：$OPT_{fractional} < S + v_k$
5. 结合以上两点：$OPT \le OPT_{fractional} < S + v_k \le V_{greedy} + V_{max}$
6. 根据鸽巢原理（或简单的放大不等式），$V_{greedy}$ 和 $V_{max}$ 中至少有一个大于或等于 $\frac{1}{2}(V_{greedy} + V_{max})$：$A_{mod}(I) = \max(V_{greedy}, V_{max}) \ge \frac{1}{2}(V_{greedy} + V_{max}) > \frac{1}{2} OPT$

---

### 2. Local Search 与 LP Rounding（18 分）

#### （1）Local Search for Parallel Job Scheduling（Local Search, Slide18）（9'）

考虑 Parallel Job Scheduling 问题：$n$ 个作业，$m$ 台相同机器，每台机器一次最多处理一个作业。Local Search 算法反复将最后完成的作业移动到最早空闲的机器上，直到无法改进。

证明该算法是 2-近似算法，并说明其时间复杂度为 $O(n)$。

**答案**：

**近似比证明**：

令 $C^*_{\max}$ 为最优调度方案的完工时间。由于每个作业必须被处理，故 $C^*_{\max} \geq \max_j p_j$。总处理时间 $P = \sum_{j=1}^n p_j$，$m$ 台机器的平均负载为 $P/m$，必然有机器分配至少平均负载的工作量，故 $C^*_{\max} \geq P/m$。

考虑 Local Search 的最终调度。设 $\ell$ 为最后完成作业，$C_\ell = C_g$ 为当前调度完工时间。由于算法终止时 $\ell$ 无法被转移，每台其他机器从时间 0 到 $S_\ell = C_\ell - p_\ell$（$\ell$ 的开始时间）必须一直忙绿。

将时间线以 $S_\ell$ 为界分为两段：
- 前一段：总工作量 $mS_\ell \leq P$，因此 $S_\ell \leq P/m \leq C^*_{\max}$
- 后一段：长度为 $p_\ell \leq C^*_{\max}$（因为 $C^*_{\max} \geq \max_j p_j$）

因此：
$$
C_g = S_\ell + p_\ell \leq C^*_{\max} + C^*_{\max} = 2C^*_{\max}
$$

故近似比为 2。$\square$

**时间复杂度假证明**：

我们证明每个作业最多被重新调度一次。设 $C_{\min}$ 为最早完成机器的完工时间，易见 $C_{\min}$ 在算法过程中单调不减。

假设某个作业 $j$ 被重新调度了两次：从机器 $i$ 转移到 $i'$，再从 $i'$ 转移到 $i^*$。第一次转移时，$j$ 在 $i'$ 上的开始时间为当前的 $C_{\min}$；第二次转移时，$j$ 在 $i^*$ 上的开始时间为当前的 $C'_{\min}$。两次转移之间 $i'$ 上的调度没有发生变化，因此 $C'_{\min}$ 必须严格小于 $C_{\min}$，这与 $C_{\min}$ 单调不减矛盾。

每个作业至多被考虑一次，故时间复杂度为 $O(n)$。$\square$

---

#### （2）LP Randomized Rounding for Set Cover（LP Rounding, Slide18）（9'）

对于 Set Cover 问题，LP 随机舍入算法如下：
- **Step 1**：求解 LP 松弛得到最优解 $x^*_S$，以概率 $x^*_S$ 独立选取每个集合 $S$ 到 $S'$。
- **Step 2**：重复 Step 1 共 $c\log n$ 次（$c$ 为满足 $(1/e)^{c\log n} \leq 1/(4n)$ 的常数），取所有被选集合的并集 $C'$。

请回答以下问题：

（a）证明 Step 1 的期望代价等于 LP 最优值 $OPT_f$。（3'）

（b）证明单个元素在 Step 1 中未被覆盖的概率不超过 $1/e$。（3'）

（c）说明为何需要 Step 2，并证明 $C'$ 是有效 set cover 且代价不超过 $OPT_f \cdot 4c\log n$ 的概率至少为 $1/2$。（3'）

**答案**：

**（a）期望代价**：

设指示变量 $I_S$ 表示集合 $S$ 是否被选中，则 $\Pr[I_S = 1] = x^*_S$。Step 1 的期望代价为：
$$
E[cost(S')] = \sum_{S \in S} \Pr[S \text{ is picked}] \cdot c(S) = \sum_{S \in S} x^*_S \cdot c(S) = OPT_f
$$

其中 $OPT_f$ 是 LP 松弛的最优值。$\square$

**（b）单个元素的未覆盖概率**：

设元素 $e_i$ 出现在 $k$ 个集合 $S_1, \ldots, S_k$ 中。由于 $e_i$ 被分数覆盖，有 $x_{S_1} + \cdots + x_{S_k} \geq 1$。

$e_i$ 未被覆盖的概率为：
$$
\Pr[e_i \text{ not covered}] = \prod_{i=1}^k (1 - x_{S_i})
$$

应用 AM-GM 不等式 $\sqrt[k]{x_1 \cdots x_k} \leq \frac{1}{k}(x_1 + \cdots + x_k)$：
$$
\prod_{i=1}^k (1 - x_{S_i}) \leq \left(\frac{k - (x_{S_1} + \cdots + x_{S_k})}{k}\right)^k \leq \left(1 - \frac{1}{k}\right)^k \leq \frac{1}{e}
$$

其中最后一步利用了 $\left(1 - \frac{1}{k}\right)^k = e^{k\ln(1-1/k)} \leq e^{k \cdot (-1/k)} = e^{-1}$。$\square$

**（c）Step 2 的必要性与成功概率分析**：

Step 1 中单个元素未被覆盖的概率高达 $1/e$，且 $S'$ 很可能不是有效的 set cover。需要重复多次来提高覆盖率。

Step 2 重复 $c\log n$ 次后，单个元素 $e_i$ 在 $C'$ 中仍未被覆盖的概率为：
$$
\Pr[e_i \text{ not covered by } C'] \leq \left(\frac{1}{e}\right)^{c\log n} \leq \frac{1}{4n}
$$

由 Union Bound，$C'$ 不是有效 set cover 的概率：
$$
\Pr[C' \text{ not a valid set cover}] \leq n \cdot \frac{1}{4n} = \frac{1}{4}
$$

Step 2 的期望代价 $E[cost(C')] \leq OPT_f \cdot c\log n$。由 Markov 不等式：
$$
\Pr[cost(C') \geq OPT_f \cdot 4c\log n] \leq \frac{E[cost(C')]}{OPT_f \cdot 4c\log n} \leq \frac{1}{4}
$$

结合两个条件：
$$
\Pr[C' \text{ is valid} \land cost(C') \leq OPT_f \cdot 4c\log n] \geq 1 - \frac{1}{4} - \frac{1}{4} = \frac{1}{2}
$$

即至少以 $1/2$ 的概率，$C'$ 是有效 set cover 且代价不超过 $O(\log n)$ 倍最优值。若不满足则可重复整个算法，期望重复次数不超过 2 次。$\square$

---

### 3. Sequential Algorithm 设计与分析（14 分）

#### （1）Sequential Maximum Cut（Sequential Algorithm, Slide17）（7'）

对于 Maximum Cut 问题，Sequential 算法的策略是：任选两个顶点分别放入 $A$ 和 $B$，然后对剩余每个顶点 $v$，若 $v$ 到 $A$ 的边数不少于到 $B$ 的边数，则将 $v$ 放入 $B$，否则放入 $A$。

证明该算法是 2-近似算法，并给出一个紧例。

**答案**：

**近似比证明**：

设 $m$ 为总边数。每条边 $(v_i, v_j)$ 是否属于割由两个端点被处理时决定。算法的每次迭代中，至少有一半与该顶点相关的边会被分配到割中（因为总是将顶点放到连接边数较少的一侧），且这些边一旦分配不再改变。

因此 $|A_g| \geq |E|/2$。显然 $|OPT| \leq |E|$，故：
$$
\frac{|OPT|}{|A_g|} \leq \frac{|E|}{|E|/2} = 2
$$

$\square$

**紧例**：

考虑一个 4 个顶点的环图 $C_4$：
- 顶点 $1, 2, 3, 4$ 依次相连，边集为 $\{(1,2), (2,3), (3,4), (4,1)\}$

假设算法先选 $v_1 \in A$，$v_2 \in B$。
- 处理 $v_3$：$d(v_3, A) = 1$（连 $v_2$，但 $v_2 \in B$，所以实际 $d(v_3, A)=0$），$d(v_3, B) = 1$（连 $v_2$），由于 $0 < 1$，$v_3$ 放入 $A$。
- 处理 $v_4$：$d(v_4, A) = 1$（连 $v_1$ 且 $v_1 \in A$），$d(v_4, B) = 1$（连 $v_3$ 且 $v_3 \in A$，所以实际为 0），$d(v_4, A) \geq d(v_4, B)$，$v_4$ 放入 $B$。

结果：$A = \{1, 3\}$，$B = \{2, 4\}$，割边数为 2（$(1,2)$ 和 $(3,4)$）。

最优割：$A = \{1, 2\}$，$B = \{3, 4\}$，割边数为 4。

因此 $|OPT|/|A_g| = 4/2 = 2$，界是紧的。

---

#### （2）LPT 调度算法（Sequential Algorithm, Slide17）（7'）

对于 Minimum Scheduling on Identical Machines 问题，LPT（Largest Processing Time）算法先将作业按处理时间非增序排列，然后依次将每个作业分配给当前负载最小的机器。

证明该算法的近似比为 $\frac{4}{3} - \frac{1}{3p}$，其中 $p$ 为机器数。

**答案**：

**证明**：记 $m^*(T)$ 为最优完工时间。设 $t_j$ 为 LPT 算法最后考虑的作业，其长度为 $l_{\min}$。

**Case 1**：$l_{\min} > m^*(T)/3$

此时每台机器最多分配 2 个作业（否则某台机器的总处理时间会超过 $3 \cdot m^*(T)/3 = m^*(T)$，与最优性矛盾）。因此 $p < |T| \leq 2p$。

可以证明当 $|T| \leq 2p$ 时，LPT 能得到最优解。通过添加虚拟的零长度作业使 $|T| = 2p$，LPT 和最优解都会将这 $2p$ 个作业配成 $p$ 对。假设 $m_L(T) = l_i + l_{2p-i+1}$ 是第 $i$ 台机器的完工时间（即 makespan），若 $l_{2p-i+1} = 0$ 则 $m_L(T) = m^*(T) = l_i$。若 $l_{2p-i+1} > 0$ 而 $m_L(T) > m^*(T)$，则在最优解中与 $\{l_1, \ldots, l_{i-1}\}$ 配对的必须来自 $\{l_{2p-i+2}, \ldots, l_{2p}\}$，否则新配对会更大，矛盾。

因此 $m_L(T) = m^*(T)$，近似比为 1。

**Case 2**：$l_{\min} \leq m^*(T)/3$

令 $W = \sum_{k=1}^{|T|} l_k$。由于 $m^*(T) \geq W/p$（平均负载下界），且 $t_j$ 被分配给当前负载最小的机器，该机器的完工时间 $A_h(|T|)$ 即为 makespan $m_L(T)$。

在 $t_j$ 被分配前，其他机器的完工时间至少为 $A_h(|T|) - l_{\min}$。因此：
$$
W \geq p(A_h(|T|) - l_{\min}) + l_{\min}
$$

整理得：
$$
m_L(T) = A_h(|T|) \leq \frac{W}{p} + \frac{p-1}{p}l_{\min}
$$

利用 $m^*(T) \geq W/p$ 和 $l_{\min} \leq m^*(T)/3$：
$$
m_L(T) \leq m^*(T) + \frac{p-1}{3p}m^*(T) = \left(\frac{4}{3} - \frac{1}{3p}\right)m^*(T)
$$

综合两情形，LPT 的近似比为 $\frac{4}{3} - \frac{1}{3p}$。$\square$

---

### 4. Greedy Algorithm 设计与分析（15 分）

#### （1）Greedy Independent Set（Greedy Algorithm, Slide17）（7'）

对于 Maximum Independent Set 问题，Greedy 算法每次选择当前剩余图中度数最小的顶点加入独立集，然后删除该顶点及其所有邻居。

证明该算法的近似比为 $\delta + 1$，其中 $\delta = m/n$ 是图的平均度数，并说明为什么该问题属于 Poly-APX。

**答案**：

**证明**：设 $V^*$ 为最优独立集，$m^*(G) = |V^*|$。在第 $i$ 轮迭代中，令 $x_i$ 为所选顶点，$d_i$ 为 $x_i$ 的度数，则本轮删除 $d_i + 1$ 个顶点。记 $k_i$ 为本轮删除的顶点中属于 $V^*$ 的个数。

算法终止时所有顶点被删除，故：
$$
\sum_{i=1}^{m_g(G)} (d_i + 1) = n \quad \text{(1)}
$$
$$
\sum_{i=1}^{m_g(G)} k_i = |V^*| = m^*(G) \quad \text{(2)}
$$

每个 $x_i$ 及 $d_i + 1$ 个顶点内部至少贡献 $\frac{d_i(d_i+1)}{2}$ 条边。但 $V^*$ 中顶点间无边相连，需扣除 $k_i$ 个顶点间不存在的边：
$$
\sum_{i=1}^{m_g(G)} \frac{d_i(d_i+1) + k_i(k_i-1)}{2} \leq m = \delta n \quad \text{(3)}
$$

(1)+(2)+(3)：
$$
\sum_{i=1}^{m_g(G)} \left[(d_i+1)^2 + k_i^2\right] \leq n(2\delta + 1) + m^*(G)
$$

由 Cauchy-Schwarz 不等式，左边在 $d_i+1 = \frac{n}{m_g(G)}$ 和 $k_i = \frac{m^*(G)}{m_g(G)}$ 时取极小：
$$
\frac{n^2 + m^*(G)^2}{m_g(G)} \leq n(2\delta + 1) + m^*(G)
$$

推导得：
$$
\frac{m^*(G)}{m_g(G)} \leq \delta + 1
$$

因此 Greedy Independent Set 的近似比为 $\delta + 1$。由于 $\delta$ 可以是 $n$ 的多项式（如完全图 $K_n$ 中 $\delta = (n-1)/2$），该算法属于 Poly-APX。$\square$

---

#### （2）Set Cover 贪心算法的 price 分析（Greedy Algorithm, Slide17）（8'）

对于 Set Cover 问题，Greedy 算法每次选择性价比最优（即 $c(S)/|S - C|$ 最小）的集合。算法的近似比为 $H_n$。

证明过程中用到了以下关键引理：将元素按被覆盖的次序编号为 $e_1, \ldots, e_n$，则对任意 $k \in \{1, \ldots, n\}$，有：
$$
price(e_k) \leq \frac{m^*(U)}{n - k + 1}
$$

请证明该引理，并据此推导算法的近似比 $H_n$。

**答案**：

**引理证明**：

设 $C$ 为当前已覆盖的元素集合。在覆盖 $e_k$ 的迭代开始时，至少还有 $n - k + 1$ 个元素未被覆盖，即 $|\overline{C}| \geq n - k + 1$。

最优解可以用代价 $m^*(U)$ 覆盖所有剩余元素，因此在剩余的集合中，必有一个集合 $S$ 的性价比不高于 $m^*(U)/|\overline{C}|$（否则所有剩余集合的性价比均高于该值，总代价将超过最优解）。

Greedy 算法选择性价比最优的集合，因此当前迭代所选集合的性价比 $\leq m^*(U)/|\overline{C}|$，从而 $price(e_k) \leq m^*(U)/|\overline{C}| \leq m^*(U)/(n - k + 1)$。$\square$

**近似比推导**：

算法总代价等于所有元素 price 之和：
$$
m_g(U) = \sum_{k=1}^n price(e_k) \leq \sum_{k=1}^n \frac{m^*(U)}{n - k + 1} = m^*(U) \cdot \sum_{i=1}^n \frac{1}{i} = H_n \cdot m^*(U)
$$

因此近似比为 $H_n$。$\square$

---

## 三、附加思考题（Bonus）

### （Local Search for Max Cut, Slide18）

在 Local Search 求解 Max Cut 的近似比证明中，利用了局部最优的邻域条件：$\forall v_i \in V_1$，$|m_{1i}| - |m_{2i}| \leq 0$；$\forall v_j \in V_2$，$|m_{2j}| - |m_{1j}| \leq 0$。请解释这些不等式的含义，并说明如何由此推出 $m_N(G) \geq m/2$。

**答案**：

**不等式含义**：对于局部最优割 $(V_1, V_2)$，若将任一顶点移动到另一侧，割的大小不会增加。例如，$v_i \in V_1$ 当前在 $V_1$ 侧，$|m_{1i}|$ 是 $v_i$ 与 $V_1$ 内顶点的边数（不在割中），$|m_{2i}|$ 是 $v_i$ 与 $V_2$ 内顶点的边数（在割中）。将 $v_i$ 移到 $V_2$ 后，新的割大小变化为 $|m_{1i}| - |m_{2i}|$（原先不在割中的边现在加入，原先在割中的边现在移除）。由于是局部最优，此变化 $\leq 0$，即 $|m_{1i}| - |m_{2i}| \leq 0$。

**推导**：对 $V_1$ 中所有顶点求和：
$$
\sum_{v_i \in V_1} (|m_{1i}| - |m_{2i}|) = 2m_1 - m_N(G) \leq 0 \Rightarrow 2m_1 \leq m_N(G)
$$

对 $V_2$ 中所有顶点求和：
$$
\sum_{v_j \in V_2} (|m_{2j}| - |m_{1j}|) = 2m_2 - m_N(G) \leq 0 \Rightarrow 2m_2 \leq m_N(G)
$$

两式相加：$2(m_1 + m_2) \leq 2m_N(G)$，即 $m_1 + m_2 \leq m_N(G)$。

由 $m = m_1 + m_2 + m_N(G)$ 得 $m - m_N(G) \leq m_N(G)$，故 $m_N(G) \geq m/2$。
