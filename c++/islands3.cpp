#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <sstream>
#include <unordered_set>
#include <map>

std::vector<std::string> m;
int r, c;

void flood(int x, int y) {
    if (x < 0 || y < 0 || x >= c || y >= r || m[y][x] == 'W' || m[y][x] == 'P')
        return;
    m[y][x] = 'P';
    flood(x + 1, y);
    flood(x - 1, y);
    flood(x, y + 1);
    flood(x, y - 1);
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::cin >> r >> c;
    std::string k;
    std::getline(std::cin, k);
    for (auto i = 0; i < r; i++) {
        std::getline(std::cin, k);
        m.push_back(k);
    }

    auto f = 0;
    for (auto i = 0; i < c; i++)
    {
        for (auto j = 0; j < r; j++)
        {
            if (m[j][i] == 'L') {
                f++;
                flood(i, j);
            }
        }
    }
    std::cout << f << std::endl;
}
