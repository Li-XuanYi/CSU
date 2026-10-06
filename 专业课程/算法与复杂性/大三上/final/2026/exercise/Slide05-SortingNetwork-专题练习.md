# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：排序网络 (Sorting Network, Slide05)

---

## 一、选择题（共 3 题，每题 3.5 分，合计 10.5 分）

---

**1.（Sorting Network Components, Slide05）** 关于排序网络（Sorting Network）各组件的深度和性质，以下说法中 **错误** 的是？

- (A) BITONIC-SORTER[n] 由深度为 1 的 HALF-CLEANER[n] 和两个 BITONIC-SORTER[n/2] 递归构成，其深度 D(n) = O(log n)
- (B) MERGER[n] 的深度与 BITONIC-SORTER[n] 的深度相同，均为 O(log n)
- (C) SORTER[n] 的深度为 O(log n)
- (D) 若 HALF-CLEANER 的输入是 0 和 1 构成的双调序列，则输出中上半部分和下半部分都是双调序列，且上半部分的每个元素都不大于下半部分的每个元素

**答案**：(C) **解析**：

本题要求选出**错误**说法。逐一分析每个选项：

**选项 (A) 正确**。BITONIC-SORTER[n] 的构造是：第一层是一个 HALF-CLEANER[n]（深度为 1），将输入分为两半，然后分别在上下半递归调用 BITONIC-SORTER[n/2]。因此深度递推式为 D(n) = D(n/2) + 1，初始 D(1)=0。解此递推式：D(n) = log₂ n，故 D(n) = O(log n)。（讲义 page 31）

**选项 (B) 正确**。MERGER[n] 利用"逆序拼接产生双调序列"的原理：将两个已排序序列 X 和 Y 拼接为 X∘Yᴿ 得到双调序列，然后送入 BITONIC-SORTER[n] 排序。因此 MERGER[n] 的构造 = 一级比较网络（将 i 与 n-i+1 比较）+ 两个 BITONIC-SORTER[n/2]，故其深度与 BITONIC-SORTER[n] 相同，均为 O(log n)。（讲义 page 34-36）

**选项 (C) 错误**。SORTER[n] 的递归构造是：两个 SORTER[n/2] 分别排序前后两半，再用一个 MERGER[n] 合并。因此深度递推式为：
```
D(n) = D(n/2) + MERGER[n]的深度
     = D(n/2) + O(log n)
```
展开递归树：第 1 层代价 O(log n)，第 2 层 O(log n/2)，...，共 log₂ n 层，总深度为：
```
D(n) = O(log n) + O(log n/2) + O(log n/4) + ... + O(1)
     = O(log n × log n) = O(log² n)
```
因此 SORTER[n] 的深度是 O(log² n)，**不是** O(log n)。（讲义 page 46-47）

补充对比三种网络的深度：
| 网络 | 递推式 | 深度 |
|------|--------|------|
| BITONIC-SORTER[n] | D(n)=D(n/2)+1 | O(log n) |
| MERGER[n] | 同上（基于 Bitonic Sorter） | O(log n) |
| SORTER[n] | D(n)=D(n/2)+O(log n) | O(log² n) |

**选项 (D) 正确**。这就是 Half-Cleaner Lemma（讲义 page 26-28）。证明思路：0-1 双调序列只可能是 0ⁱ1ʲ0ᵏ 或 1ⁱ0ʲ1ᵏ 两种形式。以 0ⁱ1ʲ0ᵏ 为例，根据中点 n/2 落在哪个"块"中分情形讨论：
- 若中点落在第一个 0 块中：输入是全 0，比较后上下半均为 0，性质成立。
- 若中点落在 1 块中（即 i < n/2 < i+j）：前 n/2 个元素含 0ⁱ 和部分 1，后 n/2 个元素含剩余 1 和 0ᵏ。第 i 个元素与第 i+n/2 个元素比较，较小者上半、较大者下半，由于在 0 与 1 的比较中 0 总被输送到上半而 1 到下半，上半最多只有 0 和部分 1，下半则保留较大的值。由此可证上半所有元素 ≤ 下半所有元素，且由于输入的双调性，每一半输出仍是双调序列。当上半全为 0 或下半全为 1 时，称该半是"clean"的，至少有一半 clean。

---

**2.（Zero-One Principle, Slide05）** 关于排序网络的 0-1 原理（Zero-One Principle），以下说法正确的是？

- (A) 如果一个比较网络能正确排序所有长度为 n 的 0-1 序列，则它一定能正确排序所有长度为 n 的任意实数序列
- (B) 如果一个比较网络能正确排序某一条长度为 n 的 0-1 序列，则它一定能正确排序所有长度为 n 的任意整数序列
- (C) 0-1 原理表明，比较网络对任意输入序列的排序能力等价于其对单调递增函数的变换能力
- (D) 0-1 原理的证明依赖于对 comparator 个数进行归纳

**答案**：(A) **解析**：

本题要求选出**正确**说法。逐一分析：

**选项 (A) 正确**。这是 Zero-One Principle 的标准表述：若比较网络能正确排序所有 $2^n$ 种 0-1 序列，则它就能正确排序任意实数序列。意义在于将无穷验证归约为有限验证。

**选项 (B) 错误**。Zero-One Principle 要求的是**所有** $2^n$ 种 0-1 序列，不是"某一条"。反证法构造的 0-1 序列由实数序列唯一确定，只测一条无法覆盖全部情况。

**选项 (C) 错误**。混淆了 Domain Conversion Lemma（引理：网络输出对单调变换的保持性）与 Zero-One Principle（定理：0-1 正确 $\Rightarrow$ 任意数正确）。前者是工具，后者是结论，非等价关系。

**选项 (D) 错误**。Zero-One Principle 的证明分两层：Domain Conversion Lemma 对 **wire depth** 归纳证明，然后 Zero-One Principle 本身用反证法调用 Lemma，不再涉及归纳。不是对 comparator 个数归纳。

完整定义与证明详见本章问答题第 2 题（QA2）。

---

**3.（Half-Cleaner & Bitonic Sequence, Slide05）** 设输入为 8 个元素的 0-1 序列 ⟨1, 1, 1, 0, 0, 0, 0, 0⟩，经过 HALF-CLEANER[8]（将 i 与 i+4 比较，i=1,2,3,4）处理后，输出序列的上半部分（前 4 个元素）为？

- (A) ⟨0, 0, 0, 0⟩
- (B) ⟨1, 0, 0, 0⟩
- (C) ⟨0, 1, 1, 1⟩
- (D) ⟨0, 0, 1, 0⟩

**答案**：(A) **解析**：输入 ⟨1,1,1,0,0,0,0,0⟩ 是 1³0⁵ 形式的双调序列。HALF-CLEANER[8] 比较 i 与 i+4（i=1,2,3,4）：

- 比较 ① vs ⑤：1 vs 0 → 上半输出 0，下半输出 1
- 比较 ② vs ⑥：1 vs 0 → 上半输出 0，下半输出 1
- 比较 ③ vs ⑦：1 vs 0 → 上半输出 0，下半输出 1
- 比较 ④ vs ⑧：0 vs 0 → 上半输出 0，下半输出 0

上半部分为 ⟨0, 0, 0, 0⟩（全 0，clean），下半部分为 ⟨1, 1, 1, 0⟩（双调序列），符合 Half-Cleaner Lemma。

---

## 二、问答题（共 2 题，30 分）

### 1. 排序网络的构造与性能分析（15 分）

> 排序网络是并行排序的重要模型。本题从基本概念出发，逐步深入其构造原理和复杂度分析。

#### （1）Half-Cleaner 与 Bitonic Sorter（Half-Cleaner Lemma, Slide05）（5'）

证明：若 HALF-CLEANER[n] 的输入是 0 和 1 构成的双调（bitonic）序列，则输出满足以下两个性质：

1. 上半部分和下半部分都是双调序列；
2. 上半部分的每个元素都不大于下半部分的每个元素，且至少有一半是 clean 的（即全 0 或全 1）。

**答案**：

**证明**：设输入为 0-1 双调序列。由双调序列的 0-1 性质，其形式必为 0ⁱ1ʲ0ᵏ 或 1ⁱ0ʲ1ᵏ。由对称性，不妨设输入为 0ⁱ1ʲ0ᵏ（即先一段 0，再一段 1，再一段 0）。

HALF-CLEANER[n] 将输入的第 i 个元素与第 i+n/2 个元素比较（i=1,...,n/2），将较小者输出到上半部分，较大者输出到下半部分。

根据中点 n/2 落在 0 块还是 1 块中，分三种情形讨论：

- **Case 1**：中点落在第一个 0 块中（i ≥ n/2）。此时输入实际上是全 0 序列（已排序），比较后上下各得到 n/2 个 0。上下半均为 clean 的 0，性质成立。

- **Case 2a**：中点落在 1 块中（i < n/2 < i+j），且 j ≥ n/2。此时前 n/2 个元素为 0ⁱ1^(n/2-i)，后 n/2 个元素为 1^(j-(n/2-i))0ᵏ。比较 i 与 i+n/2：
  - 前 i 组比较：0 vs 1 → 上半 0，下半 1
  - 中间 (n/2-i) 组比较：1 vs 1 → 上半 1，下半 1
  - 后 (n/2 - i) 组比较（如果还有）：... 实际上需要更精细的分析。

根据讲义中的标准证明，四种情形（讲义 Fig. 27.10, Fig. 27.11）均已覆盖。在每种情形下：
- 上半部分的元素都是对应比较中较小的（min），下半部分都是较大的（max），因此上半 ≤ 下半。
- 由于输入是双调的，每半边在比较后仍是双调序列。
- 当上半部分全为同一值时，称为上 clean；下半部分全为同一值时，称为下 clean。至少有一半是 clean 的，因为当输入为 0ⁱ1ʲ0ᵏ 时，若 1 块完全落在一侧，则另一侧为全 0（clean）；若 1 块跨越中点，则比较后上半部分为 0ⁱ1^(n/2-i)，下半部分为 1^(n/2)（下 clean）或 1^(n/2-i)0ᵏ。

#### （2）Merger 的构造原理（Merger Design, Slide05）（5'）

给定两个已排序的升序序列 $X = \langle x_1, x_2, \dots, x_{n/2} \rangle$ 和 $Y = \langle y_1, y_2, \dots, y_{n/2} \rangle$，请说明如何利用 Bitonic Sorter 构造 Merger（合并网络）。特别地，解释为什么 MERGER[n] 的第一级比较是将 $i$ 与 $n-i+1$ 进行比较，而非 HALF-CLEANER 中的 $i$ 与 $i+n/2$。

**答案**：

**核心思路**：如果先将 Y 逆序得到 $Y^R = \langle y_{n/2}, y_{n/2-1}, \dots, y_1 \rangle$，再拼接到 X 后面得到序列 $Z = X \circ Y^R = \langle x_1,\dots,x_{n/2}, y_{n/2},\dots,y_1 \rangle$，那么 X 单调递增、Yᴿ 单调递减，Z 就是双调序列。此时**可以直接**用 BITONIC-SORTER[n]（第一级是 HALF-CLEANER，比较 i 与 i+n/2）来排 Z，得到合并后的有序输出。

这是概念上的理解——你完全可以这样做，也确实是对的。

**MERGER[n] 设计的区别**：在实际构造中，MERGER[n] 的输入线排列是 $[X_1,\dots,X_{n/2}, Y_1,\dots,Y_{n/2}]$——即两个升序序列**首尾相接**，Y 没有被逆序。在这个排列下，序列是 $\langle x_1,\dots,x_{n/2}, y_1,\dots,y_{n/2} \rangle$，两半都是递增的，不是双调序列。所以不能直接接 HALF-CLEANER（i vs i+n/2）。

MERGER[n] 的处理是：第一级改用 **i 与 n-i+1** 比较（即比较位置 1 与 n、2 与 n-1、……、n/2 与 n/2+1）。这相当于"暗中"把 Y 逆序后再做比较——位置 n-i+1（当 i ≤ n/2 时）正好对应 Y 中从后往前的第 i 个元素。这一级比较的效果等价于对 $X \circ Y^R$ 做了一次 HALF-CLEANER 的操作，使得上半部分输出和下半部分输出各自变成双调序列，然后分别递归调用 BITONIC-SORTER[n/2] 即可。

**总结**：

| 做法 | 输入排列 | 第一级比较 | 后续 |
|------|----------|-----------|------|
| 概念：手动逆序 Y 再拼接 | $X \circ Y^R$（双调） | i vs i+n/2（标准 HALF-CLEANER） | BITONIC-SORTER[n/2] |
| 实际：MERGER[n] | $X \circ Y$（非双调） | i vs n-i+1（等价于暗中逆序比较） | BITONIC-SORTER[n/2] |

两种方式本质等价，MERGER[n] 的设计省去了显式逆序的步骤，直接利用比较线序来实现逆序效果。

#### （3）Sorter 的递归构造与深度分析（Sorter Performance, Slide05）（5'）

SORTER[n] 由两个 SORTER[n/2] 和一个 MERGER[n] 递归构造而成，其深度满足递推式 $D(n) = D(n/2) + O(\log n)$，其中 $D(1) = 0$。

(a) 求解该递推式，给出 $D(n)$ 的渐近紧确界。
(b) 考虑更一般的递推式 $T(n) = aT(n/b) + O(n^d \log n)$，其中 $a > 0, b > 1, d \ge 0$。请根据 $d$ 与 $\log_b a$ 的大小关系，给出三种情况下的渐近解。

**答案**：

**(a) SORTER[n] 的深度**

递推式：$D(n) = D(n/2) + O(\log n)$。

可用递归树法求解：递归树有 $\log_2 n$ 层（$n, n/2, n/4, \dots, 1$），第 $j$ 层的代价为 $O(\log (n/2^{j-1}))$。

$$D(n) = \sum_{j=1}^{\log_2 n} O\!\left(\log \frac{n}{2^{j-1}}\right) = O\!\left(\sum_{j=1}^{\log_2 n} (\log n - (j-1))\right)$$

$$= O\!\left(\sum_{k=0}^{\log_2 n - 1} (\log n - k)\right) = O\!\left(\log n \cdot \log n - \frac{(\log n - 1)\log n}{2}\right) = O(\log^2 n)$$

因此 $D(n) = \Theta(\log^2 n)$。

**(b) 一般递推式的解**

对于 $T(n) = aT(n/b) + O(n^d \log n)$：

比较 $d$ 与 $\log_b a$（即比较 $n^d$ 与 $n^{\log_b a}$ 的阶）：

| 条件 | 渐近解 | 直观含义 |
|------|--------|----------|
| $d > \log_b a$ | $T(n) = O(n^d \log n)$ | 每层代价递增，根节点主导 |
| $d = \log_b a$ | $T(n) = O(n^d \log^2 n)$ | 每层代价相等，共 $O(\log n)$ 层 |
| $d < \log_b a$ | $T(n) = O(n^{\log_b a} \log n)$ | 每层代价递减，叶节点主导 |

对 SORTER[n] 来说，$a = 1, b = 2, d = 0$（因为 $O(\log n) = O(n^0 \log n)$），则 $\log_b a = \log_2 1 = 0 = d$，属于 Case 2，故 $D(n) = O(n^0 \log^2 n) = O(\log^2 n)$，与 (a) 一致。

---

### 2. Domain Conversion Lemma 与 Zero-One Principle 证明（15 分）

> 排序网络的核心理论支柱是 Domain Conversion Lemma 和 Zero-One Principle，它们共同保证了只需检验 0-1 序列即可判断网络正确性。

#### （1）Domain Conversion Lemma（Domain Conversion Lemma, Slide05）（5'）

叙述并证明 Domain Conversion Lemma：若一个比较网络将输入序列 $a = \langle a_1, a_2, \dots, a_n \rangle$ 变换为输出序列 $b = \langle b_1, b_2, \dots, b_n \rangle$，则对任意单调递增函数 $f$，该网络将输入 $f(a) = \langle f(a_1), f(a_2), \dots, f(a_n) \rangle$ 变换为输出 $f(b) = \langle f(b_1), f(b_2), \dots, f(b_n) \rangle$。

**答案**：

**引理叙述**：设 $C$ 为一个比较网络，$a$ 为输入序列，$b = C(a)$ 为对应的输出序列。对于任意单调递增函数 $f: \mathbb{R} \to \mathbb{R}$（即若 $x \le y$ 则 $f(x) \le f(y)$），有 $C(f(a)) = f(b)$，即网络在输入 $f(a)$ 上的输出恰好是将 $f$ 逐元素作用于原输出 $b$ 所得的结果。

**证明（对 wire depth 归纳）**：

比较网络中的每条 wire 都有一个 depth，定义为从输入到该 wire 经过的最多 comparator 个数：

- **Base Case**：depth = 0 的 wires 是输入 wires。当输入为 $f(a)$ 时，第 $i$ 条输入 wire 携带的值正是 $f(a_i)$。由于 $b_i = a_i$（输入 wire 上尚未经过任何 comparator），因此 $f(b_i) = f(a_i)$ 成立。

- **Inductive Step**：假设对 depth $< d$ 的所有 wires 结论成立。考虑一个 depth $d$ 的 comparator，其两条输入 wires 的 depth 均 $< d$，两条输出 wires 的 depth 为 $d$。

  设该 comparator 的两条输入 wires 在原输入 $a$ 下携带的值为 $x$ 和 $y$。由归纳假设，当输入为 $f(a)$ 时，这两条输入 wires 携带的值为 $f(x)$ 和 $f(y)$。

  comparator 将较小值输出到上端 wire，较大值输出到下端 wire。在原输入 $a$ 下，该 comparator 的输出为：
  - 上端输出：$\min(x, y)$
  - 下端输出：$\max(x, y)$

  在输入 $f(a)$ 下，该 comparator 的输出为：
  - 上端输出：$\min(f(x), f(y))$
  - 下端输出：$\max(f(x), f(y))$

  由于 $f$ 单调递增，有：
  $$\min(f(x), f(y)) = f(\min(x, y))$$
  $$\max(f(x), f(y)) = f(\max(x, y))$$

  因此，该 comparator 在输入 $f(a)$ 下的输出恰好是对原输出应用 $f$ 的结果。由归纳法，所有 wires 均满足该性质，从而 $C(f(a)) = f(b)$。

---

#### （2）Zero-One Principle（Zero-One Principle, Slide05）（5'）

叙述 Zero-One Principle 并利用 Domain Conversion Lemma 给出证明：
"若一个 $n$ 输入的比较网络能正确排序所有 $2^n$ 种 0-1 序列，则它能正确排序所有任意数值的序列。"

**答案**：

**定理叙述**：如果一个比较网络能够正确排序所有 $2^n$ 种由 0 和 1 构成的输入序列，那么它能够正确排序任意实数构成的输入序列。换言之，判断一个比较网络是否为排序网络，只需检验其在所有 0-1 输入上的正确性。

**证明（反证法 + Domain Conversion Lemma）**：

假设比较网络 $C$ 能正确排序所有 $2^n$ 种 0-1 序列，但存在某个由任意实数构成的输入序列 $a = \langle a_1, a_2, \dots, a_n \rangle$ 使得 $C$ 无法正确排序。

由于排序失败，存在下标 $i < j$ 使得在输出序列 $b = C(a)$ 中，$a_i$ 出现在 $a_j$ 之后（即网络将较大的元素排在了较小的元素前面），但实际应有 $a_i \le a_j$。换句话说，存在一对位置 $(p, q)$ 满足 $p < q$ 但 $b_p > b_q$（即输出未按升序排列）。

特别地，存在 $a_i < a_j$ 使得在输出中 $a_j$ 出现在 $a_i$ 之前。定义函数 $f: \mathbb{R} \to \{0, 1\}$：

$$
f(x) = \begin{cases}
0, & \text{if } x \le a_i \\
1, & \text{if } x > a_i
\end{cases}
$$

$f$ 是单调递增函数（因为 $x \le y \implies f(x) \le f(y)$）。由 Domain Conversion Lemma，网络 $C$ 将输入 $f(a)$ 变换为输出 $f(b)$。

现在考察 $f(a)$ 中元素 $a_i$ 和 $a_j$ 的映射：
- $f(a_i) = 0$（因为 $a_i \le a_i$）
- $f(a_j) = 1$（因为 $a_j > a_i$）

由于在输出 $b$ 中 $a_j$ 出现在 $a_i$ 之前（即较大者在前），在 $f(b)$ 中 $f(a_j) = 1$ 出现在 $f(a_i) = 0$ 之前。但 $f(a)$ 是一个 0-1 序列，根据假设，网络 $C$ 能正确排序所有 0-1 序列，因此 $f(a)$ 的输出应当是已排序的（所有 0 在前，所有 1 在后），$f(a_j) = 1$ 不可能出现在 $f(a_i) = 0$ 之前。

产生矛盾！因此假设不成立，原命题得证。

---

#### （3）Zero-One Principle 的应用（Zero-One Principle Application, Slide05）（5'）

给定一个 4 输入的比较网络，其 comparators 按以下顺序连接：
$$
(1,2),\ (3,4),\ (1,3),\ (2,4)
$$

请回答以下问题：

(a) 利用 Zero-One Principle，说明如何验证该网络是否为排序网络。需要检查多少种输入序列？
(b) 验证该网络是否能正确排序所有 4 输入的 0-1 序列。

**答案**：

**(a) 验证方法**

由 Zero-One Principle，要判断该网络是否为排序网络，只需检查所有 $2^4 = 16$ 种 0-1 输入序列，验证输出是否均为升序（所有 0 在前，所有 1 在后）。

**(b) 验证过程**

输入 1001，得到 0101，该网络非排序网络

---

## 参考答案速查

### 选择题

| 题号 | 答案 | 考点 |
|------|------|------|
| 1 | (C) | Sorter 深度 O(log² n) 而非 O(log n) |
| 2 | (A) | Zero-One Principle 的准确表述 |
| 3 | (A) | Half-Cleaner 对 0-1 双调序列的处理 |

### 问答题评分标准

| 子题 | 分值 | 评分要点 |
|------|------|----------|
| (1) | 5' | 分情形讨论（3'）+ 性质证明（2'） |
| (2) | 5' | 逆序拼接构造双调（2'）+ 比较方式解释（3'） |
| (3) | 5' | 递归树求解 O(log² n)（3'）+ 一般形式三种情形（2'） |

#### QA2 — Domain Conversion Lemma 与 Zero-One Principle

| 子题 | 分值 | 评分要点 |
|------|------|----------|
| (1) | 5' | 引理准确叙述（1'）+ 归纳证明结构完整（base 1' + inductive step 2'）+ 单调性使用正确（1'） |
| (2) | 5' | 定理准确叙述（1'）+ 反证法结构（1'）+ 构造单调函数 f（1'）+ 应用 Domain Conversion Lemma 导出矛盾（2'） |
| (3) | 5' | 16 种 0-1 序列穷举（2'）+ 验证过程正确（1'）+ 不能只检查一条的理由解释充分（2'） |
