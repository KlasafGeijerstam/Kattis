#include <iostream>

int main()
{
    std::ios_base::sync_with_stdio(false);
    unsigned long long s, p, y, j, d = 12, spot, puff, yertle;
    
    while (std::cin >> s >> p >> y >> j) {
        if ((d + j + s + y) % 3 == 0) {
            spot = (d + j + s + y) / 3;
            yertle = spot - y;
            puff = spot - s;
        }
        else if ((d + j + s + y) % 3 == 1) {
            spot = (d + j + s + y + 2) / 3;
            yertle = spot - y - 1;
            puff = spot - s - 1;
        }
        else {
            spot = (d + j + s + y + 1) / 3;
            yertle = spot - y;
            puff = spot - s;
            if (y == s + p + 1)
                puff--;
            else
                yertle--;
        }
        std::cout << spot << " " << puff << " " << yertle << std::endl;
    }
} 
