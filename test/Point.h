#ifndef POINTH
#define POINTH
#include <cmath>
#include <algorithm>

template<typename T> class Point {
    public:
        T x, y;
    double distance(const Point<T> &p);
    double distance2(const Point<T> &p);
    double distanceToSegment(const Point<T> &l1, const Point<T> &l2);
    double tcDistance(const Point<T> &p);
    Point<T>& operator+=(const Point<T> &rhs);
    Point<T>& operator*=(const double scalar);
    Point<T>& operator-=(const Point<T> &rhs);
    double operator^(const Point<T> &rhs);
    double magnitude(const Point<T> &lhs); 
    double magnitude2(const Point<T> &lhs); 
    Point<T>& operator=(const Point<T> &rhs) {
        this->x = rhs.x;
        this->y = rhs.y;
        return this;
    }
};

#endif
