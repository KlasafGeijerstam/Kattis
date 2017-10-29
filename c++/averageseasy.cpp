#include <iostream>
#include <string>
#include <vector>

int main() {
    std::ios_base::sync_with_stdio(false);
    int ecoc, comc, c, cur;
    std::cin >> c;
    std::string s;
    //newline after cases
    std::getline(std::cin, s);
    while (c--) {
        unsigned long long eco = 0, com = 0;
        double ecoa = 0, coma = 0;
        std::vector<int> ls;
        //newline in the beginning
        std::getline(std::cin, s);
        std::cin >> comc >> ecoc;
        for (int i = 0; i < comc; i++) {
            std::cin >> cur;
            com += cur;
            ls.push_back(cur);
        }
        for (int i = 0; i < ecoc; i++) {
            std::cin >> cur;
            eco += cur;
        }
        std::getline(std::cin, s);
        coma = com / (double)comc;
        ecoa = eco / (double)ecoc;
        int count = 0;
        for (const auto& p : ls) {
            if (p < coma && p > ecoa)
                count++;
        }
        std::cout << count << std::endl;
    }
}
