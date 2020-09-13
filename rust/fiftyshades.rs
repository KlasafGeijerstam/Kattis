mod inp;
use inp::Inp;

use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new(); 
    let n = inp.next();
    let mut c = 0;
    for _ in 0..n {
        let l: String = inp.next_line()
            .chars()
            .filter(|&x| x != ' ')
            .map(|x| x.to_lowercase().next().unwrap())
            .collect();
        if l.contains("pink") || l.contains("rose") {
            c += 1;
        }
    }
    if c == 0 {
        println!("I must watch Star Wars with my daughter");
    } else {
        println!("{}", c);
    }
    Ok(())
}
