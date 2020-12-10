mod fast_input;
use fast_input::{FastInput};

fn main() {
    let inp = FastInput::new();
    for _ in 0..inp.next() {
        let (n, m, l) = inp.next_triple::<usize, u32, u32>();

        let mut dominoes: Vec<_> = (0..n).map(|_| {
            Vec::with_capacity(100)
        }).collect();

        // Connect
        for c in 0..m {
            let (x, y) = inp.next_tuple::<usize, usize>();            
            dominoes[x - 1].push(y - 1);
        }
        
        let mut flip = vec![false; n];
        let mut flipped = 0;
        let mut stack = Vec::with_capacity(10000);
        for _ in 0..l {
            let j: usize = inp.next();
            stack.clear();
            stack.push(j - 1);

            while !stack.is_empty() {
                let current = stack.pop().unwrap();
                if flip[current] {
                    continue
                }
                flipped += 1;
                flip[current] = true;
                for n in &dominoes[current] {
                    stack.push(*n);
                }
            }
        }
        println!("{}", flipped);
    }
}
