mod fast_input;
use fast_input::FastInput;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Eq, Debug)]
struct CoWorker {
    annoyance: u128,
    annoyance_increase: u128,
}

impl CoWorker {
    fn next_annoyance(&self) -> u128 {
        self.annoyance + self.annoyance_increase
    }

    fn annoy(&mut self) {
        self.annoyance += self.annoyance_increase;
    }
}

impl Ord for CoWorker {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for CoWorker {
    fn eq(&self, other: &Self) -> bool {
        self.next_annoyance() == other.next_annoyance()
    }
}

impl PartialOrd for CoWorker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.next_annoyance().partial_cmp(&other.next_annoyance())
    }
}

fn main() {
    let inp = FastInput::new();

    let (h, c) = inp.next_tuple();
    let mut coworkers = BinaryHeap::with_capacity(c);
    for _ in 0..c {
        let (annoyance, annoyance_increase) = inp.next_tuple();
        coworkers.push(Reverse(CoWorker {
            annoyance,
            annoyance_increase,
        }));
    }

    for _ in 0..h {
        let Reverse(mut least_annoyed) = coworkers.pop().unwrap();
        least_annoyed.annoy();

        coworkers.push(Reverse(least_annoyed));
    }

    let most_annoyed = coworkers
        .iter()
        .map(|Reverse(x)| x)
        .max_by_key(|x| x.annoyance)
        .unwrap();
    println!("{}", most_annoyed.annoyance);
}
