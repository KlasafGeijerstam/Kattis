#include <iostream>
#include <vector>
#include <algorithm>

struct Ingredient{
    int s,b;
};

std::vector<Ingredient> ing;
int min = 2000000000;

void comb(int len, int start, std::vector<Ingredient> res) {
    if(!len) {
        int s = 1, b = 0;
        for(const auto& i : res) {
            s *= i.s;
            b += i.b;
        }
        min = std::min(min, std::abs(s - b));
        return;
    }
    for(int i = start; i <= ing.size() - len; i++) {
        res[res.size() - len] = ing[i];
        comb(len-1, i+1, res);
    } 
}

int main() {
    std::ios_base::sync_with_stdio(false);
    int c, s, b;
    std::cin >> c;
    
    for(int i = 0; i < c; i++) {
        std::cin >> s >> b;
        ing.push_back({s,b});    
    }
    for(int i = 1; i <= c; i++) {
        comb(i, 0, *(new std::vector<Ingredient>(i)));
    }
    std::cout << min << std::endl;
}
