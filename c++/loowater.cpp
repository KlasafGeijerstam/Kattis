#include <iostream>
#include <algorithm>
#include <vector>
using namespace std;
int main() {
    ios_base::sync_with_stdio(false);
    int n, m, a, cost, ki;
    cin >> n >> m;
    while(n + m > 0) {
        vector<int> h(n);
        vector<int> k(m);
        cost = ki = 0;
        while(n--) {
            cin >> a;
            h[n] = a;
        }
        while(m--) {
            cin >> a;
            k[m] = a;
        }
        sort(h.begin(), h.end());
        sort(k.begin(), k.end());
        for(const auto& i : h) {
            if(ki == k.size()) {
                cout << "Loowater is doomed!" << endl;
                goto end;
            }
            for(; ki < k.size(); ki++) {
                if(k[ki] >= i) {
                    cost += k[ki++];
                    break;
                }
            }
        } 
        cout << cost << endl;
end:
        cin >> n >> m;
    } 
}
