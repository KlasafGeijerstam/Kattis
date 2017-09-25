#include <iostream>
#include <queue>
#include <algorithm>

int main()
{
    int f, s, g, u, d, steps[1000001], fin = -1;
    std::fill(steps, steps + 1000001, -1);
    std::cin >> f >> s >> g >> u >> d;
    std::queue<int> q;
    q.push(s);
    steps[s] = 0;
    while (!q.empty()) {
        int c = q.front();

        if (c == g) {
            fin = steps[c];
            break;
        }
        q.pop();
        if (c + u <= f && steps[c + u] < 0) {
            q.push(c + u);
            steps[c + u] = steps[c] + 1;
        }
        if (c - d > 0 && steps[c - d] < 0) {
            q.push(c - d);
            steps[c - d] = steps[c] + 1;
        }
    }
    if (fin == -1)
        std::cout << "use the stairs" << std::endl;
    else
        std::cout << fin << std::endl;
}
