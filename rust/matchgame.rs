mod inp;
use inp::Inp;
use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let (x, y) = inp.next_str_tuple();

    let mut offending_index = vec![];
    let mut n1: u128 = 0;
    let mut n2: u128 = 0;
    let mask = [
        0b1111110, //0
        0b0001100, //1
        0b0110111, //2
        0b0011111, //3
        0b1001101, //4
        0b1011011, //5
        0b1111011, //6
        0b0001110, //7
        0b1111111, //8
        0b1011111, //9
    ];

    for (i, (a, b)) in x.chars().zip(y.chars()).enumerate() {
        if a != b {
            offending_index.push(i);
        }
        n1 |= mask[a as usize - '0' as usize] << (i * 7);
        n2 |= mask[b as usize - '0' as usize] << (i * 7);
    }

    let mut ways = 0;
    for &o1 in &offending_index {
        for &o2 in &offending_index {
            for i in 0..7 {
                for j in 0..7 {
                    let of1: u128 = 1 << ((o1 * 7) + i);
                    let of2: u128 = 1 << ((o2 * 7) + j);
                    if n1 & of1 > 0 && n1 & of2 == 0 && ((n1 & !of1) | of2) == n2 {
                        ways += 1;
                    }
                }
            }
        }
    }

    if ways == 1 {
        println!("yes");
    } else {
        println!("no");
    }

    Ok(())
}
