mod inp;
use inp::Inp;
use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let (n, p) = inp.next_tuple();
    let students: Vec<u32> = inp.next_as_iter().collect();
    let mut sum = students.iter().sum::<u32>();
    let studs = students.len() as u32;
    let mut add = 0;
    if p < 100 {
        while sum / (studs + add) < p {
            sum += 100;
            add += 1;
        }
        println!("{}", add);
    } else {
        println!("impossible");
    }

    Ok(())
}
