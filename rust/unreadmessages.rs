mod fast_input;
use fast_input::{FastInput, FastParse};
use std::collections::HashMap;

fn main() {
    let inp = FastInput::new();
    
    let (people, messages): (usize, usize) = inp.next();
    let mut senders = HashMap::new();
    for i in 1..=messages {
        let sender: usize = inp.next_parsed();
        senders.insert(sender, i);
        let non_senders = people - senders.len();
         
        let mut unread = i * non_senders;
        for &when in senders.values() {
            unread += i - when;
        }
        println!("{unread}");
    }
}
