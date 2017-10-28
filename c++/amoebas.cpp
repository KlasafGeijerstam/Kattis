#include <string>
#include <iostream>

char map[100][100];
int m, n;

void flood(int x, int y) {
    if (x < 0 || y < 0 || x >= n || y >= m || map[x][y] != '#')
        return;

    map[x][y] = '!';

    for (int i = -1; i < 2; i++)
        for (int j = -1; j < 2; j++)
            flood(x + i, y + j);
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    int k = 0;
    std::cin >> m >> n;
    for (int i = 0; i < m; i++)
        for (int j = 0; j < n; j++)
            std::cin >> map[j][i];

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
            if (map[i][j] == '#') {
                flood(i, j);
                k++;
            }
        }
    }
    std::cout << k << std::endl;
}
