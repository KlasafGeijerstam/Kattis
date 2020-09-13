mod inp;
use inp::Inp;

use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new(); 
    'case: loop {
        let t: u32 = inp.next();
        if t == 0 {
            break;
        }

        let mut nums:Vec<f64> = inp.next_as_iter().collect();
        nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut k = 0.0;
        for n in nums {
            if k >= n {
                println!("YES");
                continue 'case;
            }
            k += n;
        }
        println!("NO");
        
    }
    Ok(())
}
