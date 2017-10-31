#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

int main() {
    std::ios_base::sync_with_stdio(false);
    int c, t; 
    while(std::cin >> c) {
        if(!c)
            break;
        std::vector<int> l1, l2;        
        std::map<int,int> m;
        for(int i = 0; i < c; i++) {
            std::cin >> t;
            l1.push_back(t);
        }
        std::vector<int> lo(l1);
        for(int i = 0; i < c; i++) {
            std::cin >> t;
            l2.push_back(t);
        }

        std::sort(l1.begin(), l1.end());
        std::sort(l2.begin(), l2.end());
        for(int i = 0; i < c; i++)
            m[l1[i]] = l2[i];
        for(const auto& p : lo)
            std::cout << m[p] << std::endl;
        std::cout << std::endl;
    }
} 
