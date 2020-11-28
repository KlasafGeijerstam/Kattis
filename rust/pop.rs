mod fast_input;
mod point3d;
use point3d::*;
use fast_input::{FastInput};

struct Balloon {
    pos: Point3D,
    r: f64,
    popped: bool,
}

fn collide(b: &Balloon, origin: &Point3D, dir: &Point3D) -> bool {
    let before = (b.pos - *origin).normalize();

    if dir.dot(before) < 0.0 {
        return false
    }

    let p2 = *origin + *dir * 200.0;
    let vl = p2 - *origin;
    let w = b.pos - *origin;
    let ul = vl.normalize();
    let dist = ul.cross(w).norm();

    dist <= b.r
}

fn main() {
    let inp = FastInput::new();
    loop {
        let n: usize = inp.next();

        if n == 0 {
            break
        }
        
        let mut balloons: Vec<_> = (0..n).map(|_| {
            let (r, s, x, y): (_, f64, _, _) = inp.next_quad();            
            let z = s + r;
            Balloon {
                pos: (x, y, z).into(),
                r,
                popped: false,
            }
        }).collect();
        
        for _ in 0..inp.next() {
            let shot = inp.next_septuple();
            let origin = (shot.0, shot.1, shot.2).into();
            let dir = (shot.3, shot.4, shot.5).into();
            let mut points = 0;
            for b in &mut balloons {
                if !b.popped && collide(b, &origin, &dir) {
                    b.popped = true; 
                    points += 1;
                }
            }

            println!("{}", points);
        }
    }
}
