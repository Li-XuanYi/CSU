# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：DFS / BFS / Graph Decomposition（Slide12）

---

## 一、选择题

**1.（DFS Edge Types, Slide12）** 在有向图 G 上执行深度优先搜索（DFS），得到以下 pre/post 区间。对于一条边 (u, v)，其 pre/post 区间分别为 PRE(u)=2, POST(u)=7, PRE(v)=3, POST(v)=6。请问边 (u, v) 属于哪种类型？

- (A) Tree edge
- (B) Back edge
- (C) 该边不存在
- (D) Cross edge

**答案**：A **解析**：pre/post 区间为 [2,7] 和 [3,6]，前者包含后者，且 u 的 pre 较小（先发现），说明 u 是 v 的祖先，v 是 u 的后代。由于 v 的区间完全包含在 u 内，且 u 先被访问，这是 tree edge 或 forward edge 的特征。

---

**2.（Cycle Detection, Slide12）** 关于有向图的环检测，以下哪个说法是正确的？

- (A) 若 DFS 中出现了 cross edge，则图一定存在环
- (B) 若 DFS 中出现了 back edge，则图一定存在环
- (C) 若 DFS 中出现了 forward edge，则图一定存在环
- (D) 有向图无环当且仅当 DFS 中不存在 cross edge

**答案**：B **解析**：根据讲义 Lemma：A directed graph has a cycle if and only if its depth-first search reveals a back edge。back edge 指向祖先，构成环；cross edge 和 forward edge 在有环和无环图中都可能出现（DAG 中可以有 cross edge 和 forward edge）。

---

**3.（Topological Sort, Slide12）** 对以下 DAG 执行 DFS，得到的 POST 编号从大到小排序为：j, i, h, g, f, e, d, c, b, a。则以下哪个是合法的拓扑排序？

- (A) a, b, c, d, e, f, g, h, i, j
- (B) j, i, h, g, f, e, d, c, b, a
- (C) a, b, f, j, c, d, e, i, h, g
- (D) j, f, i, h, g, e, d, c, b, a

**答案**：B **解析**：DAG 的线性化（拓扑排序）可以通过按 POST 编号递减顺序排列顶点得到。讲义指出：In a dag, every edge leads to a vertex with a lower POST number。因此按 POST 降序排列即得拓扑排序。

---

**4.（SCC Meta-Graph, Slide12）** 关于有向图的强连通分量（SCC）及其 meta-graph，以下哪个说法是 FALSE？

- (A) 每个有向图都可以看作其 SCC 的 DAG（meta-graph）
- (B) 在 GR（反向图）上执行 DFS 时，POST 编号最大的节点一定位于 G 的 sink SCC 中
- (C) DFS 中 POST 编号最小的节点一定位于 sink SCC 中
- (D) 对 GR 执行 DFS 得到 POST 编号，再按 POST 递减顺序在 G 上执行 DFS 可线性时间求出所有 SCC

**答案**：C **解析**：选项 C 是 FALSE。讲义第 21 页给出了明确的反例：考虑一个 4 个 SCC 的有向图，其中各节点的探索顺序为 1→2→3→4→5→6→7，对应的 pre/post 区间分别为 (1,10)、(2,5)、(3,4)、(6,9)、(7,8)、(11,14)、(12,13)。该图中 POST 编号最小的节点是 3（POST=4），但 3 所在的 SCC 并不是 sink SCC——sink SCC 反而是 POST 编号为 8 的节点所在的绿色 SCC。因此最小 POST 编号未必在 sink SCC 中。

A 正确：meta-graph 的顶点为 SCC，若 meta-graph 有环则相邻 SCC 可互通，应合并为同一个 SCC。
B 正确：GR 的 source SCC 对应 G 的 sink SCC，而 POST 最大的节点必在 GR 的 source SCC 中（引理 B）。
D 正确：这正是 Kosaraju 算法的步骤——第一趟 DFS 在 GR 上按 POST 降序得顺序，第二趟在 G 上按此顺序 DFS 逐个输出 SCC。

---

**5.（BFS Correctness, Slide12）** BFS 算法中，队列 Q 在某一时刻的性质是以下哪一项？

- (A) Q 中包含所有距离 ≤ d 的节点
- (B) Q 中包含所有距离 ≥ d 的节点
- (C) Q 中恰好包含所有距离为 d 的节点
- (D) Q 中包含距离为 d 和 d+1 的节点

**答案**：C **解析**：根据讲义 Lemma：For each d = 0, 1, 2, ... there is a moment at which the queue contains exactly the nodes at distance d。BFS 逐层遍历，队列中始终维护当前层的所有节点。

---

## 二、问答题

### 1. Strongly Connected Components 算法设计（17'）

#### （1）SCC 概念与性质（5'）

定义有向图的强连通分量（Strongly Connected Component），并证明以下引理：每个有向图都可以看作其 SCC 的 DAG（meta-graph 无环）。

**答案**：

**定义**：有向图 G 中，两个节点 u, v 是强连通的当且仅当存在从 u 到 v 的路径且存在从 v 到 u 的路径。强连通关系是 V 上的等价关系，每个等价类称为一个强连通分量（SCC）。

**引理证明**：设 meta-graph 的节点为 G 的 SCC。若 meta-graph 中存在环 C₁ → C₂ → ... → Cₖ → C₁，则对于任意 u ∈ Cᵢ, v ∈ Cⱼ，存在 u ⇝ v 和 v ⇝ u 的路径，因此 Cᵢ 和 Cⱼ 应属于同一个 SCC，与它们是不相交 SCC 矛盾。故 meta-graph 中无环，是 DAG。

---

#### （2）POST 编号与 SCC 的关系（6'）

证明以下两个引理：

**引理 A**：若 C 和 C' 是不同 SCC，且存在从 C 到 C' 的边，则 C 中最高的 POST 编号大于 C' 中最高的 POST 编号。

**引理 B**：DFS 中 POST 编号最大的节点一定位于 source SCC 中。

**答案**：

**引理 A 证明**：设边 (u, v) 满足 u ∈ C, v ∈ C'。在 DFS 中，有两种情况：
- 若 C 先被发现（即 C 中某节点先被访问）：则 DFS 会遍历 C 中所有可达节点，包括 C'（因为存在边从 C 到 C'）。C 中节点将在 C' 中节点之后完成（POST），故 C 中最高 POST > C' 中最高 POST。
- 若 C' 先被发现：由于不存在从 C' 到 C 的路径（否则 C 和 C' 将合并为同一 SCC），DFS 将在 C' 完全结束后才开始探索 C，故 C' 的 POST 区间完全在 C 的 POST 区间之前，C 中最高 POST > C' 中最高 POST。

**引理 B 证明**：由引理 A，在 meta-graph DAG 中，POST 编号沿边方向递减。因此 POST 编号最大的节点所在的 SCC 没有入边（否则入边来自的 SCC 会有更大的 POST 编号），即为 source SCC。

---

#### （3）线性时间 SCC 算法（6'）

利用以上性质，设计一个线性时间 O(|V| + |E|) 的算法找出有向图 G 的所有 SCC。请写出伪代码并分析时间复杂度。

**答案**：

**算法思路**（Kosaraju 算法）：

关键观察：要在 G 中找到 sink SCC，可以在 GR（反向图）上找 source SCC（因为 GR 的 source SCC 就是 G 的 sink SCC）。而由引理 B，在 GR 上执行 DFS 时 POST 编号最大的节点一定位于 GR 的 source SCC（即 G 的 sink SCC）。

找到 sink SCC 后删除之，剩余图中 POST 编号最大的节点仍在新的 sink SCC 中。

```
算法 Kosaraju SCC
输入：有向图 G = (V, E)
输出：每个节点 v 的 SCC 编号 scc[v]

// 第一趟：在 GR 上执行 DFS，记录 POST 编号
1 在 GR = (V, ER) 上执行 DFS，计算每个节点的 POST 编号
2 将节点按 POST 编号从大到小排序存入 order[]

// 第二趟：在 G 上按 order 顺序执行 DFS
3 初始化 visited[v] = false (∀v ∈ V)
4 scc_id = 0
5 for each v in order[]:
6     if not visited[v]:
7         EXPLORE_SCC(G, v, scc_id)
8         scc_id = scc_id + 1

// EXPLORE 子过程
EXPLORE_SCC(G, v, scc_id):
1 visited[v] = true
2 scc[v] = scc_id
3 for each (v, u) ∈ E:
4     if not visited[u]:
5         EXPLORE_SCC(G, u, scc_id)
```

**时间复杂度分析**：
- 第一趟 DFS：O(|V| + |E|)
- 排序：O(|V| log |V|)，可优化为 O(|V|)（按 POST 递减顺序直接记录）
- 第二趟 DFS：O(|V| + |E|)
- 总时间复杂度：O(|V| + |E|)，即线性时间

**正确性**：第一趟在 GR 上找 POST 编号最大的节点，定位 G 的 sink SCC；第二趟在 G 上从该节点开始 EXPLORE，恰好找出该 SCC。删除后重复此过程。

---

### 2. BFS 与 DFS 对比分析（15'）

#### （1）算法描述（5'）

分别给出 BFS 和 DFS 的伪代码，并分析它们的时间复杂度。

**答案**：

```
// DFS
DFS(G):
1 for each v ∈ V:
2     visited[v] = false
3 for each v ∈ V:
4     if not visited[v]:
5         EXPLORE(G, v)

EXPLORE(G, v):
1 visited[v] = true
2 PREVISIT(v)
3 for each (v, u) ∈ E:
4     if not visited[u]:
5         EXPLORE(G, u)
6 POSTVISIT(v)
```

```
// BFS
BFS(G, s):
1 for each u ∈ V:
2     dist[u] = ∞
3 dist[s] = 0
4 Q = [s]    // 队列
5 while Q 不为空:
6     u = DEQUEUE(Q)
7     for each (u, v) ∈ E:
8         if dist[v] = ∞:
9             ENQUEUE(Q, v)
10            dist[v] = dist[u] + 1
```

**时间复杂度**：两者均为 O(|V| + |E|)。DFS 中每个顶点被 EXPLORE 一次，每条边被检查两次；BFS 中每个顶点入队一次，每条边被检查一次。

---

#### （2）pre/post 区间性质（5'）

证明以下引理：在无向图的 DFS 中，对于任意两个节点 u, v，区间 [PRE(u), POST(u)] 和 [PRE(v), POST(v)] 要么不相交，要么一个完全包含另一个。

**答案**：

**证明**：考虑 DFS 的执行过程，不失一般性设 PRE(u) < PRE(v)，即 u 先于 v 被发现。

- 若 v 在 u 的 EXPLORE 过程中被发现（即 v 是 u 的后代），则 v 的 PRE 在 u 的 PRE 之后，且 v 的 POST 在 u 的 POST 之前（因为 u 要等所有后代都完成才 POST），故 [PRE(v), POST(v)] ⊆ [PRE(u), POST(u)]。

- 若 v 不在 u 的 EXPLORE 过程中被发现，则 u 的 EXPLORE 先完全结束（POST(u) 完成），之后才在某次外层循环中发现 v，此时 [PRE(u), POST(u)] 整个在 [PRE(v), POST(v)] 之前，区间不相交。

因此两种区间关系必居其一。

---

#### （3）BFS 的正确性证明（5'）

证明 BFS 算法能正确计算出从源点 s 到所有可达节点的最短距离（边权均为 1）。

**答案**：

**证明**：用数学归纳法证明以下不变性：对于每个 d = 0, 1, 2, ...，存在某一时刻满足：
1. 所有距离 s ≤ d 的节点的 dist 值已被正确设置；
2. 所有其他节点的 dist 值仍为 ∞；
3. 队列 Q 中恰好包含所有距离为 d 的节点。

**基础情况** d = 0：初始化时 dist[s] = 0，其他为 ∞，Q = [s]，成立。

**归纳步骤**：假设对 d 成立。队列中恰有距离为 d 的节点。对每个距离为 d 的节点 u 出队，检查其邻接边 (u, v)：
- 若 dist[v] = ∞，说明 v 尚未被访问且距离为 d+1（因为 u 距离为 d），设置 dist[v] = d+1 并入队。
- 处理完所有距离 d 的节点后，队列中恰好包含所有距离为 d+1 的节点，且所有 ≤ d+1 的节点距离已正确设置。

由归纳法，对所有 d 成立。当 BFS 终止时，所有可达节点的 dist 值已被正确设置为最短距离。

---

## 答案速查

### 选择题

| 题号 | 答案 | 考点 |
|------|------|------|
| 1 | A | DFS 边类型判断（pre/post 区间） |
| 2 | B | 有向图环检测与 back edge 关系 |
| 3 | B | DAG 拓扑排序（POST 降序） |
| 4 | C | SCC 性质与 POST 编号（选 FALSE） |
| 5 | C | BFS 队列性质 |
