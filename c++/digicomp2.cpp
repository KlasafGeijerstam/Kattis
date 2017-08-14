#include <iostream>
#include <vector>
#include <string>

struct Switch {
    long long ballz;
    bool v,visited;
    int left, right;
    std::vector<int> con;
    Switch() {

    }
    Switch(bool val, int l, int r)
    {
        v = val;
        left = l;
        right = r;
        visited = false;
    }
    void mod();
};

Switch* tree;
void Switch::mod()
{
    if (visited)
        return;
    visited = true;
    for (const auto& m : con)
        tree[m].mod();
    if (v)
    {
        tree[left].ballz += (ballz+1)/2;
        tree[right].ballz += ballz / 2;
    }
    else
    {
        tree[right].ballz += (ballz + 1) / 2;
        tree[left].ballz += ballz / 2;
    }
}

inline bool even(unsigned long long p)
{
    return !(p & 1);
}



int main()
{
    std::ios_base::sync_with_stdio(false);
    unsigned long long n;
    int l, r, m;
    char s;

    std::cin >> n >> m;
    tree = new Switch[500001];
    for (int i = 1; i <= m; i++)
    {
        std::cin >> s >> l >> r;
        tree[i].v = s == 'L';
        tree[i].left = l;
        tree[i].right = r;
        tree[i].ballz = 0;
        tree[i].visited = false;
        tree[r].con.push_back(i);
        tree[l].con.push_back(i);
    }
    tree[1].ballz = n;
    tree[0].ballz = 0;
    tree[0].left = 0;
    tree[0].right = 0;
    tree[0].visited = false;
    tree[0].mod();
    for (int i = 1; i <= m; i++)
        std::cout << ((even(tree[i].ballz) ? tree[i].v : !tree[i].v) ? 'L' : 'R');
    std::cout << std::endl;
}
