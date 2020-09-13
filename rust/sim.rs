mod inp;
use inp::Inp;

use std::io::Result;
use std::collections::LinkedList;

fn main() -> Result<()> {
    let mut inp = Inp::new(); 
    let t = inp.next();
    for _ in 0..t {
        let mut current: LinkedList<char> = LinkedList::new();
        let mut total: LinkedList<char> = LinkedList::new();
        let line = inp.next_line();
        let mut at_front = true;
        for c in line.trim().chars() {
            match c {
                '[' => {
                    at_front = false;
                    for c in current.iter().rev() {
                        total.push_front(*c);
                    }
                    current.clear();
                },
                ']' => {
                    if at_front {
                        continue;
                    }
                    at_front = true;
                    for c in current.iter().rev() {
                        total.push_front(*c);
                    }
                    current.clear();
                },
                '<' => {
                    if at_front {
                        total.pop_back();  
                    } else {
                        current.pop_back();
                    }
                },
                x => {
                    if at_front {
                        total.push_back(x);
                    } else {
                        current.push_back(x);
                    }
                }
            }
        }
        print!("{}", current.iter().collect::<String>());
        println!("{}", total.iter().collect::<String>());
    }
    Ok(())
}

