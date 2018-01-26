#include <iostream>
#include <stdio.h>
#include <vector>

int main() {
    std::ios_base::sync_with_stdio(false);
    int cases, ppl;
    std::cin >> cases;
    while(cases--) {
        std::cin >> ppl;
        std::vector<int> peps(ppl);
        int sum = 0;
        while(ppl--) {
            std::cin >> peps[ppl];
            sum += peps[ppl];
        }
        double avrg = (1.0*sum) / peps.size();
        int plep = 0;
        for(const auto& p : peps)
            if(p > avrg)
                plep++;
        
        printf("%.3f%%\n", 100*((1.0*plep)/peps.size()));
    }
}
