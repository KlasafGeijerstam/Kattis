mod fast_input;

use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let (_, m): (i32, usize) = inp.next_tuple();
    let (mut best_index, mut best_score) = (0, 0);
    let strs: Vec<_> = (0..=m).map(|i| i.to_string()).collect();
    for (i, l) in inp.lines().enumerate() {
        let score = (1..=m)
            .zip(l.split_whitespace())
            .fold(0, |score, (i, given)| {
                let actual = if i % 3 == 0 && i % 5 == 0 {
                    "fizzbuzz"
                } else if i % 3 == 0 {
                    "fizz"
                } else if i % 5 == 0 {
                    "buzz"
                } else {
                    &strs[i]
                };

                if given == actual {
                    score + 1
                } else {
                    score
                }
            });

        if score > best_score {
            best_score = score;
            best_index = i;
        }
    }
    println!("{}", best_index + 1);
}
