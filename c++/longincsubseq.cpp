#include <iostream>
#include <algorithm>
#include <set>
#include <vector>
using namespace std;
typedef long long ul;
const ul INF = 20000000000;
vector<ul> LIS(vector<ul> num) {
    auto i = 1, lislen = 0, max = -1, mi = 0;
    auto len = num.size();
    vector<ul> L(len + 1);
    vector<ul> I(len + 1);
    
    I[0] = -INF;
    for(; i < len; i++)
        I[i] = INF;
    for(i = 0; i < len; i++) {
        auto l = 0, h = lislen, m = 0;
        while(l <= h) {
            m = (l + h) / 2;
            if(I[m] < num[i])
                l = m + 1;
            else
                h = m - 1;
        }

        I[l] = num[i];
        L[i] = l;
        if(L[i] >= max) {
            mi = i;
            max = L[i]; 
        }
        if(lislen < l)
            lislen = l;
    } 
    ul p = INF, ins = max - 1; 
    vector<ul> ans(max);
    for(;mi >= 0; mi--) {
        if(num[mi] < p && L[mi] == max) {
            ans[ins--] = mi;
            max--;
            p = num[mi];
        }
    }
    return ans;
}

int main() {
    ios_base::sync_with_stdio(false);
    int n;
    while(cin >> n) {
        vector<ul> nums(n);
        for(int i = 0; i < n; i++)
            cin >> nums[i];
        
        auto st = LIS(nums);
        
        cout << st.size() << endl;
        for(auto i : st)
            cout << i << " ";
        cout << endl;
    }
}
