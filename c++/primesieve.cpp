#include <iostream>
#include <string>
#include <bitset>
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdint>
using namespace std;

uint_fast64_t bits[781251];
#define SHIFT(i) (1l << (((i - 1) / 2) % 64))

inline uint_fast64_t shift(int i) {
    return 1lu << (((i - 1) / 2) % 64);
}

inline void set(int i) {
    if(!(i & 1)) {
        return;
    }

    bits[i / 128] |= shift(i);
}

inline bool get(int i) {
    if(i == 2) {
        return true;
    }

    if(!(i & 1)) {
        return false;
    }
    uint_fast64_t m = shift(i);
    return (bits[i / 128] & m) != m;
}

int main()
{
    ios_base::sync_with_stdio(false);
    int n, q, primes;
    cin >> n >> q;
    bits[0] = 1;
    primes = (int)ceil(n / 2.0);

    int limit = sqrt(n);

    for(int i = 3; i <= limit; i += 2) {
        if(get(i)) {
            for(int j = i*i; j <= n; j += i) {
                if(get(j)){
                    primes--;
                    set(j);
                }
            }
        }
    }

    cout << primes << endl;
    for(int i = 0; i < q; i++)
    {
        cin >> n;
        printf("%d\n",get(n));
    }
}
