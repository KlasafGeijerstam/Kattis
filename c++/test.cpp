#include <iostream>
#include <vector>

struct fenwick {
    int n;
    std::vector<long long> data;
    fenwick(int _n) : n(_n), data(std::vector<long long>(_n)) { }
    void inc(int x, long long val) {
        while(x < n) {
            data[x] += val;
            x |= x + 1;
        }
    }
    long long sum(int x) {
        long long res = 0;
        while(x >= 0) {
            res += data[x];
            x = (x & (x + 1)) - 1;
        }
        return res;
    }
};

int main() {
    std::ios_base::sync_with_stdio(false);
    std::cin.tie(NULL);

    int n, q, a, b;
    char op;
    std::cin >> n >> q;
    fenwick f(n+1);
    for(int i=0; i<q; i++) {
        std::cin >> op;
        if(op == '+') {
            std::cin >> a >> b;
            f.inc(a, b);
        } else {
            std::cin >> a;
            std::cout << f.sum(--a) << '\n';
        }
    }
    return 0;
}