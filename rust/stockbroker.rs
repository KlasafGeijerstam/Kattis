mod inp;
use inp::Inp;
use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let mut money = 100u64;

    let d = inp.next();
    let mut prev: u64 = inp.next();
    for _ in 1..d {
        let p = inp.next();
        if p > prev {
            let max = u64::min(money / prev, 100_000);
            money +=  max * p - prev * max;
        }
        prev = p;
    }
    println!("{}", money);
    Ok(())
}
