mod fast_input;
use fast_input::FastInput;
use std::collections::HashSet;

use std::io::Result;
struct Beacon {
    x: i32,
    y: i32,
    d: i32,
}

impl Beacon {
    fn on_side(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() == self.d
    }
}

// With help from BAPC 2016 post-comp docs.
fn main() -> Result<()> {
    let mut inp = FastInput::new();
    let n = inp.next();
    let beacons: Vec<_> = (0..n).map(|_| {
        let (x, y, d) = inp.next_triple();
        Beacon { x, y, d }
    }).collect();

    let Beacon {x, y, d} = beacons[0];
    let mut points = HashSet::<(i32, i32)>::new();

    points.extend(process(&beacons, x - d, x, -1, x - d - y));
    points.extend(process(&beacons, x - d, x, 1, x - d + y));
    points.extend(process(&beacons, x, x + d, 1, x + d + y));
    points.extend(process(&beacons, x, x + d, -1, x + d - y));
    
    if points.is_empty() {
        println!("impossible");
    } else if points.len() > 1 {
        println!("uncertain");
    } else {
        for (x, y) in points {
            println!("{} {}", x, y);
        }
    }
    
    Ok(())
}

fn process(beacons: &Vec<Beacon>, mut xs: i32, mut xe: i32, ysign: i32, rhs: i32) -> HashSet<(i32, i32)> {
    let mut output = HashSet::new();
    for Beacon {x, y, d} in beacons {
        if x + d + ysign * y == rhs {
            xs = i32::max(xs, *x);
            xe = i32::min(xe, x + d);
        } else if x - d + ysign * y == rhs {
            xs = i32::max(xs, x - d);
            xe = i32::min(xe, *x);
        } else {
            let rhs2 = x - d - ysign * y;
            let p = ((rhs + rhs2) / 2, ysign * (rhs - rhs2) / 2);
            if (rhs - rhs2) % 2 == 0 && possible(p.0, p.1, beacons) {
                output.insert(p);
            }
            let rhs2 = x + d - ysign * y;
            let p = ((rhs + rhs2) / 2, ysign * (rhs - rhs2) / 2);
            if (rhs - rhs2) % 2 == 0 && possible(p.0, p.1, beacons) {
                output.insert(p);
            }
            return output
        }

        if xs > xe {
            return output
        }
    }

    output.insert((xs, (rhs - xs) * ysign));
    output.insert((xe, (rhs - xe) * ysign));
    output
}

fn possible(x: i32, y: i32, beacons: &Vec<Beacon>) -> bool {
    beacons.iter().all(|b| b.on_side(x, y))
}


