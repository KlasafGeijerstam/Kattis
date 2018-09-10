#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>
#include "IntervalTree.h"

using namespace std;

int n,q, xa,ya, X1,Y1,x2,y2; 

inline int d(int x1, int y1) {
    return abs(x1-xa)+abs(y1-ya);
}

int main() {
    ios_base::sync_with_stdio(false);
    cin >> n >> q >> xa >> ya;
    vector<Span> brds(n);
    while(n--) {
        cin >> X1 >> Y1 >> x2 >> y2;
        Span s;
        if((xa >= X1 && xa <= x2 && ya >= Y1 && ya <= y2) 
        || (xa >= x2 && xa <= X1 && ya >= y2 && ya <= Y1)
        || (xa >= X1 && xa <= x2 && ya >= y2 && ya <= Y1)
        || (xa >= x2 && xa <= X1 && ya >= Y1 && ya <= y2)) {

            s.min = 0;
        } else if((xa > X1 && xa < x2) || (xa > x2 && xa < X1)) {
           s.min = min(abs(ya-Y1), abs(ya-y2));
        } else if((ya > Y1 && ya < y2) || (ya > y2 && ya < Y1)) {
           s.min = min(abs(xa-X1), abs(xa-x2));
        } else {
          s.min = min(d(X1,Y1), min(d(x2,y2), min(d(x2, Y1), d(X1,y2))));
        }
        s.max = max(d(X1,Y1), max(d(x2,y2), max(d(x2, Y1), d(X1,y2))));
        brds[n] = s;
    }
    auto it = IntervalTree(brds);
    while(q--) {
        cin >> n;
        cout << it.intersectionCount(n) << endl;
    }
}
