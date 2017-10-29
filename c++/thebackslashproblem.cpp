#include <iostream>
#include <string>

int main() {
    std::ios_base::sync_with_stdio(false);
    int c;
    std::string s;
    while (std::cin >> c) {
        std::getline(std::cin, s);
        std::getline(std::cin, s);

        int nslash = 0;
        for (int i = 0; i < c; i++)
            nslash = nslash * 2 + 1;

        for (const auto& c : s) {
            if ((c >= '!' && c <= '*') || (c >= '[' && c <= ']'))
                for (int i = 0; i < nslash; i++)
                    std::cout << '\\';
            std::cout << c;
        }
        std::cout << std::endl;
    }
}
