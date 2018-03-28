#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <sstream>
#include <unordered_set>
#include <map>

struct Trans
{
    int shortest;
    std::unordered_set<std::string> trans;
};

int main()
{
    std::ios_base::sync_with_stdio(false);
    std::string txt, w1,w2;

    int n, m;

    std::cin >> n >> m;
    std::getline(std::cin, txt);

    std::getline(std::cin, txt);
    std::stringstream s(txt);
    
    std::map<std::string, Trans*> transMap;
    
    
    for (int i = 0; i < m; i++)
    {
        std::cin >> w1 >> w2;
        auto b1 = transMap.find(w1) != transMap.end(), b2 = transMap.find(w2) != transMap.end();
        if (!b1 && !b2) {
            //No one exists
            Trans* t = new Trans();
            t->shortest = std::min(w1.length(), w2.length());
            t->trans.emplace(w1);
            t->trans.emplace(w2);
            transMap[w1] = t;
            transMap[w2] = t;
        }
        else if (!b1 && b2) {
            //w2 exists
            auto p = transMap[w2];
            p->shortest = std::min(w1.length(), (size_t)p->shortest);
            p->trans.emplace(w1);
            transMap[w1] = p;
        }
        else if (b1 && !b2) {
            //w1 exists
            auto p = transMap[w1];
            p->shortest = std::min(w2.length(), (size_t)p->shortest);
            p->trans.emplace(w2);
            transMap[w2] = p;
        }
        else {
            //Both exist, move the shorter one over to larger one
            auto p1 = transMap[w1], p2 = transMap[w2];
            if (p1->trans.size() > p2->trans.size()) {
                for(auto c : p2->trans) {
                    p1->trans.emplace(c);
                    if (p1->shortest > c.length())
                        p1->shortest = c.length();
                    transMap[c] = p1;
                }
                //delete(p2);
            }
            else {
                for (auto c : p1->trans) {
                    p2->trans.emplace(c);
                    if (p2->shortest > c.length())
                        p2->shortest = c.length();
                    transMap[c] = p2;
                }
                //delete(p1);
            }
        }
    }
    int len = 0;
    while (s >> txt) {
        if (transMap.find(txt) != transMap.end())
            len += transMap[txt]->shortest;
        else
            len += txt.length();
    }
    std::cout << len << std::endl;
}
