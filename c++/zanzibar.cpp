#include <iostream>

int main()
{
    std::ios_base::sync_with_stdio(false);
    int c;
    std::cin >> c;
    for (int i = 0; i < c; i++)
    {
        int n, prev, tot = 0;
        std::cin >> prev;
        while (true) {
            std::cin >> n;
            if (n == 0)
                break;
            if (n > prev * 2)
                tot += n - prev * 2;
            prev = n;
        }
        std::cout << tot << std::endl;
    }
} 
