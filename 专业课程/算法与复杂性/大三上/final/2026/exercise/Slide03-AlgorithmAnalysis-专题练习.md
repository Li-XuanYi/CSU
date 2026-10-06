# CS2308《算法与复杂性》Slide03 Algorithm Analysis 专题练习

> 本专题练习覆盖 Slide03 全部知识点：计算复杂性理论、渐近符号、复杂度分析方法、搜索与排序算法分析。
> 难度以讲义和 Lab01 实验题为基准。
> 全卷共 2 部分，满分 100 分（选择题 35' + 问答题 65'）

---

## Part 1. 选择题（共 10 题，每题 3.5 分，共 35 分）

---

**1.（Nested Loop Complexity, Slide03）** 以下代码的时间复杂度为多少？

```
count ← 0
for i ← 1 to n do
    j ← 2
    while j ≤ n do
        count ← count + 1
        j ← j²
```

- (A) $O(n)$
- (B) $O(n \log n)$
- (C) $O(n \log \log n)$
- (D) $O(n^2)$

**答案**：C **解析**：外层 for 循环执行 $n$ 次。对于每个 $i$，内层 while 循环中 $j$ 的取值序列为 $2, 2^2, 2^4, \dots, 2^{2^k}$，即执行次数满足 $2^{2^k} \le n$，得 $k = \log \log n$。因此内层循环执行 $\log \log n + 1$ 次。总复杂度为 $O(n \log \log n)$。（对应讲义 Count3 示例）

---

**2.（Asymptotic Notation Definition, Slide03）** 关于渐近符号的定义，下列说法正确的是：

- (A) $f(n) = O(g(n))$ 当且仅当存在 $c > 0$ 使得对所有 $n$ 有 $f(n) \le c \cdot g(n)$
- (B) $f(n) = \Omega(g(n))$ 当且仅当存在 $c > 0$ 和 $n_0 > 0$ 使得对所有 $n \ge n_0$ 有 $f(n) \ge c \cdot g(n)$
- (C) $f(n) = \Theta(g(n))$ 当且仅当 $f(n) = O(g(n))$ 或 $f(n) = \Omega(g(n))$
- (D) $f(n) = o(g(n))$ 当且仅当存在 $c > 0$ 和 $n_0 > 0$ 使得对所有 $n \ge n_0$ 有 $f(n) < c \cdot g(n)$

**答案**：B **解析**：A 缺少"存在 $n_0$，对所有 $n \ge n_0$"的条件；C 应为"且"而非"或"；D 中 $o$ 的定义要求"对任意 $c > 0$"，而非"存在 $c > 0$"。

---

**3.（Growth Rate Ordering, Slide03）** 将以下函数按渐近增长速度从小到大排列：

1. $\log n$　　2. $\sqrt{n}$　　3. $n$　　4. $n \log n$　　5. $n^2$　　6. $2^n$　　7. $n!$

- (A) $1 \to 2 \to 3 \to 4 \to 5 \to 6 \to 7$
- (B) $1 \to 3 \to 2 \to 4 \to 5 \to 6 \to 7$
- (C) $2 \to 1 \to 3 \to 4 \to 5 \to 6 \to 7$
- (D) $1 \to 2 \to 4 \to 3 \to 5 \to 6 \to 7$

**答案**：A **解析**：Complexity Classes: $1 \prec \log \log n \prec \log n \prec \sqrt{n} \prec n \prec n \log n \prec n^2 \prec 2^n \prec n! \prec 2^{n^2}$。因此 $\log n \prec \sqrt{n} \prec n \prec n \log n \prec n^2 \prec 2^n \prec n!$。

---

**4.（Best/Worst/Average Case, Slide03）** 下列关于算法复杂度分析的说法中，**错误**的是：

- (A) 默认情况下，不特别说明时通常指的是最坏情况时间复杂度
- (B) 平均情况分析需要假设输入的概率分布
- (C) 摊还分析（Amortized Analysis）也需要假设输入的概率分布
- (D) 最好情况分析给出的是算法运行时间的下界

**答案**：C **解析**：摊还分析是对操作序列取平均，保证最坏情况下每个操作的摊还代价，不需要假设输入的概率分布。这与平均情况分析不同——平均情况分析需要对输入分布做假设。

---

**5.（Input Size Concept, Slide03）** 算法接收一个正整数 $n$ 作为输入，其伪代码如下：

```
sum ← 0
for j ← 1 to n do
    sum ← sum + j
return sum
```

若以 $n$ 的二进制位数 $k$ 为输入规模度量，则该算法的时间复杂度为：

- (A) $O(k)$
- (B) $O(2^k)$
- (C) $O(k^2)$
- (D) $O(n)$

**答案**：B **解析**：输入 $n$ 的二进制位数 $k = \lfloor \log n \rfloor + 1$，即 $n \approx 2^k$。循环执行 $n \approx 2^k$ 次，因此时间复杂度为 $O(2^k)$，是指数时间。（对应讲义 Summation2 示例）

---

**6.（Space Complexity, Slide03）** 下列关于空间复杂度的说法中，正确的是：

- (A) 空间复杂度包含输入数据所占用的存储空间
- (B) 任何算法的空间复杂度不可能超过其时间复杂度
- (C) Merge Sort 的空间复杂度是 $O(\log n)$
- (D) 空间复杂度和时间复杂度之间不存在 trade-off 关系

**答案**：B **解析**：空间复杂度定义为算法执行所需的额外工作空间（不含输入空间），排除输入空间是为了使亚线性空间复杂度有意义。工作空间不可能超过运行时间，即 $S(n) = O(T(n))$。Merge Sort 需要 $O(n)$ 辅助空间，Quick Sort 递归栈空间为 $O(\log n)$。时间与空间复杂度之间存在 trade-off 关系。

---

**7.（Sorting Algorithm Comparison, Slide03）** 以下关于排序算法的说法中，**错误**的是：

- (A) Selection Sort 的最好、最坏、平均情况时间复杂度均为 $\Theta(n^2)$
- (B) Bubble Sort 在输入已经有序时时间复杂度为 $\Theta(n)$
- (C) Insertion Sort 在输入已经有序时时间复杂度为 $\Omega(n)$
- (D) Quick Sort 的最坏情况时间复杂度为 $O(n^2)$

**答案**：B **解析**：讲义中 Bubble Sort 的实现即使在输入有序时仍然会执行完整的双层循环，比较次数为 $n(n-1)/2$，因此最好情况也是 $\Theta(n^2)$。（但若加入提前终止优化，最好情况可变为 $O(n)$。）Insertion Sort 在输入有序时每个元素只需一次比较，总比较次数为 $n-1$，最好情况 $\Omega(n)$。Quick Sort 当每次 pivot 将数组分为 $1$ 和 $n-1$ 时达到最坏 $O(n^2)$。

---

**8.（Loop Iteration Counting, Slide03）** 以下算法中 `count ← count + 1` 语句的执行次数是：

```
count ← 0
for i ← 1 to n do
    m ← ⌊n / i⌋
    for j ← 1 to m do
        count ← count + 1
```

- (A) $\Theta(n)$
- (B) $\Theta(n \log n)$
- (C) $\Theta(n^2)$
- (D) $\Theta(\sqrt{n})$

**答案**：B **解析**：内层 for 循环执行次数为 $\lfloor n/i \rfloor$，$i$ 从 $1$ 到 $n$。总执行次数为 $\sum_{i=1}^{n} \lfloor n/i \rfloor \approx n \cdot H_n = \Theta(n \log n)$。（对应讲义 Count2 示例）

---

**9.（Binary Search Analysis, Slide03）** 在一个长度为 $n = 2^k - 1$ 的有序数组中进行二分查找（Binary Search），平均情况下的比较次数约为：

- (A) $n/2$
- (B) $\log n$
- (C) $n$
- (D) $n \log n$

**答案**：B **解析**：当 $n = 2^k - 1$ 时，二分查找的平均比较次数为 $\frac{1}{n} \cdot \sum_{i=1}^{k} i \cdot 2^{i-1} = \frac{k \cdot 2^k - 2^k + 1}{n} = 2\log n + \frac{1}{n}$，即 $O(\log n)$。对应讲义中 A.G.P. 求和计算。

---

**10.（Algorithm Analysis Terminology, Slide03）** 在算法分析中，"基本操作（Basic Operation）"是指：

- (A) 所有执行时间相同的操作
- (B) 算法中执行频率最高（至多常数因子以内）的基本元素操作
- (C) 算法中的赋值操作
- (D) 输入数据的读入操作

**答案**：B **解析**：基本操作（Basic Operation）定义为在算法中执行频率最高的基本元素操作（至多常数因子以内）。例如排序算法中选比较操作，矩阵乘法中选标量乘法，链表遍历中选指针更新操作。

---

## Part 2. 问答题（共 4 题，共 65 分）

---

### 一、Asymptotic Notation & Complexity Class（15 分）

#### （1）定义与等价条件（Asymptotic Definition, Slide03）（5'）

用极限形式分别给出 $O(\cdot)$、$\Omega(\cdot)$、$\Theta(\cdot)$、$o(\cdot)$、$\omega(\cdot)$ 的等价判定条件（假设极限存在）。

**答案**：设 $\lim_{n \to \infty} f(n)/g(n)$ 存在，则：

- $\lim f(n)/g(n) \ne \infty \Rightarrow f(n) = O(g(n))$
- $\lim f(n)/g(n) \ne 0 \Rightarrow f(n) = \Omega(g(n))$
- $\lim f(n)/g(n) = c$（常数 $0 < c < \infty$）$\Rightarrow f(n) = \Theta(g(n))$
- $\lim f(n)/g(n) = 0 \Rightarrow f(n) = o(g(n))$
- $\lim f(n)/g(n) = \infty \Rightarrow f(n) = \omega(g(n))$

#### （2）函数排序（Complexity Ordering, Slide03）（5'）

将以下函数按渐近增长速度从小到大排序（用"$\prec$"连接），并简要说明理由：

$f_1(n) = \lg n$（以 10 为底），$f_2(n) = \log^2 n$（以 2 为底），$f_3(n) = 2^{\sqrt{\log n}}$，$f_4(n) = \sqrt{n}$，$f_5(n) = n$，$f_6(n) = n^2$，$f_7(n) = 2^n$，$f_8(n) = n!$

其中 $\log n$ 表示以 2 为底的对数。

**答案**：

取 $\log n = \log_2 n$，$\lg n = \log_{10} n$。

$\lg n = \log_{10} n = \frac{\log_2 n}{\log_2 10} = \Theta(\log n)$，因此 $f_1 = \Theta(\log n)$。

排序结果：
$f_1 \prec f_2 \prec f_3 \prec f_4 \prec f_5 \prec f_6 \prec f_7 \prec f_8$

理由：
- $f_1 = \Theta(\log n)$，$f_2 = \Theta(\log^2 n)$，$\log n \prec \log^2 n$
- $f_2$ 与 $f_3$：对 $f_3$ 取对数得 $\sqrt{\log n}$，对 $f_2$ 取对数得 $2 \log \log n$。由于 $\sqrt{\log n} \succ \log \log n$，故 $f_2 \prec f_3$
- $f_3$ 与 $f_4$：令 $n = 2^{2^k}$，则 $f_3 = 2^{2^{k/2}}$，$f_4 = 2^{2^{k-1}}$。当 $k$ 充分大时 $2^{k-1} \gg 2^{k/2}$，故 $f_3 \prec f_4$
- $f_4 = \sqrt{n} \prec n = f_5 \prec n^2 = f_6 \prec 2^n = f_7 \prec n! = f_8$

#### （3）极限法证明（Limit Method, Slide03）（5'）

使用极限定义法证明：$\log(n!) = \Theta(n \log n)$。

**答案**：

利用 Stirling 公式：$n! \approx \sqrt{2\pi n}(n/e)^n$，或利用放缩法：

**上界**：$\log(n!) = \sum_{i=1}^{n} \log i \le \sum_{i=1}^{n} \log n = n \log n$，所以 $\log(n!) = O(n \log n)$。

**下界**：$\log(n!) = \sum_{i=1}^{n} \log i \ge \sum_{i=n/2}^{n} \log i \ge \frac{n}{2} \cdot \log\frac{n}{2} = \frac{n}{2}(\log n - 1) = \Omega(n \log n)$。

由上下界得 $\log(n!) = \Theta(n \log n)$。

**极限法**：
$$\lim_{n \to \infty} \frac{\log(n!)}{n \log n} = \lim_{n \to \infty} \frac{\sum_{i=1}^{n} \log i}{n \log n}$$

由定积分逼近：$\sum_{i=1}^{n} \log i = \int_1^n \log x \, dx + O(\log n) = [x \log x - x]_1^n + O(\log n) = n \log n - n + O(\log n)$

因此 $\lim_{n \to \infty} \frac{\log(n!)}{n \log n} = \lim_{n \to \infty} \frac{n \log n - n + O(\log n)}{n \log n} = 1$，故 $\log(n!) = \Theta(n \log n)$。

---

### 二、Loop Iteration Counting（16 分）

#### （1）循环计数分析（Counting Iterations, Slide03）（8'）

分析以下算法中 `count ← count + 1` 语句的执行次数，给出 $\Theta$ 界。

```
Algorithm: CountA(n)
Input: n = 2^k, for some positive integer k
Output: count = number of times Step 4 is executed

count ← 0
while n ≥ 1 do
    for j ← 1 to n do
        count ← count + 1
    n ← n / 2
return count
```

**答案**：

while 循环执行 $k+1$ 次（$n$ 取值依次为 $2^k, 2^{k-1}, \dots, 2, 1$，然后 $0.5$ 退出）。

对于每次 while 迭代，内层 for 循环执行 $n$ 次。因此总执行次数为：

$$T(n) = \sum_{j=0}^{k} \frac{n}{2^j} = n \cdot \sum_{j=0}^{k} \left(\frac{1}{2}\right)^j = n \cdot \left(2 - \frac{1}{2^k}\right) = 2n - 1$$

由于 $n = 2^k$，得 $T(n) = 2n - 1 = \Theta(n)$。（对应讲义 Count1 示例）

#### （2）递推循环分析（Nested Loop with Dependent Bounds, Slide03）（8'）

分析以下算法的时间复杂度，给出 $\Theta$ 界。

```
Algorithm: CountB(n)
Input: A positive integer n
Output: count = number of times Step 4 is executed

count ← 0
for i ← 1 to n do
    j ← i
    while j ≤ n do
        count ← count + 1
        j ← j + i
return count
```

**答案**：

外层 for 循环 $i$ 从 $1$ 到 $n$。对于每个 $i$，内层 while 循环中 $j$ 的取值为 $i, 2i, 3i, \dots, \lfloor n/i \rfloor \cdot i$，共执行 $\lfloor n/i \rfloor$ 次。

总执行次数为 $\sum_{i=1}^{n} \lfloor n/i \rfloor$。

由调和级数逼近：
$$\sum_{i=1}^{n} \left\lfloor \frac{n}{i} \right\rfloor = n \cdot H_n - O(n) = n(\ln n + \gamma) - O(n) = \Theta(n \log n)$$

因此时间复杂度为 $\Theta(n \log n)$。

---

### 三、Sorting Algorithm Analysis（18 分）

#### （1）Insertion Sort 分析（Insertion Sort, Slide03）（9'）

针对 Insertion Sort 算法，回答以下问题：

```
Algorithm: InsertionSort(A[·])
Input: An array A[1, · · · , n] of n elements
Output: A[1, · · · , n] sorted in nondecreasing order

for i ← 2 to n do
    x ← A[i]
    j ← i - 1
    while j > 0 and A[j] > x do
        A[j + 1] ← A[j]
        j ← j - 1
    A[j + 1] ← x
```

（a）分析最好情况、最坏情况下的比较次数，并说明各自何时出现。（3'）

（b）假设输入 $A[1, \dots, n]$ 包含 $1$ 到 $n$ 的排列，且所有 $n!$ 种排列等概率出现。求平均情况下插入第 $i$ 个元素 $A[i]$ 时所需的期望比较次数。（3'）

（c）基于（b）的结果，推导 Insertion Sort 平均情况的总比较次数，并给出其渐近复杂度。（3'）

**答案**：

（a）**最好情况**：数组已升序排列。对于每个 $i$，$A[i-1] \le A[i]$，while 循环条件 $A[j] > x$ 立即不成立，每次只需 $1$ 次比较。总比较次数 $= n - 1 = \Omega(n)$。

**最坏情况**：数组已降序排列。对于每个 $i$，$A[i]$ 需要一直移动到数组最前端。第 $i$ 个元素需要 $i - 1$ 次比较。总比较次数 $= \sum_{i=2}^{n} (i - 1) = \frac{n(n-1)}{2} = O(n^2)$。

（b）插入第 $i$ 个元素时，其正确位置 $j$ 以等概率 $1/i$ 取 $1, 2, \dots, i$ 中的任意值。

- 当 $j = 1$ 时（插入到最前面），需要 $i - 1$ 次比较
- 当 $j \ge 2$ 时，需要 $i - j + 1$ 次比较

期望比较次数为：
$$\begin{aligned}
E_i &= \frac{i-1}{i} + \sum_{j=2}^{i} \frac{i - j + 1}{i} \\
&= \frac{i-1}{i} + \sum_{k=1}^{i-1} \frac{k}{i} \\
&= \frac{i-1}{i} + \frac{(i-1)i}{2i} \\
&= \frac{i}{2} - \frac{1}{i} + \frac{1}{2}
\end{aligned}$$

（c）总期望比较次数为：
$$E_{\text{total}} = \sum_{i=2}^{n} \left(\frac{i}{2} - \frac{1}{i} + \frac{1}{2}\right) = \frac{n^2}{4} + \frac{3n}{4} - \sum_{i=1}^{n} \frac{1}{i}$$

其中 $\sum_{i=1}^{n} 1/i = H_n = \Theta(\log n)$，因此平均情况复杂度为 $O(n^2)$。

#### （2）排序算法综合对比（Sorting Comparison, Slide03）（9'）

完成下表，填写各排序算法在最好、最坏、平均情况下的时间复杂度以及空间复杂度（用 $O$、$\Omega$ 或 $\Theta$ 表示）：

| 算法 | 最好情况 | 最坏情况 | 平均情况 | 空间复杂度 |
|------|----------|----------|----------|------------|
| Selection Sort | | | | |
| Bubble Sort | | | | |
| Insertion Sort | | | | |
| Merge Sort | | | | |
| Quick Sort | | | | |

**答案**：

| 算法 | 最好情况 | 最坏情况 | 平均情况 | 空间复杂度 |
|------|----------|----------|----------|------------|
| Selection Sort | $\Theta(n^2)$ | $\Theta(n^2)$ | $\Theta(n^2)$ | $O(1)$ |
| Bubble Sort | $\Theta(n^2)$ | $\Theta(n^2)$ | $\Theta(n^2)$ | $O(1)$ |
| Insertion Sort | $\Omega(n)$ | $O(n^2)$ | $O(n^2)$ | $O(1)$ |
| Merge Sort | $\Theta(n \log n)$ | $\Theta(n \log n)$ | $\Theta(n \log n)$ | $O(n)$ |
| Quick Sort | $\Omega(n \log n)$ | $O(n^2)$ | $O(n \log n)$ | $O(\log n)$ |

说明：
- Selection Sort 和 Bubble Sort 始终执行完整的双层循环，不依赖输入顺序。
- Insertion Sort 在输入有序时只需线性时间，但平均和最坏为平方级。
- Merge Sort 始终先分后合，无论输入如何都需要 $\Theta(n \log n)$。
- Quick Sort 最好情况是每次 pivot 平分数组，最坏情况是每次分为 $1$ 和 $n-1$。平均情况通过递推可得 $T(n) = O(n \log n)$。空间复杂度来自递归栈。

---

### 四、Algorithm Design & Complexity Analysis（16 分）

#### （1）算法设计（Searching Problem, Slide03）（5'）

假设你有一个 $n \times n$ 的二维数组 $A$，其每一行均已按非递减顺序排列，每一列也按非递减顺序排列（即 $A[i][j] \le A[i][j+1]$ 且 $A[i][j] \le A[i+1][j]$）。设计一个算法判断给定值 $x$ 是否存在于数组中，要求时间复杂度为 $O(n)$。给出伪代码。

**答案**：

利用矩阵的"杨氏矩阵（Young Tableau）"性质，从右上角开始搜索：

```
Algorithm: SearchYoung(A[·][·], n, x)
Input: n × n 矩阵 A，行列均非递减；目标值 x
Output: 若 x 存在返回 true，否则返回 false

i ← 1
j ← n
while i ≤ n and j ≥ 1 do
    if A[i][j] = x then
        return true
    else if A[i][j] > x then
        j ← j - 1    // 排除当前列
    else
        i ← i + 1    // 排除当前行
return false
```

正确性：从右上角开始，若当前元素大于 $x$，则 $x$ 不可能在当前列中（因为该列向下递增），故向左移动；若当前元素小于 $x$，则 $x$ 不可能在当前行中（因为该行向左递减），故向下移动。每一步排除一行或一列，最多 $2n$ 步。

#### （2）复杂度分析（Complexity Analysis, Slide03）（5'）

分析上述算法的时间复杂度和空间复杂度，并说明理由。

**答案**：

**时间复杂度**：$O(n)$。每次迭代要么 $i$ 增加 $1$，要么 $j$ 减少 $1$。$i$ 从 $1$ 到 $n$（最多 $n$ 次），$j$ 从 $n$ 到 $1$（最多 $n$ 次），因此最多执行 $2n$ 次迭代。每次迭代为常数时间，故总复杂度为 $O(n)$。

**空间复杂度**：$O(1)$。只使用了常数个额外变量（$i, j$），不依赖输入规模。

#### （3）正确性证明（Proof of Correctness, Slide03）（6'）

用循环不变式（Loop Invariant）证明上述算法的正确性。

**答案**：

**循环不变式**：每次迭代开始时，目标值 $x$ 不可能出现在 $A[1..i-1][j+1..n]$ 区域（即已排除区域的并集），也不在 $A[i..n][1..j]$ 区域之外。

具体地，维护以下两个不变性质：
1. 若 $x$ 存在于矩阵中，则 $x$ 一定位于子矩阵 $A[i..n][1..j]$ 中（即当前搜索区域的左下部分）
2. $A[1..n][j+1..n]$ 区域中的所有元素均 $> x$
3. $A[1..i-1][1..j-1]$ 区域中的所有元素均 $< x$

**初始化**：$i = 1, j = n$，搜索区域为整个矩阵 $A[1..n][1..n]$，不变式平凡成立。

**保持**：假设进入循环时不变式成立。比较 $A[i][j]$ 与 $x$：
- 若 $A[i][j] = x$，算法正确返回 true。
- 若 $A[i][j] > x$，则由于列单调递增，$A[i][j]$ 及其正下方的所有元素均 $> x$，因此 $x$ 不可能在第 $j$ 列中。令 $j \leftarrow j - 1$，缩小搜索区域。
- 若 $A[i][j] < x$，则由于行单调递增，$A[i][j]$ 及其左侧的所有元素均 $< x$，因此 $x$ 不可能在第 $i$ 行中。令 $i \leftarrow i + 1$，缩小搜索区域。

两种情况下不变式均保持成立。

**终止**：当 $i > n$ 或 $j < 1$ 时循环终止，搜索区域为空，此时不变式表明 $x$ 不在矩阵中，算法正确返回 false。因此算法正确。$\square$

---

## 参考答案速查

### 选择题

| 题号 | 答案 | 考点 |
|------|------|------|
| 1 | C | Nested Loop Complexity（Count3 类型） |
| 2 | B | Asymptotic Notation 定义辨析 |
| 3 | A | Growth Rate Ordering |
| 4 | C | Best/Worst/Average/Amortized 概念 |
| 5 | B | Input Size 度量 |
| 6 | B | Space Complexity |
| 7 | B | Sorting Algorithm 复杂度对比 |
| 8 | B | Loop Iteration Counting（Count2 类型） |
| 9 | B | Binary Search Average Case |
| 10 | B | Basic Operation 定义 |

### 问答题

| 题号 | 主题 | 分值 | 关键结论 |
|------|------|------|----------|
| 一(1) | 极限判定 | 5' | O: $\lim \ne \infty$, $\Omega$: $\lim \ne 0$, $\Theta$: $\lim = c$, $o$: $\lim = 0$, $\omega$: $\lim = \infty$ |
| 一(2) | 函数排序 | 5' | $\lg n \prec \log^2 n \prec 2^{\sqrt{\log n}} \prec \sqrt{n} \prec n \prec n^2 \prec 2^n \prec n!$ |
| 一(3) | 极限证明 | 5' | $\log(n!) = \Theta(n \log n)$ |
| 二(1) | Count1 分析 | 8' | $T(n) = 2n - 1 = \Theta(n)$ |
| 二(2) | 调和级数和 | 8' | $\sum \lfloor n/i \rfloor = \Theta(n \log n)$ |
| 三(1) | InsertionSort | 9' | 最好 $\Omega(n)$，最坏 $O(n^2)$，平均 $O(n^2)$ |
| 三(2) | 综合对比 | 9' | 见上表 |
| 四(1) | 算法设计 | 5' | 右上角搜索，$O(n)$ |
| 四(2) | 复杂度分析 | 5' | $T(n) = O(n)$, $S(n) = O(1)$ |
| 四(3) | 循环不变式 | 6' | 搜索区域逐步缩小 |
