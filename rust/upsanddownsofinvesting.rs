mod fast_input;
use fast_input::{FastParse, FastInput};

fn is_peak(numbers: &[u32], index: usize, n: usize) -> bool {
    if index < n - 1 {
        return false
    }
    if index + n - 1 >= numbers.len() {
        return false
    }
    for i in index - (n - 1)..index {
        if numbers[i] >= numbers[i + 1] {
            return false;
        }
    }

    for i in index..index + n - 1 {
        if numbers[i] <= numbers[i + 1] {
            return false;
        }
    }

    true
}

fn is_valley(numbers: &[u32], index: usize, m: usize) -> bool {
    if index < m - 1 {
        return false
    }
    if index + m - 1 >= numbers.len() {
        return false
    }
    for i in index - (m - 1)..index {
        if numbers[i] <= numbers[i + 1] {
            return false;
        }
    }

    for i in index..index + m - 1 {
        if numbers[i] >= numbers[i + 1] {
            return false;
        }
    }

    true
}

fn main() {
    let inp = FastInput::new();
    let (s, n, m): (usize, usize, usize) = inp.next();
    let mut prices = Vec::with_capacity(s);
    while inp.has_next_line() {
        inp.next_as_iter().for_each(|price| prices.push(price));
    }
    
    let mut peaks = 0; 
    let mut valleys = 0;
    for i in 0..prices.len() {
        if is_peak(&prices, i, n) {
            peaks += 1;
        } else if is_valley(&prices, i, m) {
            valleys += 1;
        }
    }

    println!("{} {}", peaks, valleys);
}
