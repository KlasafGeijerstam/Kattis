use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::collections::LinkedList;

fn main() -> io::Result<()> {
    let i = read_ints()?; 
    let input = BufReader::new(io::stdin());
    let mut people: Vec<(i32, i32)> = input.lines().map(|x| {
        let l = x.unwrap();
        let mut l = l.split(" ").map(|y| y.parse::<i32>().unwrap());
        (l.next().unwrap(), l.next().unwrap()) 
    }).collect();

    people.sort_unstable_by_key(|x| x.0);
    people.reverse();
    let mut answer: Vec<i32> = Vec::new();
    for _ in 0..i[1] {
        answer.push(0);
    }
    for (m, t) in people.iter() {
        for i in (0..t + 1).rev() {
            if answer[i as usize] == 0 {
                answer[i as usize] = *m;
                break;
            }
        }
    }
    println!("{}", answer.iter().map(|x| *x).sum::<i32>());
    Ok(())
}

fn read_ints() -> Result<Vec<i32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
