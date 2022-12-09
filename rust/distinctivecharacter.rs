mod fast_input;
use fast_input::FastInput;
use std::collections::VecDeque;

fn main() {
    let inp = FastInput::new();
    let (n, k) = inp.next_tuple();
    let total_combinations = 2u32.pow(k);

    let mut distances = vec![total_combinations; total_combinations as usize];
    let mut characters: VecDeque<_> = (0..n)
        .map(|_| u32::from_str_radix(inp.next_line(), 2).unwrap())
        .collect();

    for &character in &characters {
        distances[character as usize] = 0;
    }

    let mut last_popped = 0;
    while let Some(current) = characters.pop_front() {
        last_popped = current;

        for one_mod in (0..k).map(|i| current ^ (1 << i)) {
            if distances[one_mod as usize] > distances[current as usize] + 1 {
                characters.push_back(one_mod);
                distances[one_mod as usize] = distances[current as usize] + 1;
            }
        }
    }

    println!("{:0width$b}", last_popped, width = k as usize);
}
