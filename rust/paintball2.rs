mod fast_input;
use fast_input::FastInput;
use std::collections::{HashSet, HashMap};

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
    let circles: Vec<Circle> = (0..n)
        .map(|_| inp.next_triple().into())
        .collect();
    
    let mut uf = UnionFind::new(n);

    for (i, c) in circles.iter().enumerate() {
        for (j, u) in circles.iter().enumerate().skip(i) {
            if c.collides(u) {
                uf.union(i, j); 
            }
        }
    }
    let mut graphs = HashMap::new();
    for i in 0..n {
        let root = uf.find(i); 
        graphs.entry(root).or_insert(HashSet::new()).insert(i);
    }

    let mut top_left = 1000.0;
    let mut top_right = 1000.0;

    for graph in graphs.values() {
        let mut hit_left = false;
        let mut hit_top = false;
        let mut hit_right = false;
        let mut hit_bottom = false;

        let mut left_max_y = 0.0;
        let mut left_min_y = 1000.0;

        let mut top_max_x = 0.0;
        let mut top_min_x = 1000.0;

        let mut right_max_y = 0.0;
        let mut right_min_y = 1000.0;

        for circle in graph.iter().map(|i| &circles[*i]) {
            //Does circle hit left wall?
            if circle.origin.x - circle.radius < 0.0 {
                hit_left = true;

                let a = circle.origin.x;
                let y = circle.origin.y;
                let r = circle.radius;
                let b = (a.powf(2.0) - r.powf(2.0)).abs().sqrt();
                //Intersects at (a, y + b), (a, y - b)
                if y + b < 1000.0 {
                    left_max_y = f64::max(left_max_y, y + b);
                }

                if y - b > 0.0 {
                    left_min_y = f64::min(left_min_y, y - b);
                }
            } else if circle.origin.x + circle.radius > 1000.0 {
                hit_right = true;

                let a = 1000.0 - circle.origin.x;
                let y = circle.origin.y;
                let r = circle.radius;
                let b = (a.powf(2.0) - r.powf(2.0)).abs().sqrt();
                //Intersects at (x, y + b), (x, y - b)
                if y + b < 1000.0 {
                    right_max_y = f64::max(right_max_y, y + b);
                }

                if y - b > 0.0 {
                    right_min_y = f64::min(right_min_y, y - b);
                }
            }

            if circle.origin.y + circle.radius > 1000.0 {
                hit_top = true;

                let a = 1000.0 - circle.origin.y;
                let x = circle.origin.x;
                let r = circle.radius;
                let b = (a.powf(2.0) - r.powf(2.0)).abs().sqrt();
                //Intersects at (x + b, a), (x - b, a)
                if x + b < 1000.0 {
                    top_max_x = f64::max(top_max_x, x + b);
                }

                if x - b > 0.0 {
                    top_min_x = f64::min(top_min_x, x - b);
                }
            } 

            if circle.origin.y - circle.radius < 0.0 {
                hit_bottom = true; 
            }

        }
        
        // Topmost entry is now low
        if hit_left && hit_top {
            top_left = f64::min(left_min_y, top_left);
        }

        if hit_right && hit_top {
            top_right = f64::min(right_min_y, top_right);
        }

        if hit_top && hit_bottom { 
            println!("IMPOSSIBLE");
            return
        }
    }

    println!("0.00 {:.2} 1000.00 {:.2}", top_left, top_right);
}
