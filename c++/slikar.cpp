#include <iostream>
#include <queue>
#include <algorithm>
#include <vector>
#include <string>

struct mapPos {
    int x, y, ch, steps;
};

int main()
{
    std::ios_base::sync_with_stdio(false);

    int w, h, ps, found;
    std::queue<mapPos> q;
    int px, py, gx, gy;

    std::cin >> h >> w;
    std::string tmp;
    std::vector<std::vector<char>> map(h);
    for (int i = 0; i < h; i++) {
        std::cin >> tmp;
        map[i] = std::vector<char>(tmp.c_str(), tmp.c_str() + tmp.size() + 1);

        for (int x = 0; x < w; x++)
        {
            if (map[i][x] == '*')
                q.push({ x, i, '*', 0 });
            else if (map[i][x] == 'S') {
                px = x;
                py = i;
            }
            else if (map[i][x] == 'D') {
                gx = x;
                gy = i;
            }

        }
    }

    q.push({ px, py, 'S', 0 });
    ps = 1;
    found = 0;
    while (ps > 0) {
        auto t = q.front();
        char p = map[t.y][t.x];
        q.pop();
        if (map[t.y][t.x] == 'S')
            ps--;

        if (p == 'S' && t.y == gy && t.x == gx) {
            found = 1;
            std::cout << t.steps << std::endl;
            break;
        }

        if (t.x + 1 < w && (map[t.y][t.x + 1] == '.' || (t.ch == 'S' ? map[t.y][t.x + 1] == 'D' : false))) {
            q.push({ t.x + 1, t.y , t.ch, t.steps + 1 });
            map[t.y][t.x + 1] = t.ch;
            if (t.ch == 'S')
                ps++;
        }
        if (t.x - 1 >= 0 && (map[t.y][t.x - 1] == '.' || (t.ch == 'S' ? map[t.y][t.x - 1] == 'D' : false))) {
            q.push({ t.x - 1, t.y , t.ch, t.steps + 1 });
            map[t.y][t.x - 1] = t.ch;
            if (t.ch == 'S')
                ps++;
        }
        if (t.y + 1 < h && (map[t.y + 1][t.x] == '.' || (t.ch == 'S' ? map[t.y + 1][t.x] == 'D' : false))) {
            q.push({ t.x, t.y + 1, t.ch, t.steps + 1 });
            map[t.y + 1][t.x] = t.ch;
            if (t.ch == 'S')
                ps++;
        }
        if (t.y - 1 >= 0 && (map[t.y - 1][t.x] == '.' || (t.ch == 'S' ? map[t.y - 1][t.x] == 'D' : false))) {
            q.push({ t.x, t.y - 1 , t.ch, t.steps + 1 });
            map[t.y - 1][t.x] = t.ch;
            if (t.ch == 'S')
                ps++;
        }
    }
    if (!found)
        std::cout << "KAKTUS" << std::endl;
}
