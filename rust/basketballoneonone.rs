use std::io;

fn main() -> io::Result<()> {
    let mut l = String::new();
    io::stdin().read_line(&mut l)?;
    let s: Vec<char> = l.chars().collect();
    let mut a = 0;
    let mut b = 0;
    for c in (0..l.len()).step_by(2) {
        let score = s[c + 1].to_string().parse::<i32>().unwrap();
        match s[c] {
            'A' => a += score,
            'B' => b += score,
            _ => panic!("Oh noes!")
        }

        if a > 10 && i32::abs(a - b) >= 2 {
            println!("A");
            break;
        } else if b > 10 && i32::abs(a - b) >= 2 {
            println!("B");
            break;
        }
    }

    Ok(())
}
