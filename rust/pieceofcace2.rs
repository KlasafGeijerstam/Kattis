use std::io;
use std::cmp;

#[derive(Eq)]
struct Rect { 
    w: u32,
    h: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.w * self.h * 4
    }

    fn n(w: u32, h: u32) -> Rect {
        Rect {w, h}
    }
}

impl cmp::Ord for Rect {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.area().cmp(&other.area())
    }
}

impl cmp::PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.area().cmp(&other.area()))
    }
}

impl cmp::PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.h == other.h
    }
}

fn main() -> io::Result<()> {
    let l = read_ints()?;
    let d = l[0];
    let mut rects = vec![Rect::n(d - l[1], d - l[2]),
                     Rect::n(l[1], d - l[2]),
                     Rect::n(d - l[1], l[2]),
                     Rect::n(l[1], l[2])];
    rects.sort();
    rects.reverse();

    println!("{}", rects[0].area());
    Ok(())
}

fn read_ints() -> Result<Vec<u32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
