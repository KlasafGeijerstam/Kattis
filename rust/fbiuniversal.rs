mod inp;
use inp::Inp;

use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let t = inp.next();
    let cmap: HashMap<char, u32> = 
        "0123456789ACDEFHJKLMNPRTVWX".chars().enumerate()
        .map(|(i, c)| (c, i as u32))
        .collect();
    let nums = [2, 4, 5, 7, 8, 10, 11, 13];
    for _ in 0..t {
        let mut l = inp.next_line().trim().split_ascii_whitespace();
        let (k, c) = (l.next().unwrap(), l.next().unwrap());
        let j: u32 = c.chars().take(8).map(|x| match x {
            'B' => '8',
            'G' => 'C',
            'I' => '1',
            'O' => '0',
            'Q' => '0',
            'S' => '5',
            'U' => 'V',
            'Y' => 'V',
            'Z' => '2',
            a => a
        }).enumerate()
        .map(|(i, c)| cmap[&c] * nums[i])
        .sum();

        if j % 27 == cmap[&c.chars().last().unwrap()] {
            println!("{} {}", k, pint(&c[..8]));
        } else {
            println!("{} Invalid", k);
        }

    }
    Ok(())
}

fn pint(s: &str) -> u128 {
    let map: HashMap<_, _> = [
               ('0', '0'),
               ('1', '1'),
               ('2', '2'),
               ('3', '3'),
               ('4', '4'),
               ('5', '5'),
               ('6', '6'),
               ('7', '7'),
               ('8', '8'),
               ('9', '9'),
               ('A', 'A'),
               ('C', 'B'),
               ('D', 'C'),
               ('E', 'D'),
               ('F', 'E'),
               ('H', 'F'),
               ('J', 'G'),
               ('K', 'H'),
               ('L', 'I'),
               ('M', 'J'),
               ('N', 'K'),
               ('P', 'L'),
               ('R', 'M'),
               ('T', 'N'),
               ('V', 'O'),
               ('W', 'P'),
               ('X', 'Q'),]
                   .iter()
                   .cloned()
                   .collect();
    u128::from_str_radix(&s.chars()
                             .map(|x| map[&x])
                             .collect::<String>(),
                        27).unwrap()
}
