#include <iostream>
#include <cmath>
#include <vector>
#include <map>
#include <algorithm>

using namespace std;

typedef struct Node{
    vector<Node*> cns;
    int num;
    bool visited;

    Node(int n) {
        num = n;
        visited = false;
    }
    Node() {}
}Node;

void hasInternetz(Node* n) 
{
    n->visited = true;
    for (const auto& c : n->cns)
        if (!c->visited)
            hasInternetz(c);
}

int main()
{
    std::ios::sync_with_stdio(false);
    int hc, nc,h1,h2,wr = 0;
    cin >> hc >> nc;
    map<int, Node> hses;
    for (int i = 0; i < nc; i++)
    {
        cin >> h1 >> h2;
        if (hses.find(h1) == hses.end())
            hses[h1] = Node(h1);
        
        if (hses.find(h2) == hses.end())
            hses[h2] = Node(h2);
        hses[h1].cns.push_back(&hses[h2]);
        hses[h2].cns.push_back(&hses[h1]);
    }
    hasInternetz(&hses[1]);
    for(int i = 1; i <= hc; i++)
        if (!hses[i].visited) {
            cout << i << endl;
            wr = 1;
        }
    if (!wr)
        cout << "Connected" << endl;
}
