mod fast_input;
use fast_input::{FastInput};

fn main() {
    let inp = FastInput::new();
    for c in 0..inp.next() {
        let mut red = Vec::new();
        let mut blue = Vec::new();
        inp.next_line();
        for s in inp.next_split() {
            if s.ends_with("R") {
                red.push(s[..s.len() - 1].parse::<u32>().unwrap());
            } else {
                blue.push(s[..s.len() - 1].parse::<u32>().unwrap());
            }
        }
        red.sort();
        red.reverse();
        blue.sort();
        blue.reverse();

        let min = usize::min(red.len(), blue.len());
        let sum = red.iter()
            .take(min)
            .chain(blue.iter().take(min))
            .sum::<u32>() - 2 * min as u32;
        println!("Case #{}: {}", c + 1, sum);
    }
}
