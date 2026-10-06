# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：Shortest Path（Slide13）

---

## 一、选择题

**1.（Dijkstra's Algorithm Correctness, Slide13）** Dijkstra 算法要求图中所有边权非负的根本原因是什么？

- (A) 否则算法的时间复杂度会退化到 O(n²)
- (B) 否则最优子结构性质不再成立
- (C) 否则从优先队列中取出的最小距离顶点可能不是真正的最短距离（贪心选择失效）
- (D) 否则初始化步骤无法正确设置距离数组

**答案**：C **解析**：Dijkstra 算法是贪心算法，每次从 Q 中 EXTRACT-MIN 取当前距离最小的顶点 u，并断言 d[u] = d(s, u)。当边权非负时，由于所有未处理顶点的距离 ≥ d[u]，不可能通过后续顶点得到更短的 s→u 路径。但存在负权边时，可能出现 d[u] 看似最小但实际上可通过负权边得到更短路径的情况。

---

**2.（Bellman-Ford Negative Cycle Detection, Slide13）** Bellman-Ford 算法执行 n − 1 轮松弛后，再执行第 n 轮。若第 n 轮中某个顶点 v 的距离值 M[v] 被更新（减小），这说明什么？

- (A) 图中有正权环
- (B) 从源点 s 到 v 的最短路径包含负权环
- (C) 算法还未收敛，需要继续执行第 n+1 轮
- (D) 图中存在负权边但不存在负权环

**答案**：B **解析**：讲义 Lemma 指出：If OPT(n, v) < OPT(n − 1, v) for some node v, then (any) shortest path from s to v contains a cycle W and W has negative cost。因为无环简单路径最多使用 n − 1 条边，第 n 轮更新意味着最短路径包含 n 条边，由鸽笼原理必含环，且环的权值为负。

---

**3.（Floyd-Warshall Loop Order, Slide13）** 以下哪段 Floyd-Warshall 伪代码能正确计算出所有顶点对之间的最短路径？

- (A)
```
for i ← 1 to n:
    for j ← 1 to n:
        for k ← 1 to n:
            if cij > cik + ckj then cij = cik + ckj
```
- (B)
```
for k ← 1 to n:
    for i ← 1 to n:
        for j ← 1 to n:
            if cij > cik + ckj then cij = cik + ckj
```
- (C)
```
for i ← 1 to n:
    for k ← 1 to n:
        for j ← 1 to n:
            if cij > cik + ckj then cij = cik + ckj
```
- (D)
```
for k ← 1 to n:
    for j ← 1 to n:
        for i ← 1 to n:
            if cij > cik + ckj then cij = cik + ckj
```

**答案**：B **解析**：Floyd-Warshall 的核心是 DP 定义 c⁽ᵏ⁾ᵢⱼ 为仅允许使用 {1,2,...,k} 作为中间节点的最短路径长度。递推式 c⁽ᵏ⁾ᵢⱼ = min{c⁽ᵏ⁻¹⁾ᵢⱼ, c⁽ᵏ⁻¹⁾ᵢₖ + c⁽ᵏ⁻¹⁾ₖⱼ} 要求最外层循环必须是 k（中间节点集合的规模），以确保 c⁽ᵏ⁻¹⁾ 在 c⁽ᵏ⁾ 之前已计算完毕。这与蓝本 MC8 的考点一致。

---

**4.（Johnson's Algorithm, Slide13）** Johnson 算法中，使用 Bellman-Ford 计算顶点标记 h(v) 的主要目的是什么？

- (A) 消除图中的负权环
- (B) 使重新加权后的所有边权非负，从而可以对每个顶点运行 Dijkstra
- (C) 减少图的顶点数量
- (D) 降低 Floyd-Warshall 的时间复杂度

**答案**：B **解析**：Johnson 算法先用 Bellman-Ford 计算 h(v)（从虚拟源点 0 到 v 的最短路径），然后用 ŵ(u, v) = w(u, v) + h(u) − h(v) 对边重新加权。由三角不等式 h(v) ≤ h(u) + w(u, v) 可得 ŵ(u, v) ≥ 0。之后即可对每个顶点运行 Dijkstra。总时间复杂度 O(mn + n² log n)，在稀疏图中优于 Floyd-Warshall 的 Θ(n³)。

---

**5.（Shortest Path Properties, Slide13）** 关于最短路径的性质，以下哪个说法是 FALSE？

- (A) 最短路径的子路径也是最短路径（最优子结构）
- (B) 对任意三个顶点 v₁, v₂, v₃，有 d(v₁, v₂) ≤ d(v₁, v₃) + d(v₃, v₂)（三角不等式）
- (C) 若图中所有边权都加上同一个正常数，则所有最短路径保持不变
- (D) 若图中存在负权环，则某些最短路径可能不存在

**答案**：C **解析**：讲义明确指出：Adding a constant to every edge weight can fail。因为路径越长，加的常数额外权重越大，这会改变最短路径的选择。例如 s→t 直接路径 2（+5 → 7）和 s→u→v→t 路径 1+1+1=3（+5+5+5 → 18），加常数前后者更短，加常数后前者更短。A、B、D 均为讲义中的标准性质。

---

## 二、问答题

### 1. 最短路径算法综合（20'）

#### （1）Dijkstra 算法正确性证明（8'）

证明 Dijkstra 算法的正确性：当顶点 u 被加入集合 S 时，必有 d[u] = d(s, u)（即从源点 s 到 u 的最短路径长度）。

**答案**：

**证明**（反证法）：

假设 u 是第一个加入 S 时 d[u] ≠ d(s, u) 的顶点。由初始化 d[s] = 0 = d(s, s)，故 u ≠ s。

考虑从 s 到 u 的一条最短路径 P。设 x 是 P 上最后一个属于 S 的顶点（s ∈ S，u ∉ S，所以 x 存在），y 是 P 上 x 之后的下一个顶点（即 y 是 P 上第一个不属于 S 的顶点）。

```
s ──→ ... ──→ x ──→ y ──→ ... ──→ u
     ∈ S     ∈ S    ∉ S          ∉ S
```

由于 x 在 u 之前加入 S，由归纳假设 d[x] = d(s, x)。当 x 加入 S 时，边 (x, y) 被松弛，因此：
d[y] = d[x] + w(x, y) = d(s, x) + w(x, y) = d(s, y)  （最短路径的子路径也是最短路径）

由于 y 在从 s 到 u 的最短路径上，且边权非负，有 d(s, y) ≤ d(s, u)。又因 u 是当前 EXTRACT-MIN 选出的顶点（d[u] 最小），所以 d[u] ≤ d[y]。

综上：d(s, u) ≤ d[u] ≤ d[y] = d(s, y) ≤ d(s, u)

因此 d[u] = d(s, u)，与假设矛盾。证毕。

---

#### （2）Bellman-Ford 算法设计（7'）

设计 Bellman-Ford 算法求解带负权边的单源最短路径问题，并分析其时间复杂度。要求：
- 写出伪代码
- 分析 O(mn) 时间复杂度
- 说明如何检测负权环

**答案**：

```
算法 Bellman-Ford
输入：有向图 G = (V, E)，源点 s，边权 w
输出：dist[v] = s 到 v 的最短路径长度；或报告存在负权环

// 初始化
1 for each v ∈ V:
2     dist[v] = ∞
3     pred[v] = NIL
4 dist[s] = 0

// 执行 n-1 轮松弛
5 for i = 1 to n-1:
6     for each (u, v) ∈ E:
7         if dist[v] > dist[u] + w(u, v):
8             dist[v] = dist[u] + w(u, v)
9             pred[v] = u

// 检测负权环
10 for each (u, v) ∈ E:
11     if dist[v] > dist[u] + w(u, v):
12         return "存在负权环"

13 return dist[]
```

**时间复杂度分析**：
- 初始化：O(n)
- 外层循环 n-1 次，内层遍历所有 m 条边：O(mn)
- 负权环检测：O(m)
- 总时间复杂度：O(mn)

**负权环检测原理**：无负权环时，最短路径最多使用 n-1 条边（简单路径），n-1 轮松弛后 dist 应已收敛。若第 n 轮还能松弛，则说明存在负权环（由鸽笼原理，n 条边的路径必含环，且环权为负）。

**DP 定义**：OPT(i, v) 表示从 s 到 v 最多使用 i 条边的最短路径长度。
- OPT(0, s) = 0, OPT(0, v) = ∞ (v ≠ s)
- OPT(i, v) = min{OPT(i-1, v), min_{(u,v)∈E} {OPT(i-1, u) + w(u, v)}}

---

#### （3）Johnson 算法与 Floyd-Warshall 对比（5'）

Johnson 算法和 Floyd-Warshall 算法都能求解所有顶点对的最短路径问题。请从以下方面对比两种算法：

| 对比维度 | Johnson 算法 | Floyd-Warshall 算法 |
|---------|-------------|-------------------|
| 能否处理负权边 | | |
| 能否检测负权环 | | |
| 时间复杂度 | | |
| 适用场景 | | |

**答案**：

| 对比维度 | Johnson 算法 | Floyd-Warshall 算法 |
|---------|-------------|-------------------|
| 能否处理负权边 | 能（通过 reweighting） | 能 |
| 能否检测负权环 | 能（Bellman-Ford 阶段） | 能（检查对角线是否有负值） |
| 时间复杂度 | O(mn + n² log n) | Θ(n³) |
| 适用场景 | 稀疏图（m = o(n²/log n) 时优于 Floyd-Warshall） | 稠密图；实现简单 |

**Johnson 算法步骤**：
1. 添加新节点 0，用 0 权边连接到所有节点
2. 从 0 运行 Bellman-Ford 得到 h(v)（若检测到负权环则终止）
3. 重加权 ŵ(u, v) = w(u, v) + h(u) − h(v) ≥ 0
4. 对每个顶点运行 Dijkstra
5. 将结果还原：d(i, j) = d̂(i, j) − h(i) + h(j)

**Floyd-Warshall DP 定义**：c⁽ᵏ⁾ᵢⱼ 为从 i 到 j、中间节点仅允许使用 {1,...,k} 的最短路径长度，递推式 c⁽ᵏ⁾ᵢⱼ = min{c⁽ᵏ⁻¹⁾ᵢⱼ, c⁽ᵏ⁻¹⁾ᵢₖ + c⁽ᵏ⁻¹⁾ₖⱼ}。

---

### 2. 最短路径应用与证明（12'）

#### （1）重加权定理证明（6'）

证明以下定理：给定每个顶点 v 的标记 h(v)，将每条边 (u, v) 重加权为 ŵ(u, v) = w(u, v) + h(u) − h(v)，则任意两顶点之间的所有路径被重加权后增加相同的偏移量，因此最短路径保持不变。

**答案**：

**证明**：设 P 是从 v₁ 到 vₖ 的任意一条路径：v₁ → v₂ → ... → vₖ。

重加权后的路径权值为：
```
ŵ(P) = Σᵢ₌₁ᵏ⁻¹ ŵ(vᵢ, vᵢ₊₁)
     = Σᵢ₌₁ᵏ⁻¹ [w(vᵢ, vᵢ₊₁) + h(vᵢ) - h(vᵢ₊₁)]
     = Σᵢ₌₁ᵏ⁻¹ w(vᵢ, vᵢ₊₁) + h(v₁) - h(vₖ)
     = w(P) + h(v₁) - h(vₖ)
```

因此，对于任意两条从 i 到 j 的路径 P₁ 和 P₂：
```
ŵ(P₁) - ŵ(P₂) = [w(P₁) + h(i) - h(j)] - [w(P₂) + h(i) - h(j)] = w(P₁) - w(P₂)
```

两条路径的权值之差在重加权前后保持不变，故最短路径不变。

若选择 h(v) 为从虚拟源点 0 到 v 的最短路径长度，则由三角不等式 h(v) ≤ h(u) + w(u, v)，可得 ŵ(u, v) = w(u, v) + h(u) - h(v) ≥ 0。

---

#### （2）矩阵乘法方法求最短路径（6'）

考虑用矩阵乘法（min-plus 乘法）求所有顶点对的最短路径。定义 D⁽ᵐ⁾ᵢⱼ 为从 i 到 j 最多使用 m 条边的最短路径长度。

证明递推关系：D⁽ᵐ⁾ᵢⱼ = minₖ {D⁽ᵐ⁻¹⁾ᵢₖ + aₖⱼ}，其中 aₖⱼ 是邻接矩阵中边 (k, j) 的权值。

并说明如何使用重复平方法（repeated squaring）将时间复杂度从 Θ(n⁴) 降低到 Θ(n³ log n)。

**答案**：

**递推关系证明**：从 i 到 j 最多使用 m 条边的路径中，考虑最后一条边 (k, j)。路径由两部分组成：
- 从 i 到 k 最多使用 m-1 条边的路径
- 最后一条边 (k, j)

因此 D⁽ᵐ⁾ᵢⱼ = min{D⁽ᵐ⁻¹⁾ᵢⱼ（不使用第 m 条边）, minₖ {D⁽ᵐ⁻¹⁾ᵢₖ + aₖⱼ}（使用第 m 条边且最后一条边为 (k, j)）}。

当图中无负权环时，最短路径最多使用 n-1 条边，因此 D⁽ⁿ⁻¹⁾ = D⁽ⁿ⁾ = D⁽ⁿ⁺¹⁾ = ...，收敛到最终结果。

**重复平方法**：

直接计算 D⁽¹⁾, D⁽²⁾, ..., D⁽ⁿ⁻¹⁾ 需要 n-1 次 min-plus 乘法，每次 O(n³)，共 O(n⁴)。

由于 min-plus 乘法满足结合律，可以利用重复平方：
- 计算 A² = A ⊗ A（1 次乘法）
- 计算 A⁴ = A² ⊗ A²（1 次乘法）
- 依次计算 A², A⁴, A⁸, ..., A^{2^{⌈log₂(n-1)⌉}}

共需要 O(log n) 次乘法，每次 O(n³)，总时间 Θ(n³ log n)。

注意：A^{n-1} = A^{n} = A^{n+1} = ...，当图中无负权环时成立。

---

## 答案速查

### 选择题

| 题号 | 答案 | 考点 |
|------|------|------|
| 1 | C | Dijkstra 贪心选择与负权问题 |
| 2 | B | Bellman-Ford 负权环检测原理 |
| 3 | B | Floyd-Warshall 循环顺序 |
| 4 | B | Johnson 算法 reweighting 目的 |
| 5 | C | 最短路径性质（选 FALSE） |
