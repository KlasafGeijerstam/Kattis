mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let nums: Vec<_> = inp
        .next_line()
        .chars()
        .map(|c| c as usize)
        .collect();
    println!("{}", (nums.iter().sum::<usize>() / nums.len()) as u8 as char);
}
