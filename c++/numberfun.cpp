#include <iostream>

int main()
{
    std::ios_base::sync_with_stdio(false);
    float cas, a, b, c;;
    std::cin >> cas;
    while (cas--) {
        std::cin >> a >> b >> c;

        if (a + b == c || b + a == c || a * b == c || a / b == c || b / a == c || a - b == c || b - a == c)
            std::cout << "Possible" << std::endl;
        else
            std::cout << "Impossible" << std::endl;
    }
} 
