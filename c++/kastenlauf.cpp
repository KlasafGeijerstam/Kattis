#include <iostream>
#include <string>
#include <algorithm>
#include <vector>
#define DIST(p1,p2) (abs(p1.x-p2.x) + abs(p1.y-p2.y))

struct P {
    int x, y;
    bool v;
};
bool operator==(const P& lhs, const P& rhs) 
{ 
    return lhs.x == rhs.x && lhs.y == rhs.y; 
}

bool found;
std::vector<P> p;

void move(int k)
{
    if (found)
        return;
    p[k].v = true;
    if (p[k] == p[p.size() - 1])
    {
        found = true;
        return;
    }
    for (int c = 0; c < p.size(); c++)
        if (!p[c].v && DIST(p[k], p[c]) / 50.0 <= 20)
            move(c);
}

int main()
{
    int t,n,b,x,y;
    std::cin >> t;
    for (int i = 0; i < t; i++)
    {
        std::cin >> n;
        b = 20;
        found = false;
        std::cin >> x >> y;
        p.push_back({ x, y, false }); //Home
        for (int j = 0; j < n; j++)
        {
            std::cin >> x >> y;
            p.push_back({ x, y, false });
        }
        std::cin >> x >> y;
        p.push_back({ x, y, false }); //Bergkirchweih
        move(0);
        std::cout << (found ? "happy" : "sad") << std::endl;
        p.clear();
    }
}
