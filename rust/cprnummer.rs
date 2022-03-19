mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let check = [4, 3, 2, 7, 6, 5, 4, 3, 2, 1];
    let sum = inp
        .next_line()
        .replace("-", "")
        .chars()
        .map(|c| c as u32 - '0' as u32)
        .zip(check.iter())
        .fold(0, |acc, (a, &b)| acc + a * b);

    println!("{}", (sum % 11 == 0) as u32);
}
