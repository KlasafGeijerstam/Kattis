mod inp;
use inp::Inp;

use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let (a, b, c, n) = inp.next_quad::<u32>();
    if a > 0 && b > 0 && c > 0 && a + b + c >= n && n > 3 {
        println!("YES");
    } else {
        println!("NO");
    }
    Ok(())
}
