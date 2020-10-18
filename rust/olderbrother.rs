mod fast_input;
use fast_input::FastInput;

use std::io::Result;

fn main() -> Result<()> {
    let mut inp = FastInput::new();
    let mut q: u32 = inp.next();
    if q == 1 {
        println!("no");
    } else {
        let mut p = 2;
        while p * p <= q {
            if q % p == 0 {
                while q % p == 0 {
                    q /= p;
                }
                if q == 1{
                    println!("yes");
                } else {
                    println!("no");
                }
                return Ok(())
            }
            p += 1;
        }
        println!("yes");
    } 
    
    Ok(())
}
