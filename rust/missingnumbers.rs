mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let nums: Vec<u32> = (0..inp.next()).map(|_| inp.next()).collect();
    let mut has_missed = false;
    for n in (1..*nums.last().unwrap()).filter(|n| !nums.contains(&n)) {
        println!("{}", n);
        has_missed = true;
    }

    if !has_missed {
        println!("good job");
    }
}
