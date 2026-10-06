#include <iostream>
#include <vector>
#include <queue>
#include <stack>
#include <limits>

using namespace std;

// 邻接矩阵表示图
const int INF = numeric_limits<int>::max();

void dijkstra(const vector<vector<int>>& graph, int src, vector<int>& dist, vector<int>& prev) {
    int n = graph.size();
    dist.assign(n, INF);
    prev.assign(n, -1);
    vector<bool> visited(n, false);

    dist[src] = 0;
    // 小根堆，first为距离，second为顶点编号
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<>> pq;
    pq.push({0, src});

    while (!pq.empty()) {
        int u = pq.top().second;
        pq.pop();
        if (visited[u]) continue;
        visited[u] = true;

        for (int v = 0; v < n; ++v) {
            if (graph[u][v] != INF && !visited[v]) {
                if (dist[u] + graph[u][v] < dist[v]) {
                    dist[v] = dist[u] + graph[u][v];
                    prev[v] = u;
                    pq.push({dist[v], v});
                }
            }
        }
    }
}

void printPath(int v, const vector<int>& prev) {
    stack<int> path;
    while (v != -1) {
        path.push(v);
        v = prev[v];
    }
    while (!path.empty()) {
        cout << path.top();
        path.pop();
        if (!path.empty()) cout << " -> ";
    }
}

int main() {
    // 示例：4个城市，5条道路
    int n = 4, m = 5;
    vector<vector<int>> graph(n, vector<int>(n, INF));
    // 示例道路：0->1(2), 0->2(6), 1->2(3), 1->3(1), 2->3(1)
    graph[0][1] = 2;
    graph[0][2] = 6;
    graph[1][2] = 3;
    graph[1][3] = 1;
    graph[2][3] = 1;
    int src = 0; // 源点为0

    vector<int> dist, prev;
    dijkstra(graph, src, dist, prev);

    cout << "从源点" << src << "到各点的最短路径及长度:" << endl;
    for (int i = 0; i < n; ++i) {
        if (dist[i] == INF) {
            cout << "到" << i << "不可达" << endl;
        } else {
            cout << "到" << i << "的最短距离: " << dist[i] << "，路径: ";
            printPath(i, prev);
            cout << endl;
        }
    }
    return 0;
}
