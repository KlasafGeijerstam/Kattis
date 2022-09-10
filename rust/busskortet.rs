mod fast_input;
use fast_input::FastInput;
use std::collections::HashMap;

fn search(transactions: u16, amount: u16, target: u16, cache: &mut HashMap<(u16, u16), (u16, u16)>) -> (u16, u16) {
    if let Some(&value) = cache.get(&(transactions, amount)) {
        return value;
    }
    if amount >= target {
        return (transactions, amount);
    }
    for v in [500, 200, 100] {
        let (count, value) = search(transactions + 1, amount + v, target, cache);
        let entry = cache.entry((transactions, amount)).or_insert((count, value));
        if value < entry.1 || (value == entry.1 && count < entry.0) {
            *entry = (count, value);
        }
    }
    cache[&(transactions, amount)]
}

fn main() {
    let inp = FastInput::new();
    let k = inp.next();
    let mut cache = HashMap::new();
    let (count, _) = search(0, 0, k, &mut cache);
    println!("{}", count);
}
