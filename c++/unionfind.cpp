#include <iostream>
#include <map>
#include <string>
#include <vector>

class UnionFind
{
public:
	UnionFind(int items) :
		parents(items),
		ranks(items, 1)
	{
		for (int i{ 0 }; i < items; ++i)
		{
			parents[i] = i;
		}
	}

	void link(int left, int right)
	{
		int ri{ find(left) };
		int rj{ find(right) };

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

	int find(int item)
	{
		int root{ item };

		while (root != parents[root])
		{
			root = parents[root];
		}

		while (item != root)
		{
			int parent{ parents[item] };

			parents[item] = root;
			item = parent;
		}

		return root;
	}

	int size(int item)
	{
		return ranks[find(item)];
	}

private:
    std::vector<int> parents;
    std::vector<int> ranks;
};

int main() {
    std::ios_base::sync_with_stdio(false);
    int n, q, a, b;
    char c;
    std::cin >> n >> q;
    UnionFind uf = *(new UnionFind(n));
    while(q--) {
        std::cin >> c >> a >> b;
        if(c == '?')
            std::cout << (uf.find(b) == uf.find(a) ? "yes" : "no") << std::endl;
        else {
            uf.link(a,b);        
        }
    }
}
