mod fast_input;
use fast_input::{FastInput};

fn main() {
    let inp = FastInput::new();
    let sum: u32 = inp.next_line()
        .split(';')
        .map(|p| {
            let p: Vec<_> = p.split('-').collect();
            if p.len() == 1 {
                1
            } else {
                1 + p[1].parse::<u32>().unwrap() - p[0].parse::<u32>().unwrap()
            }
        })
        .sum();
    println!("{}", sum);
}
