#include <iostream>
#include <string>
#include <algorithm>

std::string map[50];
bool bol[50][50];
int w, h, gold;

inline bool bounds(int x, int y)
{
    return !(x < 0 || x > w - 1 || y < 0 || y > h - 1);
}

inline bool draft(int x, int y)
{
    int mc;
    for (int xm = -1; xm < 2; xm++)
        for (int ym = -1; ym < 2; ym++)
        {
            mc = abs(xm) + abs(ym);
            if (mc != 1 || !bounds(x + xm, y + ym))
                continue;
            if (map[y + ym][x + xm] == 'T')
                return true;
        }
    return false;
}

void move(int x, int y)
{
    if (!bounds(x,y) || map[y][x] == '#' || map[y][x] == 'T')
        return;
    if (map[y][x] == 'G' && !bol[y][x])
        gold++;
    bol[y][x] = true;
    if (draft(x, y))
        return;
    int mc;
    for (int xm = -1; xm < 2; xm++)
        for (int ym = -1; ym < 2; ym++)
        {
            mc = abs(xm) + abs(ym);
            if (mc == 1 && !bol[y+ym][x+xm])
                move(x + xm, y + ym);
        }
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::cin >> w >> h;
    std::string p;
    int sx = -1, sy, s;
    for (int i = 0; i < h; i++) 
    {
        std::cin >> p;
        if (sx == -1 && (s = p.find('P', 0))) 
        {
            sy = i;
            sx = s;
        }
        map[i] = p;
    }
    move(sx, sy);
    std::cout << gold << std::endl;
}
