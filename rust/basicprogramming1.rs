mod inp;
use inp::Inp;

use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new(); 
    let (_, t) = inp.next_tuple();
    let a:Vec<u32> = inp.next_as_iter().collect();
    match t {
        1 => {
            println!("7");
        },
        2 => {
            println!("{}", if a[0] > a[1] {
                "Bigger"
            } else if a[0] == a[1] {
                "Equal"
            } else {
                "Smaller"
            });
        },
        3 => {
            let mut p = vec![a[0], a[1], a[2]];
            p.sort();
            println!("{}", p[1]);
        },
        4 => {
            println!("{}", a.iter().map(|&x| x as u64).sum::<u64>());
        },
        5 => {
            println!("{}", a.iter().filter(|&x| x % 2 == 0).map(|&x| x as u64).sum::<u64>());
        },
        6 => {
            let s: String = a.iter().map(|&x| {
                ('a' as u8 + (x % 26) as u8) as char
            }).collect();
            println!("{}", s);
        },
        _ => {
            let mut visited = vec![false; a.len()];
            visited[0] = true;
            let mut i = 0;
            loop {
                if a[i] as usize >= a.len() {
                    println!("Out");
                    break;
                } else if a[i] as usize == a.len() - 1 {
                    println!("Done");
                    break;
                }
                if visited[a[i] as usize] {
                    println!("Cyclic");
                    break;
                }
                visited[a[i] as usize] = true;
                i = a[i] as usize;

            }

        },
    }
    Ok(())
}
