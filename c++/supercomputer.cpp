#include <iostream>
#include <stdlib.h>

using namespace std;

struct fenwick {
    int* dat;
    long length;

    fenwick(int size)
    {
        dat = (int*)calloc(size + 1, sizeof(int));
        length = size + 1;
    }
    
    void increment(int index, int value)
    {
        while (index < length) {
            dat[index] += value;
            index += index & (-index);
        }
    }

    int sum(int index)
    {
        int sum = 0;
        while (index > 0) {
            sum += dat[index];
            index -= index & (-index);
        }
        return sum;
    }

    int range(int i1, int i2)
    {
        return sum(i2) - sum(i1 - 1);
    }
};

int main()
{
    long n, k;
    cin >> n >> k;
    auto f = fenwick(n);
    bool* val = (bool*)calloc(n + 1,sizeof(bool));
    char v;
    long l, r;
    for (int i = 0; i < k; i++)
    {
        cin >> v;
        if (v == 'F') 
        {
            cin >> l;
            if (val[l]) {
                f.increment(l, -1);
                val[l] = false;
            }
            else {
                f.increment(l, 1);
                val[l] = true;
            }
        }
        else 
        {
            cin >> l >> r;
            cout << f.range(l, r) << endl;
        }
    }
}
