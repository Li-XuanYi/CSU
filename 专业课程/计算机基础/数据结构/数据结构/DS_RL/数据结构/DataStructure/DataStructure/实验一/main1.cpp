#include <iostream>
#include <vector>
#include <queue>
#include <limits>
#include <algorithm>
#include <utility>
using namespace std;

struct Edge {
    int to;
    double w;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n, m;
    if (!(cin >> n >> m)) {
        cerr << "Expected n and m." << endl;
        return 1;
    }

    vector<vector<Edge>> g(n + 1); // 1‑indexed adjacency list
    for (int i = 0; i < m; ++i) {
        int u, v; double w;
        cin >> u >> v >> w;
        g[u].push_back({v, w});
    }

    int src;
    cin >> src;
    if (src < 1 || src > n) {
        cerr << "Source vertex out of range." << endl;
        return 1;
    }

    const double INF = numeric_limits<double>::infinity();
    vector<double> dist(n + 1, INF);
    vector<int> prev(n + 1, -1);
    vector<char> visited(n + 1, 0);

    // Dijkstra using min‑heap (distance, vertex)
    dist[src] = 0.0;
    using State = pair<double, int>;
    priority_queue<State, vector<State>, greater<State>> pq;
    pq.push({0.0, src});

    while (!pq.empty()) {
        auto [d, u] = pq.top();
        pq.pop();
        if (visited[u]) continue;
        visited[u] = 1;

        for (const auto &e : g[u]) {
            int v = e.to;
            double alt = d + e.w;
            if (alt < dist[v]) {
                dist[v] = alt;
                prev[v] = u;
                pq.push({alt, v});
            }
        }
    }

    // Output results
    for (int v = 1; v <= n; ++v) {
        cout << "Destination: " << v << ", Distance: ";
        if (dist[v] == INF) {
            cout << "INF, Path: -\n";
            continue;
        }
        cout << dist[v] << ", Path: ";
        // reconstruct path s -> ... -> v
        vector<int> path;
        for (int cur = v; cur != -1; cur = prev[cur]) path.push_back(cur);
        reverse(path.begin(), path.end());
        for (size_t i = 0; i < path.size(); ++i) {
            if (i) cout << " -> ";
            cout << path[i];
        }
        cout << '\n';
    }

    return 0;
}
