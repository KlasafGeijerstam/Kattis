#include <iostream>
#include <vector>

std::vector<int> ing;
int z;
int com = 0;

void comb2(int i, int t) {
    if(i == z) {
        com += t == 67108863 ? 1 : 0;
        return;        
    }
    comb2(i+1, t | ing[i]);
    comb2(i+1, t);
}

int main() {
    std::ios_base::sync_with_stdio(false);
    int words;
    std::string w;
    std::cin >> words;
    
    while(words--) {
        std::cin >> w;
        int v = 0;
        for(const auto& c : w)
            v |= 1 << (c - 'a');
        ing.push_back(v);
    }
    z = ing.size() + 1;
    comb2(0,0); 
    
    std::cout << com/2 << std::endl; 
}
