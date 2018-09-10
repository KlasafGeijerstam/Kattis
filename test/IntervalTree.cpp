#include "IntervalTree.h"

bool Span::within(int i) {
    return i >= this->min && i <= this->max;
}

IntervalTree::IntervalTree(std::vector<Span> &s) : center(Node(s)) {

}
int IntervalTree::intersectionCount(int x) {
    return this->center.within(x);
}


int Node::center() {
    return this->cent;
}

int Node::within(int x) {
    auto c = 0;
    if(x < this->cent) {
       c += this->getLessThan(x); 
        if(this->left)
            c += this->left->within(x);

    } else if(x > this->cent) {
        c += this->getMoreThan(x);
        if(this->right)
            c += this->right->within(x);
    } else {
        c += this->bcenter.size();
    }
    return c;
}

int Node::getLessThan(int x) {
    int mid, min = 0, max = this->bcenter.size();
    if(this->bcenter.size() == 0 || this->bcenter[0].min > x)
        return 0;
    else if(this->bcenter[max-1].min <= x)
        return max;

    while(min != max) {
        mid =  min + (max - min) / 2;
        if(this->bcenter[mid].min <= x) {
            min = mid +1;
        } else {
            max = mid;
        }
    }
    return min;
}

int Node::getMoreThan(int x) {
    int mid, min = 0, max = this->ecenter.size();
    if(this->ecenter.size() == 0 || this->ecenter[0].max < x)
        return 0;
    else if(this->ecenter[max-1].max >= x)
        return max;
    while(min != max) {
        mid =  min + (max - min) / 2;
        if(this->ecenter[mid].max >= x) {
            min = mid +1;
        } else {
            max = mid;
        }
    }
    return min;
}

Node::Node(std::vector<Span> &p) {
    auto max = -1000000000, min = 1000000000;
    auto s = p;
    for(const auto& i : s) {
        max = std::max(i.min, max);
        min = std::min(i.min, min);
    }
    this->cent = (max + min) / 2;
    std::vector<Span> lhs;
    std::vector<Span> rhs; 
    for(const auto& i : s) {
        if(i.max < this->cent)
            lhs.push_back(i);
        else if(i.min > this->cent)
            rhs.push_back(i);
        else {
            this->bcenter.push_back(i);
            this->ecenter.push_back(i);
        }
    }
    std::sort(this->bcenter.begin(), this->bcenter.end(), 
                                    [](const Span &l, const Span &r) -> bool {
                                        return l.min < r.min;
                                    });
    std::sort(this->ecenter.begin(), this->ecenter.end(),
                                    [](const Span &l, const Span &r) -> bool {
                                        return l.max > r.max;
                                    });
    if(lhs.size() > 0)
        this->left = new Node(lhs);
    else
        this->left = NULL;
    if(rhs.size() > 0)
        this->right = new Node(rhs);
    else
        this->right = NULL;

}
