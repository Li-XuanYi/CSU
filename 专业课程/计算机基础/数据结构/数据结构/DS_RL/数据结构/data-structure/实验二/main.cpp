#include <iostream>
using namespace std;

// 节点结构体
struct StudentNode {
    int id;           // 学号
    double score;     // 成绩
    StudentNode *left, *right;

    StudentNode(int id_, double score_) : id(id_), score(score_), left(nullptr), right(nullptr) {}
};

// 二叉搜索树类（以id为关键字）
class StudentBST {
private:
    StudentNode *root;

    // 插入操作
    StudentNode* insert(StudentNode* node, int id, double score) {
        if (node == nullptr)
            return new StudentNode(id, score);

        if (id < node->id)
            node->left = insert(node->left, id, score);
        else if (id > node->id)
            node->right = insert(node->right, id, score);
        else
            node->score = score; // 如果学号已存在，更新成绩

        return node;
    }

    // 查找操作
    StudentNode* search(StudentNode* node, int id) {
        if (node == nullptr || node->id == id)
            return node;

        if (id < node->id)
            return search(node->left, id);
        else
            return search(node->right, id);
    }

    // 找到最小值节点
    StudentNode* findMin(StudentNode* node) {
        while (node && node->left)
            node = node->left;
        return node;
    }

    // 删除操作
    StudentNode* remove(StudentNode* node, int id) {
        if (node == nullptr)
            return node;

        if (id < node->id)
            node->left = remove(node->left, id);
        else if (id > node->id)
            node->right = remove(node->right, id);
        else {
            // 找到要删除的节点
            if (node->left == nullptr) {
                StudentNode* temp = node->right;
                delete node;
                return temp;
            }
            else if (node->right == nullptr) {
                StudentNode* temp = node->left;
                delete node;
                return temp;
            }
            // 有两个子节点
            StudentNode* temp = findMin(node->right);
            node->id = temp->id;
            node->score = temp->score;
            node->right = remove(node->right, temp->id);
        }
        return node;
    }

    // 中序遍历，打印全部学生
    void inorder(StudentNode* node) {
        if (!node) return;
        inorder(node->left);
        cout << "学号: " << node->id << " 成绩: " << node->score << endl;
        inorder(node->right);
    }

    // 销毁树
    void destroy(StudentNode* node) {
        if (!node) return;
        destroy(node->left);
        destroy(node->right);
        delete node;
    }

public:
    StudentBST() : root(nullptr) {}

    ~StudentBST() {
        destroy(root);
    }

    void insert(int id, double score) {
        root = insert(root, id, score);
    }

    bool search(int id, double &score) {
        StudentNode* res = search(root, id);
        if (res) {
            score = res->score;
            return true;
        }
        return false;
    }

    void remove(int id) {
        root = remove(root, id);
    }

    void printAll() {
        cout << "学生成绩列表：" << endl;
        inorder(root);
    }
};

// 测试主函数
int main() {
    StudentBST tree;
    // 初始化几个学生
    tree.insert(1001, 88.5);
    tree.insert(1002, 92.0);
    tree.insert(1003, 76.5);

    cout << "初始学生成绩列表：" << endl;
    tree.printAll();

    int cmd;
    do {
        cout << "请输入操作指令（1-查找 2-插入 3-删除 0-退出）：";
        cin >> cmd;
        if (cmd == 1) {
            int id;
            cout << "请输入要查找的学号: ";
            cin >> id;
            double score;
            if (tree.search(id, score))
                cout << "成绩为: " << score << endl;
            else
                cout << "未找到该学号的学生" << endl;
        } else if (cmd == 2) {
            int id;
            double score;
            cout << "请输入要插入的学号和成绩: ";
            cin >> id >> score;
            tree.insert(id, score);
        } else if (cmd == 3) {
            int id;
            cout << "请输入要删除的学号: ";
            cin >> id;
            tree.remove(id);
        }
        if (cmd != 0) {
            cout << "当前学生成绩列表：" << endl;
            tree.printAll();
        }
    } while (cmd != 0);
    return 0;
}