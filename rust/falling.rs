mod inp;
use inp::Inp;

use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let n: i32 = inp.next();
    for n2 in 1..125000 {
        let mut n1 = 0;
        let mut high = n2;

        while high - n1 > 1 {
            let mid = (high + n1) / 2;
            if n2 * n2 - mid * mid < n {
                high = mid - 1;
            } else {
                n1 = mid;
            }
        }

        if n2 * n2 - n1 * n1 == n {
            println!("{} {}", n1, n2);
            return Ok(())
        }
    }

    println!("impossible");
        
    Ok(())
}
