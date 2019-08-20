#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <cmath>

struct Circle
{
    double x, y, r;
    std::string colour;

    bool hit(double px, double py)
    {
        return pow(px - x, 2) + pow(py - y, 2) <= r*r;
    }
};

double rFromV(double v)
{
    return sqrt(v / 3.14159265359);
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    int t;
    std::cin >> t;
    for (int i = 0; i < t; i++)
    {
        std::vector<Circle> circ;
        int d;
        double x, y, v;
        std::string col;
        std::cin >> d;
        for (int dr = 0; dr < d; dr++)
        {
            std::cin >> x >> y >> v >> col;
            circ.push_back({x, y, rFromV(v), col});
        }
        std::cin >> d;
        for (int dr = 0; dr < d; dr++)
        {
            std::cin >> x >> y;
            col = "white";
            for (int j = circ.size() - 1; j >= 0; j--)
            {
                if (circ[j].hit(x, y)) {
                    col = circ[j].colour;
                    break;
                }
            }
            std::cout << col << std::endl;
        }
    }
}