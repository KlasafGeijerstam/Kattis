#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

    struct runner {
    double t1, t2;
    std::string name;
};


bool operator< (const runner& lhs, const runner& rhs) { return lhs.t2 < rhs.t2; }


int main() {
    int len;
    std::cin >> len;
    std::string name;
    double p, k;

    std::vector<runner> r;

    while (len--) {
        std::cin >> name >> p >> k;
        r.push_back({ p, k, name });
    }
    std::sort(r.begin(), r.end());

    double mk = 10000000000;
    std::vector<std::string> fin;
    for (const auto& c : r) {
        std::vector<std::string> tmp;
        double mt = c.t1;
        tmp.push_back(c.name);
        for (const auto& c1 : r) {
            if (c.name == c1.name)
                continue;
            mt += c1.t2;
            tmp.push_back(c1.name);
            if (tmp.size() == 4)
                break;
        }
        if (mt < mk) {
            mk = mt;
            fin.erase(fin.begin(), fin.end());
            for (const auto& p : tmp)
                fin.push_back(p);
        }
    }

    std::cout << mk << std::endl;
    for (const auto& k : fin)
        std::cout << k << std::endl;
    std::cin >> len;
}
