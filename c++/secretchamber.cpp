#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <sstream>
#include <unordered_set>
#include <map>
#include <string.h>
using namespace std;
map<char, unordered_set<char>*> trans;
unordered_set<char> visited;

bool find(char c, char s) {
    if(c == s)
        return true;
    visited.emplace(c);
    if(trans[c] == NULL)
        return false;
    for(auto& i : *trans[c])
        if(visited.find(i) == visited.end() && find(i, s))
            return true;
    return false;
}

int main()
{
    ios_base::sync_with_stdio(false);
    int n, m;
    string c, p;
    cin >> n >> m;
    
    while (n--) {
        cin >> c >> p;
        auto o = c[0], s = p[0];
        auto ce = trans.find(o) != trans.end();

        if(!ce) 
            trans[o] = new unordered_set<char>();
      
        trans[o]->emplace(s);
    }

    while(m--) {
        cin >> c >> p;
        if(c.length() == p.length()) {
            for(auto i = 0; i < c.length(); i++) {
                visited.clear();
                if(!find(c[i],p[i])) {
                    goto end;
                }
            }
            cout << "yes" << endl;
        } else {
            end:
            cout << "no" << endl;
        }
    }
}
