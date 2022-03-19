mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let nums: Vec<_> = inp
        .next_line()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let map = |c| if c == 0 { "." } else { "*" };
    for i in (0..4).rev().map(|i| 1 << i) {
        println!(
            "{} {}   {} {}",
            map(nums[0] & i),
            map(nums[1] & i),
            map(nums[2] & i),
            map(nums[3] & i)
        );
    }
}
