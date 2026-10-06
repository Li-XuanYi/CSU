#include <iostream>     // 用于标准输入输出 (cout)
#include <vector>       // 用于使用动态数组 std::vector
#include <random>       // 用于生成随机数
#include <chrono>       // 用于高精度计时
#include <algorithm>    // 用于 std::swap, std::is_sorted (可选的排序验证)
#include <iomanip>      // 用于 std::fixed, std::setprecision (格式化输出)

// --- 随机数据生成函数 ---
/**
 * @brief 生成一个包含随机整数的 vector
 * @param size vector 的大小 (元素数量)
 * @param minVal 随机数的最小值 (包含)
 * @param maxVal 随机数的最大值 (包含)
 * @return 包含随机整数的 std::vector<int>
 */
std::vector<int> generateRandomVector(int size, int minVal = 1, int maxVal = 10000) {
    std::vector<int> data;
    data.reserve(size); // 预先分配内存，提高效率

    std::random_device rd; // 从硬件获取一个随机数种子
    std::mt19937 gen(rd()); // 使用随机数种子初始化梅森旋转算法生成器
    std::uniform_int_distribution<> distrib(minVal, maxVal); // 定义整数均匀分布的范围

    for (int i = 0; i < size; ++i) {
        data.push_back(distrib(gen)); // 生成并添加随机数到 vector
    }
    return data;
}

// --- 排序算法实现 ---

// 1. 插入排序 (Insertion Sort)
/**
 * @brief 对 vector 进行原地插入排序
 * @param arr 待排序的 vector (会被修改)
 */
void insertionSort(std::vector<int>& arr) {
    int n = arr.size();
    // 从第二个元素开始 (下标为 1)
    for (int i = 1; i < n; ++i) {
        int key = arr[i]; // 当前待插入的元素
        int j = i - 1;    // 指向已排序部分的最后一个元素

        // 将已排序部分中大于 key 的元素向后移动一位
        // 直到找到 key 合适的插入位置或者到达数组开头
        while (j >= 0 && arr[j] > key) {
            arr[j + 1] = arr[j]; // 元素后移
            j = j - 1;           // 继续向前比较
        }
        arr[j + 1] = key; // 将 key 插入到正确的位置
    }
}

// 2. 选择排序 (Selection Sort)
/**
 * @brief 对 vector 进行原地选择排序
 * @param arr 待排序的 vector (会被修改)
 */
void selectionSort(std::vector<int>& arr) {
    int n = arr.size();
    // 遍历数组，每次从未排序部分选择最小元素放到已排序部分的末尾
    for (int i = 0; i < n - 1; ++i) {
        // 假设当前位置 i 的元素是未排序部分的最小值
        int min_idx = i;
        // 在未排序部分 (从 i+1 到 n-1) 查找真正的最小值
        for (int j = i + 1; j < n; ++j) {
            if (arr[j] < arr[min_idx]) {
                min_idx = j; // 更新最小值的下标
            }
        }
        // 如果最小值不是当前位置 i 的元素，则交换它们
        if (min_idx != i) {
             std::swap(arr[i], arr[min_idx]); // 使用 std::swap 进行交换
        }
    }
}

// 3. 快速排序 (Quick Sort)

// 快速排序辅助函数：划分 (Partition)
/**
 * @brief 对子数组进行划分操作 (Hoare 或 Lomuto 方式，这里是 Lomuto)
 * @param arr 待划分的 vector
 * @param low 子数组的起始下标
 * @param high 子数组的结束下标
 * @return 划分后枢轴 (pivot) 元素的最终下标
 */
int partition(std::vector<int>& arr, int low, int high) {
    int pivot = arr[high]; // 选择最后一个元素作为枢轴 (pivot)
    int i = (low - 1);    // i 指向小于 pivot 的区域的最后一个元素的下标

    // 遍历子数组 (从 low 到 high-1)
    for (int j = low; j <= high - 1; ++j) {
        // 如果当前元素小于或等于枢轴 (这里实现是小于)
        if (arr[j] < pivot) {
            i++; // 扩展小于 pivot 的区域
            std::swap(arr[i], arr[j]); // 将小于 pivot 的元素放到 i 指向的位置
        }
    }
    // 将枢轴元素放到正确的位置 (i+1)
    std::swap(arr[i + 1], arr[high]);
    return (i + 1); // 返回枢轴元素的最终下标
}

// 快速排序主函数 (递归实现)
/**
 * @brief 递归地对子数组进行快速排序
 * @param arr 待排序的 vector
 * @param low 子数组的起始下标
 * @param high 子数组的结束下标
 */
void quickSortRecursive(std::vector<int>& arr, int low, int high) {
    // 基本情况：如果子数组只有一个元素或为空，则不需要排序
    if (low < high) {
        // pi 是划分操作后枢轴的下标，arr[pi] 已经在最终排序位置
        int pi = partition(arr, low, high);

        // 分别对枢轴左边和右边的子数组进行递归排序
        quickSortRecursive(arr, low, pi - 1);  // 排序左子数组
        quickSortRecursive(arr, pi + 1, high); // 排序右子数组
    }
}

// 快速排序的入口函数 (包装器)
/**
 * @brief 对整个 vector 进行快速排序
 * @param arr 待排序的 vector (会被修改)
 */
void quickSort(std::vector<int>& arr) {
    if (arr.empty()) return; // 对空 vector 直接返回
    quickSortRecursive(arr, 0, arr.size() - 1); // 调用递归函数处理整个数组
}

// --- 实验运行和计时 ---
int main() {
    // 定义要测试的数据规模
    std::vector<int> dataSizes = {100, 500, 1000};
    // const int runsPerSize = 5; // 可选：为更稳定的结果，可以每个规模运行多次取平均值

    // 设置输出浮点数的格式，保留6位小数
    std::cout << std::fixed << std::setprecision(6);

    // 遍历不同的数据规模
    for (int size : dataSizes) {
        std::cout << "-----------------------------------------" << std::endl;
        std::cout << "测试数据规模: " << size << std::endl;
        std::cout << "-----------------------------------------" << std::endl;

        // 为每个规模生成一次原始随机数据
        std::vector<int> originalData = generateRandomVector(size);

        // --- 测试插入排序 ---
        std::vector<int> dataToSort_Ins = originalData; // 复制一份数据，避免影响其他排序测试
        auto start_ins = std::chrono::high_resolution_clock::now(); // 记录开始时间
        insertionSort(dataToSort_Ins);                              // 执行插入排序
        auto end_ins = std::chrono::high_resolution_clock::now();   // 记录结束时间
        std::chrono::duration<double> duration_ins = end_ins - start_ins; // 计算耗时
        std::cout << "插入排序耗时: " << duration_ins.count() << " 秒" << std::endl;
        // 可选：验证排序是否正确
        // if (!std::is_sorted(dataToSort_Ins.begin(), dataToSort_Ins.end())) {
        //     std::cerr << "错误: 插入排序失败!" << std::endl;
        // }


        // --- 测试选择排序 ---
        std::vector<int> dataToSort_Sel = originalData; // 复制一份数据
        auto start_sel = std::chrono::high_resolution_clock::now(); // 记录开始时间
        selectionSort(dataToSort_Sel);                              // 执行选择排序
        auto end_sel = std::chrono::high_resolution_clock::now();   // 记录结束时间
        std::chrono::duration<double> duration_sel = end_sel - start_sel; // 计算耗时
        std::cout << "选择排序耗时: " << duration_sel.count() << " 秒" << std::endl;
        // 可选：验证排序是否正确
        // if (!std::is_sorted(dataToSort_Sel.begin(), dataToSort_Sel.end())) {
        //     std::cerr << "错误: 选择排序失败!" << std::endl;
        // }

        // --- 测试快速排序 ---
        std::vector<int> dataToSort_Quick = originalData; // 复制一份数据
        auto start_quick = std::chrono::high_resolution_clock::now(); // 记录开始时间
        quickSort(dataToSort_Quick);                                  // 执行快速排序
        auto end_quick = std::chrono::high_resolution_clock::now();   // 记录结束时间
        std::chrono::duration<double> duration_quick = end_quick - start_quick; // 计算耗时
        std::cout << "快速排序耗时: " << duration_quick.count() << " 秒" << std::endl; // 输出耗时
        // 可选：验证排序是否正确
        // if (!std::is_sorted(dataToSort_Quick.begin(), dataToSort_Quick.end())) {
        //     std::cerr << "错误: 快速排序失败!" << std::endl;
        // }

        std::cout << std::endl; // 每个规模测试后空一行
    }

    return 0; // 程序正常结束
}
