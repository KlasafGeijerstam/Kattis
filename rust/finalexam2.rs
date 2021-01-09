mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let n = inp.next();
    let ans: Vec<char> = (0..n).map(|_| inp.next()).collect();
    let correct = ans
        .iter()
        .skip(1)
        .zip(ans.iter())
        .filter(|(x, y)| x == y)
        .count();
    println!("{}", correct);
}
