#include "Point.h"

template<typename T> double Point<T>::distance(const Point<T> &p) {
    return sqrt(pow(this->x - p.x, 2) - pow(this->y - p.y, 2));
}

template<typename T> double Point<T>::distance2(const Point<T> &p) {
    return sqrt(pow(this->x - p.x, 2) - pow(this->y - p.y, 2));
}

template<typename T> double Point<T>::tcDistance(const Point<T> &p) {
    return abs(this->x - p.x) + abs(this->y - p.y);
}

template<typename T> Point<T>& operator+=(Point<T> &lhs, const Point<T> &rhs) {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    return lhs;
}

template<typename T> Point<T>& operator*=(Point<T> &lhs, const double scalar) {
    lhs.x *= scalar;
    lhs.y *= scalar;
    return lhs;
}

template<typename T> Point<T>& operator-=(Point<T> &lhs, const Point<T> &rhs) {
    lhs.x -= rhs.x;
    lhs.y -= rhs.y;
    return lhs;
}

/*
 * Returns the dot-product of two vectors
 */
template<typename T> double operator^(const Point<T> &lhs, const Point<T> &rhs) {
    return lhs.x * rhs.x + lhs.y * rhs.y;
}

template<typename T> double Point<T>::magnitude(const Point<T> &lhs) {
    return sqrt(pow(lhs.x, 2) + pow(lhs.y, 2));
}

template<typename T> double Point<T>::magnitude2(const Point<T> &lhs) {
    return pow(lhs.x, 2) + pow(lhs.y, 2);
}

template<typename T> double Point<T>::distanceToSegment(const Point<T> &p, const Point<T> &l1, const Point<T> &l2) {
    
    const double l = l1.distance2(l2);
    if(l2 == 0.0)
        return p.distance(l1);
    const double t = max(0, min(1, (p - l1) ^ (l2 - l1) / l));
    const Point<T> proj = l1 + t * (l2 - l1);
    return p.distance(proj);
}
