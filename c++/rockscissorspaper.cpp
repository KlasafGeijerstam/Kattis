#include <iostream>
#include <string>
#include <algorithm>
#include <cstdlib>

int main()
{
    std::ios_base::sync_with_stdio(false);
    char** map,**next;
    int t, w, h, d;
    map = (char**)malloc(100 * sizeof(char*));
    next = (char**)malloc(100 * sizeof(char*));
    for (int l = 0; l < 100; l++) {
        map[l] = (char*)malloc(100 * sizeof(char));
        next[l] = (char*)malloc(100 * sizeof(char));
    }

    std::cin >> t;
    for (int i = 0; i < t; i++)
    {
        std::cin >> h >> w >> d;
        for (int p = 0; p < h; p++)
        {
            std::string l;
            std::cin >> l;
            for (int k = 0; k < w; k++) 
            {
                map[k][p] = l[k];
                next[k][p] = l[k];
            }
        }
        for (int k = 0; k < d; k++)
        {
            for (int x = 0; x < w; x++)
            {
                for (int y = 0; y < h; y++)
                {
                    for (int mx = -1; mx < 2; mx++)
                    {
                        for (int my = -1; my < 2; my++)
                        {
                            if (abs(mx) + abs(my) == 1 && x + mx >= 0 && x + mx < w && y + my >= 0 && y + my < h)
                            {
                                int ix = x + mx, iy = y + my;
                                if (map[x][y] == 'R' && map[ix][iy] == 'S')
                                    next[ix][iy] = 'R';
                                else if (map[x][y] == 'S' && map[ix][iy] == 'P')
                                    next[ix][iy] = 'S';
                                else if (map[x][y] == 'P' && map[ix][iy] == 'R')
                                    next[ix][iy] = 'P';
                            }
                        }
                    }
                }
            }
            for (int x = 0; x < w; x++)
                for (int y = 0; y < h; y++)
                    map[x][y] = next[x][y];
        }
        for (int y = 0; y < h; y++) 
        {
            for (int x = 0; x < w; x++)
                std::cout << map[x][y];
            std::cout << std::endl;
        }
        std::cout << std::endl;
    }
}
