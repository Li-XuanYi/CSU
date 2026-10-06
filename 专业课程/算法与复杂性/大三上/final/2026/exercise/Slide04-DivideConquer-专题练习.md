# CS2308《算法与复杂性》专题练习

> 本练习参照 2022 年高晓沨期末考试风格命制
> 专题：Divide and Conquer（分治）
> 难度来源：Slide04 + Lab01

---

## 一、选择题（3.5' × 10 = 35'）

**1.（Master Theorem, Slide04）** 对于递推关系 $T(n) = 5T(n/2) + O(n^2)$，根据 Master Theorem，其时间复杂度为：

- (A) $O(n^2)$
- (B) $O(n^2 \log n)$
- (C) $O(n^{\log_2 5})$
- (D) $O(n^{\log_5 2})$

**答案**：C **解析**：$a=5$, $b=2$, $d=2$。计算 $\log_b a = \log_2 5 \approx 2.32$。由于 $d = 2 < \log_2 5$，属于 Master Theorem 的 Case 3（$d < \log_b a$），故 $T(n) = O(n^{\log_b a}) = O(n^{\log_2 5})$。

---

**2.（Integer Multiplication, Slide04）** 在整数乘法的分治算法中，利用 Gauss 技巧将大整数乘法从 4 次递归乘法优化为 3 次。优化后的时间复杂度为：

- (A) $O(n^2)$
- (B) $O(n^{\log_2 3}) \approx O(n^{1.59})$
- (C) $O(n \log n)$
- (D) $O(n^{\log_3 2})$

**答案**：B **解析**：优化后的递推关系为 $T(n) = 3T(n/2) + O(n)$。由 Master Theorem，$a=3$, $b=2$, $d=1$，$d < \log_2 3$，故 $T(n) = O(n^{\log_2 3}) \approx O(n^{1.59})$。

---

**3.（Strassen's Algorithm, Slide04）** Strassen 矩阵乘法通过巧妙构造 7 个中间矩阵 $P_1 \sim P_7$ 来减少递归乘法次数。关于 Strassen 算法，下列哪项表述是**正确**的？

- (A) Strassen 算法将 $n \times n$ 矩阵乘法的时间复杂度从 $O(n^3)$ 降低到 $O(n^2)$
- (B) Strassen 算法的递推关系为 $T(n) = 7T(n/2) + O(n^2)$
- (C) Strassen 算法的递推关系为 $T(n) = 8T(n/2) + O(n^2)$
- (D) Strassen 算法在分解步骤中需要计算 8 个 $n/2$ 规模的子矩阵乘法

**答案**：B **解析**：Strassen 算法的核心是将 8 次递归乘法减少到 7 次（通过构造 $P_1$ 到 $P_7$），递推关系为 $T(n) = 7T(n/2) + O(n^2)$，解为 $O(n^{\log_2 7}) \approx O(n^{2.81})$。选项 A 错在不是 $O(n^2)$；C 是朴素分治矩阵乘法的递推式；D 错在 Strassen 只需要 7 次而不是 8 次。

---

**4.（Binary Search, Slide04）** 二分查找（Binary Search）的递推关系为 $T(n) = T(\lfloor n/2 \rfloor) + O(1)$。根据 Master Theorem，其时间复杂度为：

- (A) $O(\log n)$
- (B) $O(n)$
- (C) $O(n \log n)$
- (D) $O(1)$

**答案**：A **解析**：$a=1$, $b=2$, $d=0$。$\log_b a = \log_2 1 = 0$，$d = 0 = \log_b a$，属于 Master Theorem 的 Case 2（$d = \log_b a$），故 $T(n) = O(n^d \log n) = O(\log n)$。

---

**5.（Divide and Conquer 策略, Slide04）** 关于分治策略，下列哪项表述是**错误**的（选 FALSE）？

- (A) 分治策略将原问题分解为若干规模更小的子问题，递归求解子问题，再合并结果
- (B) 分治策略的关键在于划分方式、递归基和合并方法三个环节
- (C) 所有分治算法的时间复杂度都可以用 Master Theorem 求解
- (D) Merge Sort 是典型的分治算法，其递推关系为 $T(n) = 2T(n/2) + O(n)$

**答案**：C **解析**：Master Theorem 只能处理形如 $T(n) = aT(n/b) + f(n)$ 的递推关系。并非所有分治算法都符合这一形式，例如当子问题规模不同（如 $T(n) = T(n/3) + T(2n/3) + O(n)$）或 $a$ 不是常数时，Master Theorem 不适用，需要使用递归树法或代入法。

---

---

**6.（Recurrence Tree / Master Theorem 证明, Slide04）** 对于递推关系 $T(n) = 3T(n/4) + O(n)$，根据 Master Theorem，以下哪项是正确的？

- (A) $T(n) = O(n^{\log_4 3})$
- (B) $T(n) = O(n)$
- (C) $T(n) = O(n \log n)$
- (D) $T(n) = O(n^{\log_3 4})$

**答案**：B **解析**：$a=3$, $b=4$, $d=1$。计算 $\log_b a = \log_4 3 \approx 0.792$。由于 $d = 1 > \log_4 3$，属于 Case 1（$d > \log_b a$），故 $T(n) = O(n^d) = O(n)$。

---

**7.（Merge Sort, Slide04）** 在自底向上（Bottom-Up）的 MergeSort 算法中，Merge 操作对两个长度分别为 $m$ 和 $n$ 的有序数组进行合并。关于 Merge 过程中的比较次数，下列哪项表述是**正确**的？

- (A) 比较次数始终为 $m + n - 1$
- (B) 比较次数始终为 $\min(m, n)$
- (C) 比较次数至少为 $\min(m, n)$，至多为 $m + n - 1$
- (D) 比较次数至少为 $m + n - 1$，至多为 $m + n$

**答案**：C **解析**：幻灯片分析指出，Merge 的比较次数至少为 $\min(m, n)$（如当一个数组所有元素都小于另一个数组的第一个元素时），至多为 $m + n - 1$（如当两个数组元素交替排列时）。

---

**8.（Sorting Lower Bound, Slide04）** 基于决策树模型，基于比较的排序算法在最坏情况下的时间复杂度下界为：

- (A) $\Omega(n)$
- (B) $\Omega(n \log n)$
- (C) $\Omega(n^2)$
- (D) $\Omega(\log n)$

**答案**：B **解析**：排序算法的决策树有 $n!$ 个叶子节点。二叉树的高度至少为 $\log(n!) \approx n\log n - n\log e + O(\log n) = \Omega(n\log n)$（使用 Stirling 公式）。因此任何基于比较的排序算法在最坏情况下至少需要 $\Omega(n\log n)$ 次比较。

---

**9.（Gauss's Trick, Slide04）** Gauss 发现复数乘法 $(a+bi)(c+di)$ 可以用 3 次实数乘法完成，而不是 4 次。三个乘法分别是 $ac$, $bd$ 和 $(a+b)(c+d)$。$bc+ad$ 可以通过以下哪种方式计算？

- (A) $ac + bd$
- (B) $(a+b)(c+d) - ac - bd$
- (C) $(a+b)(c+d) + ac + bd$
- (D) $ac - bd$

**答案**：B **解析**：$(a+b)(c+d) = ac + ad + bc + bd$，所以 $bc + ad = (a+b)(c+d) - ac - bd$。这正是 Gauss 将 4 次乘法减少为 3 次的关键技巧。

---

**10.（Master Theorem Proof, Slide04）** 在 Master Theorem 的递归树证明中，第 $j$ 层共有 $a^{j-1}$ 个子问题，每个子问题的规模为 $n/b^{j-1}$。若每个子问题的划分/合并代价为 $O((n/b^{j-1})^d)$，则第 $j$ 层的总代价为：

- (A) $O(n^d) \times (a/b^d)^{j-1}$
- (B) $O(n^d) \times (a/b)^{j-1}$
- (C) $O(n^d) \times (a^d/b)^{j-1}$
- (D) $O(n) \times (a/b^d)^{j-1}$

**答案**：A **解析**：第 $j$ 层总代价 = $a^{j-1} \times O((n/b^{j-1})^d) = O(n^d) \times (a/b^d)^{j-1}$。总时间为 $O(n^d) \sum_{j=0}^{\log_b n} (a/b^d)^j$。当 $a/b^d < 1$（即 $d > \log_b a$）时，几何级数收敛于常数，故 $T(n) = O(n^d)$。

---

## 二、问答题（15'）

### 1. Tromino 棋盘覆盖问题（15'）

> 本题目参考 Lab01-Q4（Tromino Tiling Problem）

**问题描述**：一个 **tromino** 是由三个 $1 \times 1$ 方格组成的 L 形瓷砖。给定一个 $n \times n$（其中 $n = 2^k$，$k \ge 1$）的棋盘，其中有一个方格缺失。要求用 tromino 瓷砖覆盖棋盘上所有剩余方格，每个 tromino 可以任意旋转，且瓷砖之间不能重叠。

#### （1）分治算法设计（Master Theorem, 分治策略）（5'）

请描述如何用**分治策略**解决该问题。说明划分（Divide）、递归求解（Conquer）和合并（Combine）三个步骤。不需要提供伪代码。

**答案**：

**Divide**：将 $n \times n$ 棋盘平分为 4 个 $n/2 \times n/2$ 的象限。在棋盘正中心放置一个 tromino，使其恰好覆盖三个象限的角落各一格，而缺失方格所在的象限保持原样。这样每个象限都有一个"缺失"格。

**Conquer**：对每个 $n/2 \times n/2$ 的象限递归调用同样的 tromino 覆盖算法。递归基为 $n = 2$：$2 \times 2$ 棋盘缺一格时，剩余 3 格正好由一个 tromino 覆盖。

**Combine**：无需显式合并，各象限递归覆盖的结果自然构成整个棋盘的完整覆盖。

#### （2）复杂度分析（Master Theorem, Slide04）（5'）

写出该算法时间复杂度的递推关系，并用 Master Theorem 求解。

**答案**：

递推关系：$T(n) = 4T(n/2) + O(1)$

其中 $a = 4$, $b = 2$, $d = 0$。计算 $\log_b a = \log_2 4 = 2$。

由于 $d = 0 < 2 = \log_b a$，属于 Master Theorem 的 Case 3（$d < \log_b a$），故：

$$T(n) = O(n^{\log_b a}) = O(n^2)$$

即整个棋盘被完全覆盖，算法时间复杂度为 $O(n^2)$，与棋盘上的方格数成正比。

#### （3）正确性证明（归纳法）（5'）

请用数学归纳法证明该分治算法的正确性。

**答案**：

**归纳假设**：对任意 $n = 2^k$（$k \ge 1$），存在一个缺失方格的 $n \times n$ 棋盘可以被 tromino 完全覆盖。

**基例**：$k = 1$，即 $n = 2$。$2 \times 2$ 棋盘缺一格时，剩余 3 格恰好形成一个 L 形，可用一个 tromino 覆盖。假设成立。

**归纳步骤**：假设对 $n/2 = 2^{k-1}$ 的棋盘命题成立。考虑 $n = 2^k$ 的棋盘：

1. 将棋盘平分为 4 个 $n/2 \times n/2$ 的象限。其中一个象限包含原始缺失格。
2. 在棋盘中心放置一个 tromino，使其覆盖其他三个象限各一个角格。这样，每个象限现在都有一个"缺失格"（原始缺失或 tromino 覆盖产生）。
3. 由归纳假设，每个 $n/2 \times n/2$ 的象限都可以被 tromino 完全覆盖。
4. 四个象限的覆盖合在一起，加上中心的 tromino，即构成整个棋盘的完整覆盖。

由数学归纳法，对所有 $n = 2^k$（$k \ge 1$），命题成立。证毕。

---

### 2. 完美排列生成（Perfect Permutation, 分治）（15'）

> 本题目参考 2022 高晓沨期末考试 QA1(c)

**问题描述**：一个**完美排列（Perfect Permutation）**$\Pi$ 是整数 $1,2,\dots,n$ 的一个排列，满足以下条件：不存在下标 $i < k < j$ 使得 $2\Pi_k = \Pi_i + \Pi_j$。换言之，排列中不包含长度为 3 的等差数列（Arithmetic Progression）。

给定整数 $n$，请设计分治算法生成一个长度为 $n$ 的完美排列。

#### （1）算法设计（分治策略, Slide04）（5'）

请描述分治算法的核心思路——如何利用奇偶分离避免等差数列。说明为什么跨越左右两部分的三个数不可能构成等差数列。

**答案**：

**核心思路（奇偶分离）**：

将 $1$ 到 $n$ 的整数划分为奇数和偶数两部分：
- 将所有奇数放在排列的左半部分
- 将所有偶数放在排列的右半部分

**为什么跨越两部分的数不会构成等差数列？**

设 $i$ 在左半部分（奇数），$j$ 在右半部分（偶数），$k$ 可以是任意位置：
- $\Pi_i$ 是奇数，$\Pi_j$ 是偶数，所以 $\Pi_i + \Pi_j$ 是**奇数**
- 而 $2\Pi_k$ 总是**偶数**
- 因此 $2\Pi_k = \Pi_i + \Pi_j$ 不可能成立

所以任何三个跨越左右两部分的数都不可能构成等差数列。接下来只需要分别确保左半部分的奇数之间、右半部分的偶数之间也没有等差数列即可，这可以通过递归实现。

**关键观察**：奇数可写为 $2x-1$，偶数可写为 $2x$。如果序列 $x$ 没有等差数列，则 $2x-1$ 和 $2x$ 也都没有等差数列。因此可将规模为 $n$ 的问题递归转化为 $\lceil n/2\rceil$ 和 $\lfloor n/2\rfloor$ 的子问题。

#### （2）伪代码实现（5'）

请给出分治算法的伪代码，并分析时间复杂度。

**答案**：

```
算法：GENERATE-PERFECT-PERMUTATION(n)
输入：正整数 n
输出：1..n 的完美排列 Π[1..n]

1. if n == 1 then
2.     return [1]
3. 
4. left_size = ⌈n/2⌉
5. right_size = ⌊n/2⌋
6. 
7. L = GENERATE-PERFECT-PERMUTATION(left_size)
8. R = GENERATE-PERFECT-PERMUTATION(right_size)
9. 
10. Π = 长度为 n 的空数组
11. idx = 1
12. 
13. // 将左半部分映射为奇数
14. for each x in L do
15.     Π[idx] = 2 × x - 1
16.     idx = idx + 1
17. 
18. // 将右半部分映射为偶数
19. for each x in R do
20.     Π[idx] = 2 × x
21.     idx = idx + 1
22. 
23. return Π
```

**时间复杂度分析**：
- 每层递归中，合并操作需要 $O(n)$ 时间
- 递归树深度为 $\log n$（因为每次规模减半）
- 递推关系：$T(n) = T(\lceil n/2\rceil) + T(\lfloor n/2\rfloor) + O(n)$
- 解为 $T(n) = O(n\log n)$（递归树法：每层总代价为 $O(n)$，共 $\log n$ 层）

**空间复杂度分析**：
- 每层递归需要存储左右子排列，递归栈深度 $\log n$
- 总空间复杂度 $O(n\log n)$（可优化为 $O(n)$）

#### （3）正确性证明（归纳法）（5'）

请用数学归纳法证明该算法生成的排列 $\Pi$ 满足完美排列的定义。

**答案**：

**归纳假设**：对任意正整数 $n$，GENERATE-PERFECT-PERMUTATION$(n)$ 返回一个 $1$ 到 $n$ 的完美排列。

**基例**：$n=1$，排列 $[1]$ 显然不含任何长度为 3 的等差数列，假设成立。

**归纳步骤**：假设对所有小于 $n$ 的正整数命题成立。考虑 $n$：

算法递归生成 $L$（长度为 $\lceil n/2\rceil$ 的完美排列）和 $R$（长度为 $\lfloor n/2\rfloor$ 的完美排列），然后将 $L$ 映射为奇数放入左半部分，$R$ 映射为偶数放入右半部分。

任取三个元素 $\Pi_a, \Pi_b, \Pi_c$（$a < b < c$），分三种情况：

1. **三个元素都在左半部分**：它们都是奇数，即形如 $2x-1$。由归纳假设 $L$ 是完美排列，$x$ 中无等差数列，线性变换 $2x-1$ 不改变等差数列性质，故三个奇数间无等差数列。

2. **三个元素都在右半部分**：它们都是偶数，即形如 $2x$。同理，由 $R$ 是完美排列可推出三个偶数间无等差数列。

3. **元素跨越左右两部分**：左半为奇数，右半为偶数。
   - 若 $a,b$ 在左、$c$ 在右：$\Pi_a+\Pi_c$ = 奇 + 偶 = 奇数，$2\Pi_b$ = 偶数，不可能相等。
   - 若 $a$ 在左、$b,c$ 在右：同理，$\Pi_a+\Pi_c$ 为奇数，$2\Pi_b$ 为偶数，不可能相等。
   - 其他跨越情况同理，奇偶性差异保证 $2\Pi_b \ne \Pi_a+\Pi_c$。

因此，排列中不存在任何长度为 3 的等差数列，$\Pi$ 是完美排列。由数学归纳法，命题对所有 $n$ 成立。证毕。

---

### 3. Master Theorem 综合应用（15'）

> 参照 2022 高晓沨期末考试 QA1 风格命制

#### （1）默写 Master Theorem（5'）

请写出课堂上学习的 Master Theorem。设递推关系为 $T(n) = aT(n/b) + f(n)$，其中 $a > 0$, $b > 1$, $d \ge 0$。提示：若 $T(n) = aT(\lceil n/b\rceil) + O(n^d)$，则 ...

**答案**：

若 $T(n) = aT(\lceil n/b\rceil) + O(n^d)$，其中 $a > 0$, $b > 1$, $d \ge 0$，则：

$$T(n) = \begin{cases}
O(n^d) & \text{若 } d > \log_b a \quad \text{(多项式意义上的更大)}\\
O(n^d \log n) & \text{若 } d = \log_b a\\
O(n^{\log_b a}) & \text{若 } d < \log_b a
\end{cases}$$

**直观理解**：
- $d > \log_b a$：划分/合并的代价占主导，递归的代价被上层淹没
- $d = \log_b a$：每层代价相同，总代价 = 层数 $\times$ 每层代价
- $d < \log_b a$：递归的代价占主导，叶子节点数量超过合并代价

#### （2）递推判断与分析（5'）

考虑以下递推关系。判断它能否用（1）中的 Master Theorem 求解？若能，直接写出时间复杂度；若不能，请说明原因并给出求解方法。

$$T(n) = 2T(n/2) + O(n \log n)$$

**答案**：

**能**用 Master Theorem 求解。这里 $a = 2$, $b = 2$, $f(n) = O(n \log n)$。

但需要将 $f(n) = n\log n$ 与 $n^{\log_b a} = n^{\log_2 2} = n$ 比较。

$f(n) = n\log n = \Theta(n^{\log_b a} \log^k n)$ 其中 $k = 1$。这属于 Master Theorem 的 **Case 2 扩展形式**：

$$T(n) = \Theta(n^{\log_b a} \log^{k+1} n) = \Theta(n \log^2 n)$$

因此 $T(n) = \Theta(n \log^2 n)$。

**补充**：若标准形式的 Master Theorem 只有 $O(n^d)$ 的 $f(n)$，则 $n \log n$ 不符合三个标准 case 中任意一个的精确形式（它不比 $n^{1+\varepsilon}$ 大，也不比 $n^{1-\varepsilon}$ 小）。但教材中 Case 2 的推广形式 $f(n) = \Theta(n^{\log_b a} \log^k n)$ 可以直接覆盖此情况，结果为 $\Theta(n^{\log_b a} \log^{k+1} n)$。

#### （3）分治算法设计（5'）

给定一个长度为 $n$ 的整数数组 $A[1..n]$，其中 $n$ 为 2 的幂。设计一个分治算法，找出数组中的**最大子数组和**（Maximum Subarray Sum），即找到连续子数组 $A[i..j]$（$1 \le i \le j \le n$）使得其和最大。给出伪代码并分析时间复杂度。

**答案**：

**算法思路**：将数组从中间分为左右两半。最大子数组和要么完全在左半部分，要么完全在右半部分，要么跨越中点。前两种情况递归求解，第三种情况从中点向左右两侧扩展找到最大和。

```
算法：MAX-SUBARRAY(A, low, high)
输入：数组 A[low..high]
输出：最大子数组和

1. if low == high then
2.     return A[low]                    // 只有一个元素
3. mid = ⌊(low + high) / 2⌋
4. 
5. // 递归求解左右两半
6. left_sum = MAX-SUBARRAY(A, low, mid)
7. right_sum = MAX-SUBARRAY(A, mid+1, high)
8. 
9. // 求跨越中点的最大和
10. sum = 0; max_left = -∞
11. for i = mid downto low do           // 从中点向左扩展
12.     sum = sum + A[i]
13.     if sum > max_left then max_left = sum
14. 
15. sum = 0; max_right = -∞
16. for i = mid+1 to high do            // 从中点向右扩展
17.     sum = sum + A[i]
18.     if sum > max_right then max_right = sum
19. 
20. cross_sum = max_left + max_right
21. 
22. return max(left_sum, right_sum, cross_sum)
```

**时间复杂度分析**：

每个子问题将数组平分为两半，合并（求跨越中点最大和）需要 $O(n)$ 时间。递推关系为：

$$T(n) = 2T(n/2) + O(n)$$

由 Master Theorem：$a = 2$, $b = 2$, $d = 1$，$\log_b a = \log_2 2 = 1$，$d = \log_b a$，属于 Case 2：

$$T(n) = O(n^d \log n) = O(n \log n)$$

**空间复杂度**：$O(\log n)$（递归栈深度）。
