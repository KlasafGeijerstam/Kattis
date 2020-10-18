mod fast_input;
use fast_input::FastInput;
use std::collections::{HashSet, HashMap};

use std::io::Result;
fn main() -> Result<()> {
    let mut inp = FastInput::new();
    loop {
        let n:usize = inp.next();
        if n == 0 {
            break
        }
        let mut maps = HashMap::new();
        let mut files = HashSet::new();
        for _ in 0..n {
            let l = inp.next_line().to_owned();
            let h = hash_str(&l);
            *maps.entry(h).or_insert(HashMap::new()).entry(l.clone()).or_insert(0u32) += 1;
            files.insert(l); 
        }
        let collisions: u32 = maps.values().map(|v| { 
            v.values().enumerate().map(|(i, v1)| 
                v.values().skip(i + 1).fold(0, |a, v2| a + v1 * v2)
            ).sum::<u32>()
        }).sum();
        println!("{} {}", files.len(), collisions);
    }
    Ok(())
}

fn hash_str(s: &str) -> u8 {
    s.chars().fold(0, |a, c| a ^ c as u8)
}


