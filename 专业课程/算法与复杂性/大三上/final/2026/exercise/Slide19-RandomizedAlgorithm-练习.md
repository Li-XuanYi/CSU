# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：Randomized Algorithm（Slide19）

---

## 一、选择题

**1.（Monte Carlo vs Las Vegas, Slide19）** 关于 Monte Carlo 算法和 Las Vegas 算法，以下说法正确的是？

- (A) Monte Carlo 算法总是给出正确结果，但运行时间是随机的
- (B) Las Vegas 算法的运行时间是确定的，但可能给出错误结果
- (C) Monte Carlo 算法可能以较小概率出错，运行时间确定；Las Vegas 算法始终正确，运行时间为随机变量
- (D) Las Vegas 算法可以在多项式时间内解决任何 NP 问题

**答案**：C **解析**：根据 Slide19 的定义：Monte Carlo 算法运行时间确定，但可能以较小概率给出错误结果（如多项式恒等性验证）；Las Vegas 算法始终给出正确结果，运行时间为随机变量（如随机 QuickSort）。A 和 B 恰好说反；D 没有理论依据。

---

**2.（Max 3-SAT 随机赋值期望, Slide19）** 对于一个有 $k$ 个子句的 3-SAT 公式，每个变量独立以 $1/2$ 概率取 True/False，则期望满足的子句数的为？

- (A) $k/2$
- (B) $3k/4$
- (C) $7k/8$
- (D) $k$

**答案**：C **解析**：每个子句含 3 个文字，当且仅当 3 个文字全为 False 时子句不满足（概率 $(1/2)^3 = 1/8$），因此子句被满足的概率为 $1 - 1/8 = 7/8$。由期望线性性，期望满足子句数为 $7k/8$。

---

**3.（Universal Hashing 定义, Slide19）** 关于 Universal Hashing 的族 $\mathcal{H}$，以下哪个条件刻画了其 universality？

- (A) 对任意 $u, v \in U$ 和随机选取的 $h \in \mathcal{H}$，$h(u) \neq h(v)$ 的概率为 1
- (B) 对任意 $u, v \in U$ 和随机选取的 $h \in \mathcal{H}$，$\Pr[h(u) = h(v)] \leq 1/n$
- (C) 对所有 $u \in U$，$h(u)$ 服从均匀分布
- (D) 对任意 $u \neq v \in U$，$\Pr[h(u) = h(v)] = 0$

**答案**：B **解析**：Universal Hashing 的定义（Carter-Wegman）要求对任意两个不同元素 $u, v$，随机选取的哈希函数 $h$ 使它们冲突的概率不超过 $1/n$（$n$ 为哈希表大小）。这保证了期望冲突数不超过 1。A 和 D 要求绝对无冲突不可能实现；C 虽然理想但单一函数无法对所有元素均匀。

---

**4.（随机 Max Cut 期望值, Slide19）** 对于 $n$ 个顶点 $m$ 条边的图，若将每个顶点独立以 $1/2$ 概率随机分配到 $U$ 或 $V \setminus U$，则期望割大小是多少？

- (A) $m/4$
- (B) $m/2$
- (C) $3m/4$
- (D) $m$

**答案**：B **解析**：对任意边 $uv$，它属于割当且仅当 $u$ 和 $v$ 被分到不同侧，概率为 $\Pr[u \in U, v \notin U] + \Pr[u \notin U, v \in U] = 1/4 + 1/4 = 1/2$。由期望线性性，$E[|\delta(U)|] = \sum_{uv \in E} 1/2 = m/2$。

---

**5.（Chernoff Bound 应用, Slide19）** 有 $n$ 个独立作业均匀随机分配给 $n$ 台机器，令 $X_i$ 为第 $i$ 台机器的作业数。以下说法正确的是？

- (A) $E[X_i] = n$，且任意一台机器负载超过 $c$ 的概率随 $c$ 增大以指数级衰减
- (B) $E[X_i] = 1$，任意一台机器负载超过 $e \frac{\log n}{\log\log n}$ 的概率不超过 $1/n^2$
- (C) $E[X_i] = n$，根据大数定律所有机器负载均严格为 1
- (D) $E[X_i] = 1$，根据 Markov 不等式可知所有机器负载均 $O(1)$

**答案**：B **解析**：$E[X_i] = n \cdot 1/n = 1$。应用 Chernoff 界：$\Pr[X_i > c] < e^{c-1}/c^c$。令 $c = e x$ 满足 $x^x = n$，则 $\Pr[X_i > c] < (e/c)^c = (1/x)^{e x} < 1/n^2$。Union bound 得最重负载不超过 $e\frac{\log n}{\log\log n}$ 的概率 $\geq 1 - 1/n$。A 错误（期望为 1）；C 错误（不会严格为 1）；D 错误（Markov 只能给出 $O(n)$ 的弱界）。

---

## 二、问答题

### 1. 多项式恒等性验证与 Johnson 算法（15 分）

#### （1）多项式恒等性验证（Monte Carlo, Slide19）（5'）

给定两个 $d$ 次多项式 $F(x)$ 和 $G(x)$。设计一个 Monte Carlo 算法来验证 $F(x) \equiv G(x)$，要求算法以概率 $\geq 1 - 1/N$ 正确（$N$ 为给定参数）。

**答案**：

**算法**：

1. 从 $\{1, 2, \ldots, Nd\}$ 中均匀随机选择一个整数 $r$
2. 计算 $F(r)$ 和 $G(r)$
3. 若 $F(r) = G(r)$，返回"它们相等"；否则返回"它们不相等"

**分析**：

- 若 $F(x) \equiv G(x)$，则对任意 $r$ 都有 $F(r) = G(r)$，算法以概率 1 正确。
- 若 $F(x) \not\equiv G(x)$，令 $Q(x) = F(x) - G(x)$ 是次数不超过 $d$ 的非零多项式。$Q(x)$ 最多有 $d$ 个根。算法出错当且仅当选择的 $r$ 恰好是 $Q(x)$ 的一个根。

$$
\Pr[\text{错误}] = \Pr[Q(r) = 0] \leq \frac{d}{Nd} = \frac{1}{N}
$$

因此算法以 $\geq 1 - 1/N$ 的概率正确。这是典型的 Monte Carlo 算法（单侧错误，运行时间确定）。$\square$

**注**：若要进一步提高成功概率至 $1 - \epsilon$，有两种方式：

1. **增大 $N$**：单次试验中取 $N = \lceil 1/\epsilon \rceil$，则 $\Pr[\text{错误}] \leq 1/N \leq \epsilon$。
2. **固定 $N$ 重复试验**：设固定 $N$（如 $N = 100$），独立重复 $t$ 次。若任一次返回"不相等"则直接判定不等（此结果绝对正确）；若全部 $t$ 次均返回"相等"，则出错概率降至 $(1/N)^t$。要使出错概率 $\leq \epsilon$，取 $t \geq \frac{\log(1/\epsilon)}{\log N}$ 即可。

---

#### （2）Johnson 算法与 Max 3-SAT（Randomized Approximation, Slide19）（5'）

Johnson 算法反复生成随机真值赋值，直到找到满足至少 $7k/8$ 个子句的赋值为止。证明：

（a）单次随机赋值满足 $\geq 7k/8$ 个子句的概率至少为 $1/(8k)$。

（b）Johnson 算法是 $7/8$-近似算法。

**答案**：

**（a）** 设 $p_i$ 为恰好满足 $i$ 个子句的概率，$p$ 为满足 $\geq 7k/8$ 子句的概率。

由期望公式：
$$
\frac{7}{8}k = E[X] = \sum_{i=0}^{k} i p_i = \sum_{i < 7k/8} i p_i + \sum_{i \geq 7k/8} i p_i
$$

$$
\leq \left(\frac{7k - 1}{8}\right) \sum_{i < 7k/8} p_i + k \sum_{i \geq 7k/8} p_i
\leq \left(\frac{7k - 1}{8}\right) \cdot 1 + k \cdot p
$$

整理得：
$$
\frac{7}{8}k \leq \frac{7k - 1}{8} + kp \quad \Rightarrow \quad kp \geq \frac{1}{8} \quad \Rightarrow \quad p \geq \frac{1}{8k}
$$

$\square$

**（b）** 由（a），每次试验以概率 $\geq 1/(8k)$ 成功。成功所需试验次数服从几何分布，期望值为 $\leq 8k$。

对于有 $k$ 个子句的 3-SAT 实例，最优解最多满足 $k$ 个子句。Johnson 算法找到的赋值满足 $\geq 7k/8$ 个子句，故近似比为：
$$
\frac{OPT}{|A_g|} \leq \frac{k}{7k/8} = \frac{8}{7}
$$

即 Johnson 算法是 $7/8$-近似算法。$\square$

---

#### （3）概率方法（Probabilistic Method, Slide19）（5'）

利用概率方法证明：对任意 3-SAT 实例，存在一个真值赋值满足至少 $7/8$ 的子句。

**答案**：

**存在性证明**：

设 $X$ 为随机赋值满足的子句数。由前述结论，$E[X] = 7k/8$。

由于随机变量以正概率不超过其期望...更精确地说，由期望的定义 $E[X] = \sum_{i} i \cdot \Pr[X = i]$，期望值 $7k/8$ 是各可能取值的加权平均，因此必然存在某个赋值使得 $X \geq E[X] = 7k/8$。

否则若对所有赋值都有 $X < 7k/8$，则 $E[X] < 7k/8$，矛盾。

因此存在一个真值赋值满足至少 $\lceil 7k/8 \rceil$ 个子句。$\square$

---

### 2. Universal Hashing 的设计与分析（15 分）

#### （1）Universal Hashing 期望碰撞分析（Universal Hashing, Slide19）（5'）

设 $\mathcal{H}$ 是 universal 哈希函数族。从 $\mathcal{H}$ 中均匀随机选取 $h$，对任意 $u \in U$ 和任意子集 $S \subseteq U$ 且 $|S| \leq n$（$n$ 为哈希表大小），证明与 $u$ 发生碰撞的 $S$ 中元素个数的期望不超过 1。

**答案**：

**证明**：对每个 $s \in S$，定义指示变量：
$$
X_s = \begin{cases}
1 & \text{if } h(s) = h(u) \\
0 & \text{otherwise}
\end{cases}
$$

设 $X = \sum_{s \in S} X_s$ 为与 $u$ 碰撞的总次数。由期望线性性：
$$
E[X] = \sum_{s \in S} E[X_s] = \sum_{s \in S} \Pr[h(s) = h(u)]
$$

由 universal hashing 的定义，对任意 $s \neq u$，$\Pr[h(s) = h(u)] \leq 1/n$。因此：
$$
E[X] \leq \sum_{s \in S} \frac{1}{n} \leq |S| \cdot \frac{1}{n} \leq n \cdot \frac{1}{n} = 1
$$

即与 $u$ 碰撞的期望元素数不超过 1。$\square$

---

#### （2）构造 Universal Family（Universal Hashing, Slide19）（5'）

设 $U$ 为元素全集，$p$ 为满足 $p \geq n$ 的素数。将每个元素 $x \in U$ 编码为一个 $r$ 位 $p$ 进制数 $x = (x_1, x_2, \ldots, x_r)$。构造哈希函数族：
$$
\mathcal{H} = \left\{h_a : h_a(x) = \sum_{i=1}^r a_i x_i \bmod p \;\middle|\; a = (a_1, \ldots, a_r),\; 0 \leq a_i < p\right\}
$$

证明 $\mathcal{H}$ 是 universal 的。

**答案**：

**证明**：取两个不同的元素 $x = (x_1, \ldots, x_r)$ 和 $y = (y_1, \ldots, y_r)$，存在某下标 $j$ 使得 $x_j \neq y_j$。

$h_a(x) = h_a(y)$ 当且仅当：
$$
\sum_{i=1}^r a_i x_i \equiv \sum_{i=1}^r a_i y_i \pmod{p}
\iff a_j(y_j - x_j) \equiv \sum_{i \neq j} a_i(x_i - y_i) \pmod{p}
$$

考虑随机选择 $a$ 的过程：先独立均匀地选择所有 $a_i$（$i \neq j$），最后选择 $a_j$。固定 $i \neq j$ 的 $a_i$ 后，右边是确定的常数 $m$：
$$
a_j \cdot (y_j - x_j) \equiv m \pmod{p}
$$

由于 $p$ 是素数且 $y_j - x_j \neq 0 \pmod{p}$，上式在模 $p$ 意义下有唯一解。而 $a_j$ 均匀取自 $\{0, 1, \ldots, p-1\}$，因此解恰好被选中的概率为：
$$
\Pr[h_a(x) = h_a(y)] = \frac{1}{p} \leq \frac{1}{n}
$$

故 $\mathcal{H}$ 是 universal family。$\square$

---

#### （3）对抗攻击与随机化的作用（Universal Hashing, Slide19）（5'）

（a）解释为什么确定性哈希函数容易受到 Denial-of-Service 攻击，而 Universal Hashing 可以缓解这一问题。

（b）在 Universal Hashing 的场景中，"对手知道算法但不知道随机选择"的假设为什么重要？

**答案**：

**（a）确定性哈希函数的脆弱性**：

若哈希函数是固定的（如 Java 的 `hashCode()`），攻击者可以分析该函数，构造大量哈希值相同的键，将它们全部插入同一个桶中，使哈希表退化成长度为 $O(m)$ 的链表，查询时间从 $O(1)$ 退化到 $O(m)$。这在 Web 服务中可造成 DoS 攻击（如 [Crosby-Wallach 2003] 对 Bro 服务器和 Perl 5.8.0 的攻击）。

**Universal Hashing 的缓解**：

Universal Hashing 在程序启动时从 $\mathcal{H}$ 中随机选取 $h$。攻击者不知道具体使用了哪个 $h$，因此无法构造使大量元素碰撞的输入。对于任意固定的输入集 $S$，期望冲突数 $\leq 1$，因此链长期望为 $O(1)$。

**（b）假设的重要性**：

"对手知道算法但不知道随机选择"是密码学中 Kerckhoffs 原则的体现——系统的安全性不应依赖于算法的保密性，而应依赖于密钥（此处是随机种子）的保密性。若对手也知道了随机选择（例如通过侧信道泄露），则他可以针对性构造最坏情况输入，使 Universal Hashing 退化为确定性哈希。因此随机种子的保密性和不可预测性是 Universal Hashing 安全性的关键前提。

---

### 3. 随机 Max Cut 与集中不等式（15 分）

#### （1）随机 Max Cut 的期望与概率分析（Randomized Max Cut, Slide19）（8'）

对于 $m$ 条边的图 $G$，考虑随机 Max Cut 算法：每个顶点独立以 $1/2$ 概率加入 $U$。

（a）证明期望割大小为 $m/2$，且算法是 $1/2$-近似算法。

（b）利用 Reverse Markov 不等式，证明对任意 $\epsilon \in [0, 1/2]$，算法以至少 $\epsilon$ 的概率输出一个大小 $> (1/2 - \epsilon)OPT$ 的割。

（c）如何通过多次重复来提高成功概率？

**答案**：

**（a）** 对每条边 $uv$，定义指示变量 $X_{uv} = 1$ 当 $u$ 和 $v$ 在不同侧。则：
$$
\Pr[X_{uv} = 1] = \Pr[u \in U, v \notin U] + \Pr[u \notin U, v \in U] = \frac{1}{4} + \frac{1}{4} = \frac{1}{2}
$$

由期望线性性，$E[|\delta(U)|] = \sum_{uv \in E} 1/2 = m/2$。由于 $OPT \leq m$，有 $E[|\delta(U)|] \geq OPT/2$。

特别地，期望值 $\geq OPT/2$ 意味着存在某个随机选择使割大小 $\geq OPT/2$。随机化算法只需一次尝试的期望割大小已满足 $1/2$-近似。$\square$

**（b）** 令 $X = |\delta(U)|$，$B = |E| = m$。显然 $X \leq B$。Reverse Markov 不等式：
$$
\Pr[X \leq a] = \frac{E[B - X]}{B - a}
$$

取 $a = (1/2 - \epsilon)m$，已知 $E[X] = m/2$：
$$
\Pr\left[X \leq \left(\frac{1}{2} - \epsilon\right)m\right] = \frac{m - m/2}{(1/2 + \epsilon)m} = \frac{1}{1 + 2\epsilon} \leq 1 - \epsilon
$$

因此 $\Pr[X > (1/2 - \epsilon)m] \geq \epsilon$。由于 $m \geq OPT$：
$$
\Pr[X > (1/2 - \epsilon)OPT] \geq \epsilon
$$

即算法以至少 $\epsilon$ 的概率输出一个接近 $OPT/2$ 的割。$\square$

**（c）** 独立重复算法 $t$ 次，取最大割。一次成功的概率为 $\epsilon$，$t$ 次全部失败的概率为 $(1 - \epsilon)^t$。要使成功概率 $\geq 1 - \delta$，只需：
$$
(1 - \epsilon)^t \leq \delta \quad \Rightarrow \quad t \geq \frac{\ln(1/\delta)}{\ln(1/(1-\epsilon))} \approx \frac{\ln(1/\delta)}{\epsilon}
$$

取 $\epsilon = 1/4$，$\delta = 1/n$，则 $t = O(\log n)$ 次重复即可。$\square$

---

#### （2）Load Balancing 与 Chernoff 界（Load Balancing, Slide19）（7'）

有 $n$ 台相同的处理器，$m = 16n \log n$ 个作业到达。每个作业独立均匀随机地分配给一台处理器。每台处理器的期望负载为 $\mu = 16 \log n$。

（a）利用 Chernoff 上界证明 $\Pr[X_i > 2\mu] < 1/n^2$。

（b）利用 Chernoff 下界证明 $\Pr[X_i < \mu/2] < 1/n^2$。

（c）利用 Union Bound 证明所有处理器的负载都在 $\mu/2$ 到 $2\mu$ 之间的概率至少为 $1 - 2/n$。

（已知 Chernoff 界：对独立 0-1 变量 $X_1, \ldots, X_n$，$X = \sum X_i$，$\mu = E[X]$：
- 上界：$\Pr[X > (1+\delta)\mu] < \left[\frac{e^\delta}{(1+\delta)^{1+\delta}}\right]^\mu$，$\delta > 0$
- 下界：$\Pr[X < (1-\delta)\mu] < e^{-\delta^2\mu/2}$，$0 < \delta < 1$）

**答案**：

**（a）上界**：

$\delta = 1$ 时 $(1+\delta) = 2$，代入 Chernoff 上界：
$$
\Pr[X_i > 2\mu] < \left(\frac{e^1}{2^2}\right)^\mu = \left(\frac{e}{4}\right)^{16\log n}
$$

由于 $e/4 \approx 0.679 < 1/e^2 \approx 0.135$：
$$
\Pr[X_i > 2\mu] < \left(\frac{1}{e^2}\right)^{16\log n} = e^{-32\log n} = n^{-32} \ll \frac{1}{n^2}
$$

更精确地：$\left(\frac{e}{4}\right)^{16\log n} = n^{16\log(e/4)} \approx n^{-6.18} < \frac{1}{n^2}$。

因此 $\Pr[X_i > 2\mu] < 1/n^2$。$\square$

**（b）下界**：

$\delta = 1/2$ 时 $(1-\delta) = 1/2$，代入 Chernoff 下界：
$$
\Pr[X_i < \mu/2] < e^{-(1/2)^2 \cdot \mu / 2} = e^{-\mu/8} = e^{-2\log n} = \frac{1}{n^2}
$$

$\square$

**（c）Union Bound**：

令事件 $A_i = \{X_i > 2\mu\}$，$B_i = \{X_i < \mu/2\}$。由 Union Bound：
$$
\Pr[\exists i,\; X_i > 2\mu] \leq \sum_{i=1}^n \Pr[X_i > 2\mu] < n \cdot \frac{1}{n^2} = \frac{1}{n}
$$
$$
\Pr[\exists i,\; X_i < \mu/2] \leq \sum_{i=1}^n \Pr[X_i < \mu/2] < n \cdot \frac{1}{n^2} = \frac{1}{n}
$$

因此所有机器负载均在 $[\mu/2, 2\mu]$ 内的概率：
$$
\Pr[\forall i,\; \mu/2 \leq X_i \leq 2\mu] \geq 1 - \frac{1}{n} - \frac{1}{n} = 1 - \frac{2}{n}
$$

即以 $\geq 1 - 2/n$ 的高概率，每台机器负载介于 $8\log n$ 和 $32\log n$ 之间，实现了良好的负载均衡。$\square$

---

### 4. 随机 QuickSort 与几何分布（15 分）

#### （1）随机 QuickSort 的期望比较次数（Las Vegas, Slide19）（8'）

对于随机 QuickSort（每次均匀随机选择 pivot），证明其期望比较次数为 $O(n\log n)$。

**答案**：

**证明**：设 $T(n)$ 为对 $n$ 个元素排序的期望比较次数。当随机选择 pivot 时，pivot 是第 $k$ 小元素的概率为 $1/n$（$k = 1, \ldots, n$）。划分后，左右子数组大小分别为 $k-1$ 和 $n-k$。

递推关系：
$$
T(n) = (n-1) + \frac{1}{n}\sum_{k=1}^n [T(k-1) + T(n-k)]
$$

其中 $n-1$ 是划分时的比较次数。由于 $\sum_{k=1}^n T(k-1) = \sum_{k=0}^{n-1} T(k)$，且 $\sum_{k=1}^n T(n-k) = \sum_{k=0}^{n-1} T(k)$：
$$
T(n) = (n-1) + \frac{2}{n}\sum_{k=0}^{n-1} T(k)
$$

其中 $T(0) = T(1) = 0$。下面用代入法证明 $T(n) \leq 2n\log n$（以 $e$ 为底）。

**归纳基础**：$n=2$ 时，$T(2) = 1 < 2 \cdot 2 \cdot \log 2 \approx 2.77$，成立。

**归纳步骤**：假设对 $i < n$ 有 $T(i) \leq 2i\log i$，则：
$$
\begin{aligned}
T(n) &\leq (n-1) + \frac{2}{n}\sum_{k=2}^{n-1} 2k\log k \\
&\leq (n-1) + \frac{4}{n}\int_{2}^{n} x\log x \, dx \\
&= (n-1) + \frac{4}{n}\left[\frac{x^2}{2}\log x - \frac{x^2}{4}\right]_{2}^{n} \\
&= (n-1) + \frac{4}{n}\left(\frac{n^2}{2}\log n - \frac{n^2}{4} - \frac{4}{2}\log 2 + \frac{4}{4}\right) \\
&= (n-1) + 2n\log n - n - \frac{8\log 2}{n} + \frac{4}{n} \\
&< 2n\log n
\end{aligned}
$$

因此 $T(n) = O(n\log n)$。$\square$

---

#### （2）几何分布与 Coupon Collector（Probability Review, Slide19）（7'）

Coupon Collector 问题：有 $n$ 种不同的优惠券，每盒谷物食品独立均匀地包含其中一种。设 $X$ 为收齐所有 $n$ 种优惠券所需购买的盒数。

（a）将 $X$ 分解为 $n$ 个几何随机变量之和，并推导 $E[X] = nH(n)$，其中 $H(n) = \sum_{i=1}^n 1/i$ 是调和数。
（b）证明 $\ln(n+1) < H(n) < 1 + \ln n$。
（c）说明 $E[X] = \Theta(n\log n)$。

**答案**：

**（a）期望推导**：

设 $X_i$ 为已收集 $i-1$ 种不同优惠券后，收集到第 $i$ 种新优惠券所需购买的盒数。则 $X = \sum_{i=1}^n X_i$。

当已有 $i-1$ 种优惠券时，获得一个新种类的概率为 $p_i = 1 - (i-1)/n = (n-i+1)/n$。$X_i$ 服从参数为 $p_i$ 的几何分布，$E[X_i] = 1/p_i = n/(n-i+1)$。

由期望线性性：
$$
E[X] = \sum_{i=1}^n E[X_i] = \sum_{i=1}^n \frac{n}{n-i+1} = n \sum_{j=1}^n \frac{1}{j} = n H(n)
$$

$\square$

**（b）调和数界**：

利用积分近似：
$$
H(n) = \sum_{i=1}^n \frac{1}{i} > \int_1^{n+1} \frac{dx}{x} = \ln(n+1)
$$

右侧：
$$
H(n) = 1 + \sum_{i=2}^n \frac{1}{i} < 1 + \int_1^n \frac{dx}{x} = 1 + \ln n
$$

因此 $\ln(n+1) < H(n) < 1 + \ln n$。$\square$

**（c）渐近分析**：

由（b）的界：
$$
n\ln(n+1) < E[X] < n(1 + \ln n)
$$

由于 $\ln(n+1) = \Theta(\log n)$，有 $E[X] = \Theta(n\log n)$。

这意味着收齐 $n$ 种优惠券的期望购买次数是 $n$ 倍调和数，即 $\Theta(n\log n)$。当 $n$ 较大时，最后几种优惠券的收集需要很长时间（如收集最后一种时需要期望 $n$ 盒）。$\square$
