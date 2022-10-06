mod fast_input;

use fast_input::FastInput;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use std::io::{stdout, BufWriter, Write};

fn main() {
    let inp = FastInput::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let n = inp.next();
    let mut words = HashMap::new();
    let mut all_users = HashSet::new();
    for _ in 0..n {
        let mut iter = inp.next_split();
        let person = iter.next().unwrap();
        all_users.insert(person);
        for word in iter {
            let entry = words.entry(word).or_insert((0, HashSet::new()));
            entry.0 += 1;
            entry.1.insert(person);
        }
    }

    let mut used_by_all: Vec<_> = words
        .iter()
        .filter(|(_, (_, users))| users.len() == all_users.len())
        .map(|(word, (occ, _))| (Reverse(occ), word))
        .collect();

    used_by_all.sort_unstable();
    if used_by_all.is_empty() {
        println!("ALL CLEAR");
    } else {
        for (_, word) in used_by_all {
            writeln!(writer, "{}", word).ok();
        }
    }
}
