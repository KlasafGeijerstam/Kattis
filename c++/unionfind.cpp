#include <iostream>
#include <vector>

class UnionFind
{
public:
    UnionFind(long items) :
        parents(items),
        ranks(items, 1)
    {
        for (long i{ 0 }; i < items; ++i)
        {
            parents[i] = i;
        }
    }

    void link(long left, long right)
    {
        long ri{ find(left) };
        long rj{ find(right) };

        if (ri == rj)
        {
            return;
        }

        if (ranks[ri] < ranks[rj])
        {
            parents[ri] = rj;
            ranks[rj] += ranks[ri];
        }
        else
        {
            parents[rj] = ri;
            ranks[ri] += ranks[rj];
        }
    }

    long find(long item)
    {
        long root{ item };

        while (root != parents[root])
        {
            root = parents[root];
        }

        while (item != root)
        {
            long parent{ parents[item] };

            parents[item] = root;
            item = parent;
        }

        return root;
    }

    long size(long item)
    {
        return ranks[find(item)];
    }

private:
    std::vector<long> parents;
    std::vector<long> ranks;
};

int main() {
    std::ios_base::sync_with_stdio(false);
    long n, q, a, b;
    char c;
    std::cin >> n >> q;
    UnionFind uf = *(new UnionFind(n));
    while(q--) {
        std::cin >> c >> a >> b;
        if(c == '?')
            std::cout << (uf.find(b) == uf.find(a) ? "yes" : "no") << std::endl;
        else
            uf.link(a,b);        
    }
}
