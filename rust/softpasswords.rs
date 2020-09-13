mod inp;
use inp::Inp;

use std::io::Result;
use std::collections::LinkedList;

fn main() -> Result<()> {
    let mut inp = Inp::new(); 
    let s: LinkedList<char> = inp.next_line().trim().chars().collect();
    let mut p: LinkedList<char> = inp.next_line().trim().chars().collect();
    if s.len() == p.len() {
        //Identical
        if s == p {
            println!("Yes");
            return Ok(());
        }

        //Reverse case
        for c in p.iter_mut() {
            if c.is_alphabetic() {
                if c.is_lowercase() {
                    *c = c.to_uppercase().next().unwrap();
                } else {
                    *c = c.to_lowercase().next().unwrap();
                }
            }
        }

        if s == p {
            println!("Yes");
        } else {
            println!("No");
        }

    } else if p.len() == s.len() - 1 {
        for i in 0..=9 {
            let c = ('0' as u8 + i) as char;
            p.push_front(c);
            if s == p {
                println!("Yes");
                return Ok(());
            }
            p.pop_front();
            p.push_back(c);
            if s == p {
                println!("Yes");
                return Ok(());
            }
            p.pop_back();
        }

        println!("No");

    } else {
        println!("No");
    }
    

    Ok(())
}
