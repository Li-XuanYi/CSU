#include <iostream>
#include <string>
#include <vector>

// --- 朴素字符串匹配（BF）算法 ---
// 在 text 中查找所有等于 pattern 的子串，返回它们的起始位置（0-based 索引）
std::vector<size_t> bf_search(const std::string& text, const std::string& pattern) {
    std::vector<size_t> result;
    size_t n = text.size();
    size_t m = pattern.size();
    if (m == 0 || n < m) {
        return result;  // 模式串为空或模式比文本长时，直接返回空
    }

    // 对每一个可能的对齐位置 i 进行比较
    for (size_t i = 0; i + m <= n; ++i) {
        size_t j = 0;
        // 从 pattern[0] 开始，一直比到 pattern[m-1]
        while (j < m && text[i + j] == pattern[j]) {
            ++j;
        }
        if (j == m) {
            // 如果 j 能跑到 m，说明完全匹配
            result.push_back(i);
        }
    }
    return result;
}

int main() {
    struct TestCase {
        std::string text;
        std::string pattern;
    } tests[] = {
        // 几个简单测试用例
        { "hello world", "lo" },
        { "aaaaa", "aa" },
        { "abracadabra", "abra" },
        { "abcdef", "gh" },        // 不存在
        { "abababab", "aba" }
    };

    for (size_t k = 0; k < sizeof(tests)/sizeof(tests[0]); ++k) {
        const auto& tc = tests[k];
        auto positions = bf_search(tc.text, tc.pattern);

        std::cout << "Test " << (k+1) << ": text = \"" << tc.text
                  << "\", pattern = \"" << tc.pattern << "\"\n";

        if (positions.empty()) {
            std::cout << "  No match found.\n";
        } else {
            std::cout << "  Match positions: ";
            for (size_t idx = 0; idx < positions.size(); ++idx) {
                std::cout << positions[idx]
                          << (idx + 1 < positions.size() ? ", " : "");
            }
            std::cout << "\n";
        }
        std::cout << std::string(40, '-') << "\n";
    }

    return 0;
}
