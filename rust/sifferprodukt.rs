mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    println!("{}", calc(inp.next()));
}

fn calc(num: u32) -> u32 {
    let numbers: Vec<_> = num
        .to_string()
        .chars()
        .filter(|&c| c != '0')
        .map(|c| (c as u8 - '0' as u8) as u32)
        .collect();
    if numbers.len() == 1 {
        numbers[0] as u32
    } else {
        calc(numbers.iter().fold(1, |a, c| a * c))
    }
}
