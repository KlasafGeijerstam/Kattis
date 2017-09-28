#include <iostream>
#include <string>

int main()
{
    std::ios_base::sync_with_stdio(false);
    int r, c, zr, zc;
    std::cin >> r >> c >> zr >> zc;
    for (int i = 0; i < r; i++)
    {
        std::string l;
        std::cin >> l;
        for (int y = 0; y < zr; y++)
        {
            for (const auto &d : l) {
                for (int z = 0; z < zc; z++)
                {
                    std::cout << d;
                }
            }
            std::cout << std::endl;
        }
    }
    std::cin >> r;
}

