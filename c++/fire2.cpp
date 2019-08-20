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

    int cases, w, h, ps, found;
    std::cin >> cases;

    for (int cc = 0; cc < cases; cc++)
    {
        std::cin >> w >> h;
        std::string tmp;
        std::vector<std::vector<char>> map(h);
        for (int i = 0; i < h; i++) {
            std::cin >> tmp;
            map[i] = std::vector<char>(tmp.c_str(), tmp.c_str() + tmp.size() + 1);
        }

        std::queue<mapPos> q;
        int px, py;
        for (int x = 0; x < w; x++)
            for (int y = 0; y < h; y++)
            {
                if (map[y][x] == '*')
                    q.push({ x, y, '*', 0});
                else if (map[y][x] == '@') {
                    px = x;
                    py = y;
                }
            }
        
        q.push({ px, py, '@', 0});
        ps = 1;
        found = 0;
        while (ps > 0) {
            auto t = q.front();
            char p = map[t.y][t.x];
            q.pop();
            if (map[t.y][t.x] == '@')
                ps--;
            
            if (p == '@' && (t.x == w - 1 || t.x == 0 || t.y == h - 1 || t.y == 0)) {
                found = 1;
                std::cout << t.steps + 1 << std::endl;
                break;
            }

            if (t.x + 1 < w && map[t.y][t.x + 1] == '.') {
                q.push({ t.x + 1, t.y , t.ch, t.steps + 1});
                map[t.y][t.x + 1] = t.ch;
                if (t.ch == '@')
                    ps++;
            }
            if (t.x - 1 >= 0 && map[t.y][t.x - 1] == '.') {
                q.push({ t.x - 1, t.y , t.ch, t.steps + 1 });
                map[t.y][t.x - 1] = t.ch;
                if (t.ch == '@')
                    ps++;
            }
            if (t.y + 1 < h && map[t.y + 1][t.x] == '.') {
                q.push({ t.x, t.y + 1, t.ch, t.steps + 1 });
                map[t.y + 1][t.x] = t.ch;
                if (t.ch == '@')
                    ps++;
            }
            if (t.y - 1 >= 0 && map[t.y - 1][t.x] == '.') {
                q.push({ t.x, t.y - 1 , t.ch, t.steps + 1 });
                map[t.y - 1][t.x] = t.ch;
                if (t.ch == '@')
                    ps++;
            }
        }
        if (!found)
            std::cout << "IMPOSSIBLE" << std::endl;
    }
}