mod fast_input;
use fast_input::{FastInput};


fn main() {
    let inp = FastInput::new();
    let t = inp.next();
    for _ in 0..t {
        let n = inp.next();
        let mut total_votes = 0;
        let mut cands: Vec<_> = (1..=n).map(|i| {
            let vote: u32 = inp.next();
            total_votes += vote;
            (i, vote)
        }).collect();
        cands.sort_by_key(|(_, votes)| *votes);
        let c1 = cands[cands.len() - 1];
        let c2 = cands[cands.len() - 2];
        if c1.1 > total_votes / 2 {
            println!("majority winner {}", c1.0);
        } else if c1.1 > c2.1 {
            println!("minority winner {}", c1.0);
        } else {
            println!("no winner");
        }
    }
}

