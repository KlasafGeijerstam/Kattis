mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    inp.next_line();
    let mut stacks: Vec<(i16, u8)> = inp
        .next_as_iter()
        .enumerate()
        .map(|(i, x)| (x, i as u8))
        .collect();
    let mut moves = Vec::with_capacity(25000);
    loop {
        stacks.sort_unstable_by_key(|(x, _)| -x); 
        if stacks[0].0 == 0 && stacks[1].0 == 0 {
            println!("yes");
            for (a, b) in moves {
                println!("{} {}", a, b);
            }
            return
        } else if stacks[0].0 > 0 && stacks[1].0 == 0 || stacks[1].0 > 0 && stacks[1].0 == 0 {
            println!("no");
            return
        } else {
            stacks[0].0 -= 1;
            stacks[1].0 -= 1;
            moves.push((stacks[0].1 + 1, stacks[1].1 + 1));
        }
    }
}