mod fast_input;
use fast_input::{FastInput};
use std::collections::HashSet;
const S: u8 = '*' as u8;
fn main() {
    let inp = FastInput::new();
    let n: usize = inp.next(); 
    let mut words = HashSet::with_capacity(6000000);
    let mut one_digit = HashSet::with_capacity(500000);
    let mut two_digit = HashSet::with_capacity(500000);
    let mut three_digit = HashSet::with_capacity(500000);

    for _ in 0..n {
        let word: Vec<u8> = inp.next_line().chars().map(|x| x as u8).collect();
        words.insert(word.clone());

        // No swaps, perform all three types of digit changes
        for i1 in 0..word.len() {
            let mut new_word = word.clone();
            new_word[i1] = S;
            one_digit.insert(new_word.clone());

            for i2 in i1 + 1..word.len() {
                let mut new_word = new_word.clone();
                new_word[i2] = S;
                two_digit.insert(new_word.clone());
                for i3 in i2 + 1..word.len() {
                    let mut new_word = new_word.clone();
                    new_word[i3] = S;
                    three_digit.insert(new_word);
                }
            }
        }


        for i1 in 1..word.len() {
            let mut new_word = word.clone();
            let tmp = new_word[i1];
            new_word[i1] = new_word[i1 - 1];
            new_word[i1 - 1] = tmp;
            words.insert(new_word.clone());

            // One swap, perform 2 digit changes
            for i1 in 0..word.len() {
                let mut new_word = new_word.clone();
                new_word[i1] = S;
                one_digit.insert(new_word.clone());

                for i2 in i1 + 1..word.len() {
                    let mut new_word = new_word.clone();
                    new_word[i2] = S;
                    two_digit.insert(new_word);
                }
            }

            for i2 in 1..word.len() {
                let mut new_word = new_word.clone();
                let tmp = new_word[i2];
                new_word[i2] = new_word[i2 - 1];
                new_word[i2 - 1] = tmp;
                words.insert(new_word.clone());

                // Two swaps, perform one digit change
                for i1 in 0..word.len() {
                    let mut new_word = new_word.clone();
                    new_word[i1] = S;
                    one_digit.insert(new_word);
                }

                for i3 in 1..word.len() {
                    let mut new_word = new_word.clone();
                    let tmp = new_word[i3];
                    new_word[i3] = new_word[i3 - 1];
                    new_word[i3 - 1] = tmp;
                    words.insert(new_word);
                    // Three swaps, no digit changes
                }
            }
        }
    }
        
    'case: while inp.has_next_line() {
        let orig = inp.next_line();
        let word: Vec<_> = orig.chars().map(|x| x as u8).collect();
        if words.contains(&word) {
            continue
        }
        for i1 in 0..word.len() {
            let mut new_word = word.clone();
            if !word[i1].is_ascii_digit() {
                continue 
            }
            new_word[i1] = S;
            if one_digit.contains(&new_word) {
                continue 'case
            }

            for i2 in i1 + 1..word.len() {
                let mut new_word = new_word.clone();
                if !word[i2].is_ascii_digit() {
                    continue
                }
                new_word[i2] = S;
                if two_digit.contains(&new_word) {
                    continue 'case
                }
                for i3 in i2 + 1..word.len() {
                    let mut new_word = new_word.clone();
                    if !word[i2].is_ascii_digit() {
                        continue
                    }
                    new_word[i3] = S;
                    if three_digit.contains(&new_word) {
                        continue 'case
                    }
                }
            }
        }
        println!("{}", orig);
    }
}

