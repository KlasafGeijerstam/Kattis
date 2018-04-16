#include <iostream>
#include <unordered_set>
#include <string>

int main() {
    std::ios_base::sync_with_stdio(false);
    int p,n,l;
    std::cin >> p >> n;
    std::unordered_set<std::string> g;
    std::string s;
    for(auto i = 0; i < n; i++){
        std::cin >> s;
        if(g.emplace(s).second){
            p--;
            if(!p)
                l = i+1;           
        }
    }
    if(p)
        std::cout << "paradox avoided" << std::endl;
    else
        std::cout << l << std::endl;
}
