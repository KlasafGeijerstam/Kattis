#include <iostream>
#include <string>
#include <algorithm>
#define DIST(y1,x1,l) (abs(x1-l.x) + abs(y1-l.y))
struct P {
    int x, y;
};

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::string m[4];
    int a = 0;
    P d[16]{ { 0,0 },{ 0,1 },{ 0,2 },{ 0,3 },{ 1,0 },{ 1,1 },{ 1,2 },{ 1,3 },{ 2,0 },{ 2,1 },{ 2,2 },{ 2,3 },{ 3,0 },{ 3,1 },{ 3,2 } };
    for (int i = 0; i < 4; i++)
        std::cin >> m[i];
    for (int y = 0; y < 4; y++)
        for (int x = 0; x < 4; x++)
            a += m[y][x] != '.' ? DIST(x, y, d[m[y][x] - 'A']) : 0;
    std::cout << a << std::endl;
}
