use crate::point::{PointTrait, Point};
#[allow(unused)]
pub fn convex_hull<T: PointTrait>(mut points: Vec<Point<T>>) -> Vec<Point<T>> {
    points.sort_unstable();
    if points.len() <= 1 {
        return points;
    }
    let mut lower = Vec::<Point<T>>::new();
    for p in &points {
        while lower.len() >= 2 {
            let p1 = lower[lower.len() - 1];
            let p2 = lower[lower.len() - 2];
            if p2.cross2(&p1, p) > T::zero() {
                break;
            }

            lower.pop();
        }
        lower.push(*p);
    }

    let mut upper = Vec::<Point<T>>::new();
    for p in points.iter().rev() {
        while upper.len() >= 2 {
            let p1 = upper[upper.len() - 1];
            let p2 = upper[upper.len() - 2];
            if p2.cross2(&p1, p) > T::zero() {
                break;
            }

            upper.pop();
            
        }
        upper.push(*p);
    }
    upper.pop();
    lower.pop();
    lower.append(&mut upper);
    lower
}

#[test]
fn test_convex() {
    let points: Vec<_> = (0..100)
        .map(|i| Point::new((i / 10) as f64, (i % 10) as f64))
        .collect();
    
    println!("{:?}", convex_hull(points));
}
