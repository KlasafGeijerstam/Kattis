mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    inp.next_line();
    let mut cards: Vec<u32> = inp.next_as_iter().collect();
    cards.sort_unstable();
    let mut score = cards[0];
    for i in (1..cards.len()).rev() {
        if cards[i - 1] + 1 != cards[i] {
            score += cards[i];
        }
    }

    println!("{}", score);
}
