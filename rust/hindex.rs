mod inp;
use inp::Inp;
use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let n = inp.next(); 
    let mut p: Vec<usize> = (0..n).map(|_| inp.next()).collect();
    p.sort_unstable();
    let mut h = 1;
    let mut i = 0;
    loop {
        while i < p.len() && p[i] < h {
            i += 1;
        }
        if i == p.len() || p.len() - i < h {
            break
        }
        h += 1;
    }
    println!("{}", h - 1);

    Ok(())
}
