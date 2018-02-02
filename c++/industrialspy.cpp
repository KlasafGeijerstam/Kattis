#include <iostream>
#include <string>
#include <bitset>
#include <algorithm>
#include <unordered_set>
#include <cmath>
#include <vector>

std::vector<char> nums;
std::unordered_set<std::string> plebs;

void comb(std::vector<char> cur, int len) {

    //Check combination if prime    
    std::string k;
    char h = '0';
    for(int i = 0; i < len; i++) {
        h = std::max(cur[i], h);
        
        if(cur[i] != '0' || h > '0')
            k += cur[i];
    }
    plebs.emplace(k);
    
    if(len == nums.size())
        return;
        
    for(int i = 0; i < nums.size(); i++) {
        if(nums[i] == 10)
            continue;
            
        char c = nums[i];
        nums[i] = 10;
        cur[len] = c;
        comb(cur, len + 1);
        nums[i] = c;
    }
}

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::bitset<10000000> b;
    b[0] = 1;
    b[1] = 1;
    int n = 10000000;
    for(int i = 2; i*i < n + 1; i++) {
        for(int j = i*i; j <= 10000000; j += i) {
            b[j] = 1;
        }
    }

    int cass;
    std::string k;
    std::cin >> cass;
    while(cass--) {
        plebs.clear();
        nums.clear();
        int plup = 0;
        std::cin >> k;
        for(auto& c : k)
            nums.push_back(c);
        comb(*(new std::vector<char>(nums.size())), 0);
        
        for(auto &c : plebs) {
            if(c != "" && !b[std::stoi(c)])
                plup++;
        }
        std::cout << plup << std::endl;
    }   
}
