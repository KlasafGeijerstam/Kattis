mod fast_input;
use fast_input::FastInput;
use std::collections::{HashMap, HashSet};

mod point;
use point::Point;

mod union_find;
use union_find::UnionFind;

struct Circle {
    origin: Point<f64>,
    radius: f64,
}

impl Circle {
    fn collides(&self, other: &Self) -> bool {
        self.origin.dist_between(&other.origin) < self.radius + other.radius
    }
}

impl From<(f64, f64, f64)> for Circle {
    fn from((x, y, r): (f64, f64, f64)) -> Self {
        Circle {
            origin: Point::new(x, y),
            radius: r,
        }
    }
}

fn main() {
    let inp = FastInput::new();
    let n = inp.next();
    let circles: Vec<Circle> = (0..n).map(|_| inp.next_triple().into()).collect();

    let mut uf = UnionFind::new(n);

    for (i, c) in circles.iter().enumerate() {
        for (j, u) in circles.iter().enumerate().skip(i) {
            if c.collides(u) {
                uf.union(i, j);
            }
        }
    }
    let graphs = (0..n).fold(HashMap::new(), |mut graphs, i| {
        let root = uf.find(i);
        graphs.entry(root).or_insert_with(HashSet::new).insert(i);
        graphs
    });

    let mut top_left = 1000.0;
    let mut top_right = 1000.0;

    for graph in graphs.values() {
        let (mut hit_left, mut hit_top, mut hit_right, mut hit_bottom) =
            (false, false, false, false);
        let mut left_min_y = 1000.0f64;

        let mut right_min_y = 1000.0f64;

        for circle in graph.iter().map(|i| &circles[*i]) {
            if circle.origin.x - circle.radius < 0.0 {
                hit_left = true;

                let y = circle.origin.y;
                let b = ((circle.origin.x).powf(2.0) - circle.radius.powf(2.0))
                    .abs()
                    .sqrt();

                if y - b > 0.0 {
                    left_min_y = left_min_y.min(y - b);
                }
            } else if circle.origin.x + circle.radius > 1000.0 {
                hit_right = true;

                let y = circle.origin.y;
                let b = ((1000.0 - circle.origin.x).powf(2.0) - circle.radius.powf(2.0))
                    .abs()
                    .sqrt();

                if y - b > 0.0 {
                    right_min_y = right_min_y.min(y - b);
                }
            }

            if circle.origin.y + circle.radius > 1000.0 {
                hit_top = true;
            }

            if circle.origin.y - circle.radius < 0.0 {
                hit_bottom = true;
            }
        }

        if hit_left && hit_top {
            top_left = left_min_y.min(top_left);
        }

        if hit_right && hit_top {
            top_right = right_min_y.min(top_right);
        }

        if hit_top && hit_bottom {
            println!("IMPOSSIBLE");
            return;
        }
    }

    println!("0.00 {:.2} 1000.00 {:.2}", top_left, top_right);
}
