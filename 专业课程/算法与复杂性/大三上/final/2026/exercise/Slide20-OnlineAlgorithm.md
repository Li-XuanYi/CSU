# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：Online Algorithm（Slide20）

---

## 一、选择题

**1.（Competitive Ratio Definition, Slide20）** 关于在线算法的竞争比（Competitive Ratio），以下说法正确的是：

- (A) 在线算法 $A$ 是 $\alpha$-competitive 的，如果对所有输入序列 $\sigma$，有 $C_{ALG}(\sigma) \le \alpha \cdot C_{OPT}(\sigma)$
- (B) 在线算法 $A$ 是 $\alpha$-competitive 的，如果对所有输入序列 $\sigma$，有 $C_{ALG}(\sigma) \le \alpha \cdot C_{OPT}(\sigma) + \delta$，其中 $\delta$ 为常数
- (C) 竞争比 $\alpha$ 越大，算法性能越好
- (D) 竞争比只与算法本身有关，与输入序列无关

**答案**：B **解析**：根据讲义（Slide20 p.5），在线算法 $A$ 是 $\alpha$-competitive 的，如果 $C_{ALG}(\sigma) \le \alpha \cdot C_{OPT}(\sigma) + \delta$ 对所有输入序列 $\sigma$ 成立，其中 $\delta$ 是某个常数。竞争比 $\alpha$ 越小代表算法性能越好（越接近最优离线算法）。竞争比分析依赖于最坏情况输入。

---

**2.（Ski-Rental Algorithm, Slide20）** 在 Ski-Rental 问题中，假设买滑雪板的费用是租金的 $k$ 倍。Simple Algorithm 的策略是前 $k'$ 次滑雪都租，第 $k'+1$ 次购买。为使竞争比最小，$k'$ 应该取何值？

- (A) $k' = k$
- (B) $k' = k - 1$
- (C) $k' = k + 1$
- (D) $k' = \lfloor k/2 \rfloor$

**答案**：B **解析**：根据讲义（Slide20 p.8-9），当 $k' = k - 1$ 时，竞争比 $\alpha = \frac{C_{ALG}}{C_{OPT}} = \frac{k' + k}{k} = \frac{2k - 1}{k} = 2 - \frac{1}{k}$。这是因为恶意对手会让滑雪者在购买后不再有机会滑雪（最优花费为 $k$），算法花费为 $k'+k$ 次租金。取 $k'=k-1$ 最小化了 $C_{OPT}$（即 $k$）并使得 $C_{ALG} = 2k-1$。

---

**3.（Ski-Rental with Prediction, Slide20）** 在引入预测的 Ski-Rental 算法中（参数 $\lambda$），若预测的滑雪天数 $y \ge k$，算法会在第 $\lceil \lambda k \rceil$ 天购买滑雪板。以下说法正确的是：

- (A) 当 $\lambda = 0$ 时，算法退化为"从一开始就购买"
- (B) 当 $\lambda = 1$ 时，算法退化为"从第 $k$ 天开始才考虑购买"
- (C) 预测越准确，竞争比越接近 $1$
- (D) 以上都对

**答案**：D **解析**：
- 当 $\lambda = 0$ 时，$\lceil \lambda k \rceil = 0$，即第一天就购买，相当于从不租直接买。
- 当 $\lambda = 1$ 时，$\lceil \lambda k \rceil = k$，即租 $k-1$ 次后在第 $k$ 次购买，接近经典的 Ski-Rental 最优策略。
- 预测误差 $\eta = |y - x|$ 越小，竞争比越接近 $1$（见 Slide20 p.14 定理中的第二项 $(1+\lambda) + \frac{\eta}{(1-\lambda)C_{OPT}}$）。

---

**4.（Online Paging - LIFO, Slide20）** 关于 LIFO（Last In First Out）页面置换策略在 Online Paging 问题中的表现，以下说法正确的是：

- (A) LIFO 是 $k$-competitive 的
- (B) LIFO 不是 $\alpha$-competitive 的，其竞争比无上界
- (C) LIFO 在缓存大小为 2 时是最优的
- (D) LIFO 与 LRU 的竞争比相同

**答案**：B **解析**：根据讲义（Slide20 p.23-24），考虑 cache 大小为 $k$，初始包含页面 $1$ 到 $k$，最后加载的是 $k$。请求序列 $\sigma = k+1, k, k+1, k, \dots$。LIFO 每次都会产生 cache miss，而对最优算法来说只有第一次请求 $k+1$ 时产生一次 miss。因此 LIFO 的竞争比可以任意大（unbounded）。

---

**5.（Online Paging - LFU, Slide20）** 关于 LFU（Least Frequently Used）页面置换策略，以下说法正确的是：

- (A) LFU 是 $k$-competitive 的
- (B) LFU 的竞争比可以通过增加请求次数来变得任意大
- (C) LFU 和 FIFO 具有相同的竞争比
- (D) LFU 在稳定访问模式下表现很差

**答案**：B **解析**：根据讲义（Slide20 p.25-26），设计请求序列：先请求页面 $1$ 到 $k-1$ 各 $m$ 次，然后交替请求 $k+1$ 和 $k$ 各 $m$ 次。LFU 会因为频繁计数导致 $k+1$ 和 $k$ 交替被驱逐而产生 $2m$ 次 cache miss，而最优算法仅第一次请求 $k+1$ 时产生 1 次 miss。当 $m$ 增大时，竞争比可以任意大。

---

**6.（LRU Competitive Ratio, Slide20）** 关于 LRU（Least Recently Used）页面置换策略的竞争比分析，以下说法正确的是：

- (A) LRU 的竞争比为 $2$
- (B) LRU 是 $k$-competitive 的，其中 $k$ 是缓存大小
- (C) LRU 不是 $\alpha$-competitive 的
- (D) LRU 的竞争比与缓存大小无关

**答案**：B **解析**：根据讲义（Slide20 p.28-31），LRU 是 $k$-competitive 的（$k$ 为缓存大小）。证明通过将请求序列划分为 phase，每个 phase 中 LRU 恰好发生 $k$ 次 cache miss。通过分情况讨论（phase 内同一页面被 LRU fault 两次，或 phase 内 LRU fault 发生在 $k$ 个不同页面上），可以证明在每个 phase 中 OPT 至少 fault 一次。

---

**7.（Online Paging - FIFO, Slide20）** 以下关于 FIFO 和 LRU 的比较，正确的是：

- (A) FIFO 的竞争比优于 LRU
- (B) FIFO 和 LRU 都是 $k$-competitive 的
- (C) FIFO 不是 $\alpha$-competitive 的
- (D) LRU 是 $k$-competitive，而 FIFO 是 $k^2$-competitive

**答案**：B **解析**：根据讲义（Slide20 p.28-34），LRU 和 FIFO 都是 $k$-competitive 的（$k$ 为缓存大小）。两者的竞争比分析思路类似，都通过 phase 划分和反证法证明在每个 phase 中 OPT 至少 fault 一次，因此竞争比上界均为 $k$。

---

## 二、问答题

### 1. Ski-Rental 与 Competitive Ratio 分析（18 分）

#### （1）Ski-Rental 经典算法（Ski-Rental Algorithm, Slide20）（6'）

**问题**：在 Ski-Rental 问题中，设购买滑雪板的费用是单次租金的 $k$ 倍。Simple Algorithm 的策略是：前 $k-1$ 次都租，第 $k$ 次及之后购买。

请分析：
1. 该算法的竞争比是多少？
2. 给出最坏情况输入并说明为何该算法达到这一竞争比。
3. 如果 $k=10$，$k-1=9$ 次租后第 10 次买，且实际滑雪的次数恰好是 10 次，此时算法和最优的花费各是多少？竞争比是多少？

**答案**：

**竞争比分析**：
设 $x$ 为实际滑雪次数，$C_{OPT} = \min\{k, x\}$，

$$C_{ALG} = \begin{cases} x, & x < k \\ (k-1) + k = 2k-1, & x \ge k \end{cases}$$

最坏情况是 $x = k$（刚好在购买后不再使用）：
$$\alpha = \frac{C_{ALG}}{C_{OPT}} = \frac{(k-1)+k}{k} = 2 - \frac{1}{k}$$

**最坏情况输入**：一个恶意的"天气之神"让滑雪者恰好滑雪 $k$ 次——前 $k-1$ 次是租的（每次花费 $1$），第 $k$ 次购买（花费 $k$），之后就再也没机会滑雪了。最优策略则是一开始就购买，花费 $k$。

**当 $k=10$，实际滑 10 次**：
- 算法花费：$9 \times 1 + 10 = 19$（前 9 次租 + 第 10 次买）
- 最优花费：$10$（直接买的费用）
- 实际竞争比：$19/10 = 1.9$，等于 $2 - 1/10$

---

#### （2）含预测的 Ski-Rental 算法（Algorithm with Prediction, Slide20）（6'）

**问题**：给定预测天数 $y$ 和参数 $0 \le \lambda < 1$，考虑如下确定性算法：

```
Algorithm: Deterministic Ski-Rental with Prediction
Input: 参数 λ, 预测天数 y, 购买费用 k

1. if y ≥ k then
2.     在第 ⌈λk⌉ 天购买滑雪板
3. else
4.     在第 ⌈k/λ⌉ 天购买滑雪板
5. end if
```

证明该算法有如下竞争比上界：
$$\min\left\{\frac{1+\lambda}{\lambda}, \; (1+\lambda) + \frac{\eta}{(1-\lambda)C_{OPT}}\right\}$$

其中 $\eta = |y - x|$ 是预测误差，$x$ 为实际滑雪天数，$C_{OPT} = \min\{k, x\}$。

**答案**：

**证明第一个竞争比上界 $\frac{1+\lambda}{\lambda}$**：

**Case 1**：$y \ge k$（预测会滑很多次）
- 如果 $x \ge \lceil \lambda k \rceil$：$C_{ALG} = k + \lceil \lambda k \rceil - 1 \le k + \lambda k = (1+\lambda)k$。注意到 $\lceil \lambda k \rceil \le \frac{1+\lambda}{\lambda}C_{OPT}$，因为 $C_{OPT} \ge \lceil \lambda k \rceil$ 时 $x \ge \lceil \lambda k \rceil$，此时 $C_{OPT} \ge \lceil \lambda k \rceil \ge \lambda k$，所以 $(1+\lambda)k \le \frac{1+\lambda}{\lambda} \cdot \lambda k \le \frac{1+\lambda}{\lambda}C_{OPT}$。
- 如果 $x < \lceil \lambda k \rceil$：$C_{ALG} = x = C_{OPT}$。

**Case 2**：$y < k$（预测不会滑很多次）
- 如果 $x \ge \lceil k/\lambda \rceil$：$C_{ALG} = k + \lceil k/\lambda \rceil - 1 \le k + k/\lambda = \frac{1+\lambda}{\lambda}k \le \frac{1+\lambda}{\lambda}C_{OPT}$（因为此时 $C_{OPT} = k$）。
- 如果 $x < \lceil k/\lambda \rceil$：$C_{ALG} = x = C_{OPT}$。

**证明第二个竞争比上界 $(1+\lambda) + \frac{\eta}{(1-\lambda)C_{OPT}}$**：

**Case 1**：$y \ge k$
- 若 $y \le x$：则 $k \le x = C_{OPT}$，所以 $C_{ALG} \le (1+\lambda)k \le (1+\lambda)C_{OPT}$
- 若 $y > x$：则 $k \le y - x + x = \eta + C_{OPT}$，所以 $C_{ALG} \le (1+\lambda)k \le (1+\lambda)(C_{OPT} + \eta)$
- 当 $x < \lceil \lambda k \rceil$：$C_{ALG} = x = C_{OPT}$

综上，$C_{ALG} \le C_{OPT} + \frac{\eta}{1-\lambda}$（因为 $\eta = x-y$ 或 $y-x$，且当 $x \ge \lceil \lambda k \rceil$ 时最坏情况出现在 $\eta$ 很大时，$k = C_{OPT}$ 需修正），进一步得到 $C_{ALG} \le \left((1+\lambda) + \frac{\eta}{(1-\lambda)C_{OPT}}\right)C_{OPT}$。

**Case 2**：$y < k$（类似可证）

因此竞争比为两个 bound 中的最小值。

---

#### （3）鲁棒性与一致性分析（Robustness and Consistency, Slide20）（6'）

**问题**：在含预测的在线算法设计中，通常追求两个性质：
- **一致性（Consistency）**：当预测准确时，算法性能好
- **鲁棒性（Robustness）**：当预测不准确时，算法性能不会太差

对于上述带参数 $\lambda$ 的 Ski-Rental 算法，请分析：
1. $\lambda$ 趋近于 0 时，算法的一致性和鲁棒性如何？
2. $\lambda$ 趋近于 1 时，算法的一致性和鲁棒性如何？
3. 如何选择一个合适的 $\lambda$ 来平衡一致性和鲁棒性？给出直观解释。

**答案**：

**$\lambda \to 0$ 的情况**：
- 当 $y \ge k$ 时，第 $\lceil \lambda k \rceil \approx 0$ 天购买 → 几乎一开始就买。
- 一致性（预测准确时）：若 $y \ge k$ 且 $x$ 很大，$C_{ALG} \approx k$，接近最优。
- 鲁棒性（预测不准时）：若 $y \ge k$ 但 $x$ 很小（如只滑 1 次），算法直接购买花费 $k$，而最优只需 $1$ → 竞争比可达到 $k$。鲁棒性很差。

**$\lambda \to 1$ 的情况**：
- 当 $y \ge k$ 时，第 $\lceil \lambda k \rceil \approx k$ 天购买 → 接近经典 Ski-Rental 最优策略（$k-1$ 次租后买）。
- 鲁棒性：$\frac{1+\lambda}{\lambda} \approx 2$，即使预测不准竞争比也控制在 $2$ 附近，鲁棒性好。
- 一致性：若预测准确（$y \ge k$ 且 $x$ 很大），$C_{ALG} \approx 2k-1$，而最优只有 $k$ → 竞争比接近 $2$，不如 $\lambda \to 0$ 时的 $1$。

**平衡选择**：
合适的 $\lambda$ 需要权衡：
- 当对预测质量有信心时：选择较小的 $\lambda$（如 $0.3$），追求更好的一致性。
- 当对预测质量没信心时：选择较大的 $\lambda$（如 $0.8$），确保鲁棒性（竞争比不超过 $2$ 左右）。
- 实际中常取 $\lambda = 0.5$ 作为折中，此时两个 bound 分别为 $3$ 和 $1.5 + \frac{2\eta}{C_{OPT}}$。

---

### 2. Online Paging 问题与竞争比分析（17 分）

#### （1）LIFO 竞争比分析（Non-α-competitive Algorithm, Slide20）（5'）

**问题**：证明 LIFO（Last In First Out）策略不是 $\alpha$-competitive 的（即竞争比无上界）。

**答案**：

设缓存大小为 $k$，主存中共有 $n > k$ 个页面。初始时缓存包含页面 $1$ 到 $k$，且最后被加载到缓存中的页面是 $k$。

考虑请求序列 $\sigma = k+1, k, k+1, k, \dots$（交替请求 $k+1$ 和 $k$）。

**LIFO 的执行过程**：
1. 请求 $k+1$：cache miss。LIFO 将最后加载的 $k$ 驱逐，加载 $k+1$。
2. 请求 $k$：cache miss（$k$ 刚被驱逐）。LIFO 将最后加载的 $k+1$ 驱逐，加载 $k$。
3. 重复步骤 1-2。

因此，LIFO 在序列 $\sigma$ 的每个请求上都产生 cache miss。

**最优算法的执行过程**：
1. 第一次请求 $k+1$：cache miss。OPT 驱逐页面 $k$（或其他非必需页面），加载 $k+1$。
2. 之后所有请求 $k$ 和 $k+1$：都在 cache 中，不产生 miss。

因此，OPT 在 $\sigma$ 上只产生 1 次 cache miss。

令序列长度为 $2m$，则 $C_{LIFO} = 2m$，$C_{OPT} = 1$。当 $m \to \infty$ 时，竞争比 $\frac{C_{LIFO}}{C_{OPT}} = 2m \to \infty$，无上界。因此 LIFO 不是 $\alpha$-competitive 的。

---

#### （2）LFU 竞争比分析（Non-α-competitive Algorithm, Slide20）（5'）

**问题**：证明 LFU（Least Frequently Used）策略不是 $\alpha$-competitive 的。

**答案**：

设缓存大小为 $k$，初始包含页面 $1$ 到 $k$。设计如下请求序列：
1. 先请求页面 $1$ 到 $k-1$ 各 $m$ 次（使得它们的使用频率变高）。
2. 然后交替请求 $k+1$ 和 $k$ 各 $m$ 次。

**LFU 的执行过程**：
- 在阶段 1 结束后，页面 $1$ 到 $k-1$ 的频率均为 $m$，页面 $k$ 的频率为 $0$（还未被请求）。
- 阶段 2 开始，请求 $k+1$（cache miss），LFU 驱逐频率最低的页面（此时是 $k$ 或 $k+1$ 计为 $1$），加载 $k+1$。
- 请求 $k$（cache miss），LFU 比较发现 $k+1$ 频率为 $1$ 最低，驱逐 $k+1$ 加载 $k$。
- 如此交替，每次请求 $k+1$ 时 $k$ 刚被调高频率、$k+1$ 频率最低...每次请求都产生 cache miss。
- 总共产生 $2m$ 次 cache miss。

**最优算法**：
第一次请求 $k+1$ 时驱逐页面 $1$，之后 $k$ 和 $k+1$ 都在 cache 中，只产生 1 次 miss。

因此竞争比可达 $\frac{2m}{1} = 2m$，当 $m \to \infty$ 时无上界。

---

#### （3）LRU 是 $k$-competitive 的证明（k-competitive Algorithm, Slide20）（7'）

**问题**：证明 LRU（Least Recently Used）页面置换策略是 $k$-competitive 的，其中 $k$ 是缓存大小。

**答案**：

**证明思路**：考虑任意请求序列 $\sigma = \sigma_1, \sigma_2, \dots, \sigma_m$。假设 LRU 和 OPT 初始时缓存相同。将 $\sigma$ 划分为若干 **phase**（阶段），每个 phase 中 LRU 恰好产生 $k$ 次 cache miss。我们只需证明每个 phase 中 OPT 至少产生 1 次 cache miss。

**Phase 划分**：从第一个请求开始，每当 LRU 累积了 $k$ 次 cache miss 时开始一个新的 phase。

**Case 1**：Phase 内 LRU 对某个页面 $\sigma_i$ 至少 fault 两次。

若 $\sigma_i$ 在一个 phase 内被调入（fault）后又因被驱逐再次 fault，说明 $\sigma_i$ 在被调入后成为了 most recently used，但后来又成为了 least recently used 而被驱逐。

当 $\sigma_i$ 被驱逐时，cache 中其他 $k-1$ 个页面必定都在 $\sigma_i$ 被调入后至少被请求过一次（因为 LRU 策略总是驱逐最久未使用的页面）。考虑：
- $\sigma_i$ 被调入的那次请求
- 导致 $\sigma_i$ 被逐出的那次请求（必定来自另一个不同于 $\sigma_i$ 的页面）

至少有 $k+1$ 个 **不同的页面** 在这个 phase 中被请求。OPT 的缓存只有 $k$ 个位置，因此 OPT 必须至少 fault 一次（因为它不可能同时容纳 $k+1$ 个不同页面）。

**Case 2**：Phase 内 LRU fault 发生在 $k$ 个不同页面上。

- 如果这是第一个 phase：LRU 和 OPT 初始缓存相同，但 $k$ 次 fault 涉及 $k$ 个页面，它们不可能同时都在 OPT 的初始缓存中（共 $k$ 个位置），因此 OPT 至少 fault 一次。
- 如果不是第一个 phase：设 $\sigma_i$ 是上一个 phase 的最后一个请求页面（该页面在 phase 结束时同时在 LRU 和 OPT 的缓存中）。
  - **情况 2a**：LRU 在 $\sigma_i$ 上没有 fault。那么 LRU 在这个 phase 中 fault 的 $k$ 个页面不能全部在 OPT 的缓存中（因为 $\sigma_i$ 占据了一个位置），所以 OPT 至少 fault 一次。
  - **情况 2b**：LRU 在 $\sigma_i$ 上也有 fault。类似 Case 1 的论证：$\sigma_i$ 被调入时是 most recently used，后来被驱逐时成为 least recently used，意味着其他 $k-1$ 个页面至少被请求了一次。加上导致 $\sigma_i$ 被驱逐的请求，至少 $k$ 个不同页面在 $\sigma_i$ 驻留期间被请求。加上 $\sigma_i$ 本身，总共 $k+1$ 个不同页面，OPT 无法全部容纳。

**结论**：每个 phase 中 OPT 至少 fault 一次，而每个 phase 中 LRU 恰好 fault $k$ 次。因此对所有输入序列 $\sigma$，有 $C_{LRU}(\sigma) \le k \cdot C_{OPT}(\sigma) + \delta$（其中 $\delta$ 为最后一 phase 可能未满时的常数项）。故 LRU 是 $k$-competitive 的。

---
