#include <iostream>
#include <set>

using namespace std;

set<int> baseSets[100002];
long sum[100002];
int parents[100002];

void merge(int a, int b)
{
    if (parents[a] != parents[b])
    {
        int setA = parents[a], setB = parents[b];
        if (baseSets[setA].size() >= baseSets[setB].size())
        {
            sum[setA] += sum[setB];
            baseSets[setA].insert(baseSets[setB].begin(), baseSets[setB].end());
        
            for (auto i : baseSets[setB])
               parents[i] = setA; 
        }
        
        else
        {
            sum[setB] += sum[setA];
            baseSets[setB].insert(baseSets[setA].begin(), baseSets[setA].end());
            for (auto i : baseSets[setA])
               parents[i] = setB; 
        }
        
    }
}

void move(int vfrom, int vto)
{
    if (parents[vfrom] != parents[vto])
    {
        int from = parents[vfrom], to = parents[vto];
        
        sum[from] -= vfrom;
        sum[to] += vfrom;
        
        parents[vfrom] = to;
        
        baseSets[from].erase(vfrom);
        baseSets[to].insert(vfrom);
    }
}

int main()
{
    int n, m;
    
    while (cin >> n >> m)
    {
        for (int i = 1; i <= n; ++i)
        {
            parents[i] = i;
            baseSets[i].clear();
            baseSets[i].insert(i);
            sum[i] = i;
        }
        
        while (m--)
        {
            int type, p, q;
            cin >> type;
            if (type == 1)
            {
                cin >> p >> q;
                merge(p, q);
            }
            else if (type == 2)
            {
                cin >> p >> q;
                move(p, q);
            }
            else
            {
                cin >> p;
                int index = parents[p];
                cout << baseSets[index].size() << ' ' << sum[index] << endl;
            }
        }
    }
}
