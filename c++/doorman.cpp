#include <iostream>
#include <string>
#include <algorithm>
#include <stack>

int main() { 
    std::ios_base::sync_with_stdio(false);
    
    std::string l;
    std::stack<char> s;
    int x, p = 0, y = 0, w = 0, m = 0;
    
    std::cin >> x >> l;
    for(int i = l.length() - 1; i > -1; i--)    
        s.push(l[i]);
    
    while(!s.empty()) {
        if(s.top() == 'M' && std::abs(m+1 - w) <= x)
            m++;
        else if(s.top() == 'W' && std::abs(w+1-m) <= x)
            w++;
        else if(s.size() > 1 && !y) {            
            char k = s.top();
            s.pop();
            char h = s.top();
            s.pop();

            s.push(k);
            s.push(h);
            y = 1;
            
            continue;
        } 
        else
            break;
            
        y = 0;    
        s.pop();
        p++;
    }
    std::cout << p << std::endl;
}
