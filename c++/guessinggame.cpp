#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

int main() {
    std::ios_base::sync_with_stdio(false);
    while(true) {    
        int pep, min = -1, max = 1000000;
        std::string p;
        while(true) {
            std::cin >> pep;
            if(!pep)
                return 0;
            std::cin >> p >> p;          
            
            if(p == "high") {                
                max = std::min(max, pep);
            } else if(p == "low") {
                min = std::max(min, pep);
            } else {
                if(pep >= max || pep <= min)
                    std::cout << "Stan is dishonest" << std::endl;
                else                    
                    std::cout << "Stan may be honest" << std::endl;            
                break;
            }              
        }
    }    
}
