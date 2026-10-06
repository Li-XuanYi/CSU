#include <iostream>
#include <vector>
#include <string>
#include <map>           // 用于存储字符频率和生成的编码
#include <queue>         // 用于优先队列（构建哈夫曼树）
#include <memory>        // 用于智能指针 (可选, 但推荐用于简化内存管理) - 这里用普通指针并手动管理
#include <iomanip>       // 包含 setw 等格式化输出

// --- 哈夫曼树节点结构 ---
struct HuffmanNode {
    char data;                 // 存储的字符 (叶子节点) 或特殊标记 (内部节点)
    unsigned frequency;        // 字符频率或子树频率总和
    HuffmanNode *left, *right; // 指向左右子节点的指针

    // 构造函数
    HuffmanNode(char data, unsigned frequency) : data(data), frequency(frequency), left(nullptr), right(nullptr) {}

    // 内部节点构造函数（无特定字符）
    HuffmanNode(unsigned frequency, HuffmanNode* l, HuffmanNode* r) : data('\0'), frequency(frequency), left(l), right(r) {} // 用 '\0' 标记内部节点

    // 判断是否为叶子节点
    bool isLeaf() const {
        return left == nullptr && right == nullptr;
    }
};

// --- 用于优先队列的比较结构 ---
// 让频率低的节点优先级更高 (最小堆)
struct CompareNodes {
    bool operator()(HuffmanNode* lhs, HuffmanNode* rhs) const {
        return lhs->frequency > rhs->frequency; // 注意是 > , 因为 priority_queue 默认是最大堆
    }
};

// --- 递归函数：生成哈夫曼编码 ---
// 遍历哈夫曼树，为每个叶子节点（字符）生成编码
void generateCodes(HuffmanNode* root, std::string currentCode, std::map<char, std::string>& huffmanCodes) {
    if (root == nullptr) {
        return;
    }

    // 如果是叶子节点，存储它的编码
    if (root->isLeaf()) {
        // 如果树只有一个节点（只有一个字符输入），给它一个默认编码 '0'
        if (currentCode.empty() && root->data != '\0') {
             huffmanCodes[root->data] = "0";
        } else if (root->data != '\0'){ // 确保不是空的内部节点（虽然理论上叶子总有 data）
            huffmanCodes[root->data] = currentCode;
        }
        return;
    }

    // 递归遍历左子树 (路径加 '0')
    generateCodes(root->left, currentCode + "0", huffmanCodes);

    // 递归遍历右子树 (路径加 '1')
    generateCodes(root->right, currentCode + "1", huffmanCodes);
}

// --- 递归函数：清理哈夫曼树（释放内存）---
void deleteTree(HuffmanNode* node) {
    if (node == nullptr) {
        return;
    }
    deleteTree(node->left);
    deleteTree(node->right);
    delete node;
}


int main() {
    // 1. 定义字符及其频率
    std::map<char, unsigned> freqMap;
    freqMap['a'] = 5;
    freqMap['b'] = 9;
    freqMap['c'] = 12;
    freqMap['d'] = 13;
    freqMap['e'] = 16;
    freqMap['f'] = 45;

    // 处理特殊情况：频率表为空
    if (freqMap.empty()) {
        std::cout << "Frequency map is empty. Cannot build Huffman tree." << std::endl;
        return 1;
    }

    // 2. 创建优先队列 (最小堆)
    std::priority_queue<HuffmanNode*, std::vector<HuffmanNode*>, CompareNodes> minHeap;

    // 3. 将所有字符作为叶子节点放入优先队列
    for (auto const& [key, val] : freqMap) {
        minHeap.push(new HuffmanNode(key, val));
    }

     // 处理特殊情况：只有一个字符
    HuffmanNode* root = nullptr;
    if (minHeap.size() == 1) {
        root = minHeap.top();
    } else {
        // 4. 构建哈夫曼树
        while (minHeap.size() > 1) {
            HuffmanNode* left = minHeap.top(); minHeap.pop();
            HuffmanNode* right = minHeap.top(); minHeap.pop();
            unsigned combinedFreq = left->frequency + right->frequency;
            HuffmanNode* newNode = new HuffmanNode(combinedFreq, left, right);
            minHeap.push(newNode);
        }
        root = minHeap.top();
    }

    // 6. 生成哈夫曼编码
    std::map<char, std::string> huffmanCodes;
    if (root != nullptr) {
        generateCodes(root, "", huffmanCodes);
    }

    // 7. 输出每个字符的频率和哈夫曼编码
    std::cout << "Huffman Codes & Frequencies:" << std::endl;
    std::cout << "------------------------------------" << std::endl;
    // 使用 iomanip 进行格式化对齐
    std::cout << std::setw(10) << std::left << "Character"
              << std::setw(12) << std::right << "Frequency"
              << std::setw(15) << std::right << "Huffman Code" << std::endl;
    std::cout << "------------------------------------" << std::endl;

    // 遍历原始频率表，保证输出顺序或按需排序
    // 这里直接按 map 默认的键（字符）排序输出
    for (auto const& [key, freq] : freqMap) { // 直接遍历原始 freqMap
        if (huffmanCodes.count(key)) { // 检查编码是否存在
            std::cout << std::setw(10) << std::left << std::string("'") + key + "'" // 输出 'char'
                      << std::setw(12) << std::right << freq                  // 输出频率
                      << std::setw(15) << std::right << huffmanCodes[key]     // 输出编码
                      << std::endl;
        } else {
            // 理论上如果字符在 freqMap 中，就应该有编码，除非是空 map 或只有一个元素且处理出错
             std::cout << std::setw(10) << std::left << std::string("'") + key + "'"
                       << std::setw(12) << std::right << freq
                       << std::setw(15) << std::right << "Error!" << std::endl;
        }
    }
    std::cout << "------------------------------------" << std::endl;

    // 8. 清理内存
    deleteTree(root);

    return 0;
}
