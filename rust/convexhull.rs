mod inp;
mod point;
mod convex_hull;
use convex_hull::convex_hull;
use point::Point;
use inp::Inp;
use std::io::{BufWriter, Result};
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let stdin = std::io::stdout();
    let mut writer = BufWriter::new(stdin.lock());

    loop {
        let n: u32 = inp.next();
        if n == 0 {
            break
        }
        let mut arrows: HashSet<_> = (0..n)
            .map(|_| { 
                let (x, y) = inp.next_tuple::<i32>();
                Point::new(x, y)
            })
            .collect();
        let arrows = convex_hull(arrows.drain().collect());
        writeln!(writer, "{}", arrows.len())?;
        for p in &arrows {
            writeln!(writer, "{} {}", p.x, p.y)?;
        }
    }

    Ok(())
}

