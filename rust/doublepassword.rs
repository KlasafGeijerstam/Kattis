mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let ans: u32 = inp
        .next_line()
        .chars()
        .zip(inp.next_line().chars())
        .map(|(a, b)| if a == b { 1 } else { 2 })
        .product();
    println!("{}", ans);
}
