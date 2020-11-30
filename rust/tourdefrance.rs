mod fast_input;
use fast_input::{FastInput};


fn main() {
    let inp = FastInput::new();
    loop {
        let n: Vec<usize> = inp.next_as_iter().collect();

        if n[0] == 0 {
            break
        }
        let front: Vec<u32> = inp.next_as_iter().collect();
        let rear: Vec<u32> = inp.next_as_iter().collect();
        let mut ratios = Vec::new();
        for f in &front {
            for r in &rear {
                ratios.push(*r as f64 / *f as f64);
            }
        }
        ratios.sort_by(|x, y| x.partial_cmp(y).unwrap());

        let mut max = -1.0;

        for i in 1..ratios.len() {
            max = f64::max(ratios[i] / ratios[i - 1], max);
        }
        println!("{:.2}", max);
    }
}


