use crate::point::{Point};
use std::collections::HashSet;
type V = i128;
pub type Segment = (Point<V>, Point<V>);

pub fn segment_intersection((a, b): &Segment, (c, d): &Segment) -> Vec<Point<V>> {
    let oa = c.cross2(&d, &a);
    let ob = c.cross2(&d, &b);
    let oc = a.cross2(&b, &c);
    let od = a.cross2(&b, &d);
    if oa.signum() * ob.signum() < 0 && oc.signum() * od.signum() < 0 {
        return vec![(*a * ob - *b * oa) / (ob - oa)]
    }
    let mut ps = HashSet::new();
    if on_segment(&(*c, *d), a) {
        ps.insert(a);
    }
    if on_segment(&(*c, *d), b) {
        ps.insert(b);
    }
    if on_segment(&(*a, *b), c) {
        ps.insert(c);
    }
    if on_segment(&(*a, *b), d) {
        ps.insert(d);
    }
    ps.drain().cloned().collect()
}

pub fn on_segment((s, e): &Segment, p: &Point<V>) -> bool {
    p.cross2(&s, &e) == 0 && (*s - *p).dot(&(*e - *p)) <= 0
}
