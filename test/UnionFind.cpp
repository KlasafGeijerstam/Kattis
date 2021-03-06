#include <iostream>
#include <map>
#include <string>
#include <vector>

using namespace std;

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
	vector<long> parents;
	vector<long> ranks;
};
