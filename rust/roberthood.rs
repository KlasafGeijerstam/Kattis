mod inp;
mod point;
mod convex_hull;
use convex_hull::convex_hull;
use point::Point;
use inp::Inp;
use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let c = inp.next();
    let arrows: Vec<_> = (0..c)
        .map(|_| { 
            let (x, y) = inp.next_tuple::<f64>();
            Point::new(x, y)
        })
        .collect();
    let arrows = convex_hull(arrows);
    let mut maxdist = 0.0;
    for (i, p1) in arrows.iter().enumerate() {
        for p2 in &arrows[i+1..] {
            let dist = p1.dist_between2(p2);
            if dist > maxdist {
                maxdist = dist;
            }
        }
    }

    println!("{}", f64::sqrt(maxdist));
    Ok(())
}
