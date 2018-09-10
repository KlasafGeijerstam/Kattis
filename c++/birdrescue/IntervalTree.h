#include <algorithm>
#include <cmath>
#include <vector>

struct Span {
    int min, max;
    Span() : min(0), max(0) {}
    Span(int m, int n) : min(m), max(n) {}
    bool within(int i);
};

class Node {
    public:
        Node(std::vector<Span> &s);
        int center();
        int within(int x);
    private:
        int cent;
        Node* left, *right;
        std::vector<Span> bcenter, ecenter;
        int getLessThan(int x);
        int getMoreThan(int x);
};

class IntervalTree {
    public:
        IntervalTree(std::vector<Span> &s);
        int intersectionCount(int i);
    private:
        Node center;
};
