#include <iostream>
#include <vector>
#include <map>

struct team {
    int a, b;
};

bool operator <(const team& m, const team& n) {
    if (m.a != n.a)
        return m.a < n.a;
    return m.b > n.b;
}

bool operator <=(const team& m, const team& n) {
    if (m.a < n.a)
        return true;
    else if (m.a == n.a)
        return m.b >= n.b;
    return false;
}

int main() {
    std::ios_base::sync_with_stdio(false);
    int teams, events, a, plep;
    std::cin >> teams >> events;
    team us = { 0,0 };
    std::map<int, team> upper, lower;
    for (int i = 0; i < teams; i++)
        lower[i + 1] = { 0,0 };

    while (events--) {
        std::cin >> a >> plep;
        if (a == 1) {

            us.a++;
            us.b += plep;
            std::vector<int> toRemove;

            for (auto i = upper.begin(); i != upper.end(); i++)
                if (i->second <= us)
                    toRemove.push_back(i->first);
            for (const auto& i : toRemove) {
                lower[i] = upper[i]; 
                upper.erase(i);
            }
        }
        else if (upper.find(a) != upper.end()) {
            upper[a].a++;
            upper[a].b += plep;
        }
        else {
            lower[a].a++;
            lower[a].b += plep;

            if (us < lower[a]) {
                upper[a] = lower[a];
                lower.erase(a);
            }
        }
        std::cout << upper.size() + 1 << std::endl;
    }
}
