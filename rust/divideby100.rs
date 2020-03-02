
use std::io;

fn main() -> io::Result<()> {
    let mut on = String::new();
    let mut om = String::new();
    io::stdin().read_line(&mut on)?;
    io::stdin().read_line(&mut om)?;
    let n = on.trim();
    let m = om.trim();

    if n.len() < m.len() {
        let zeros = m.len() - n.len();
        print!("0."); 
        let s: String = std::iter::repeat('0').take(zeros - 1).collect();
        print!("{}", s);
        let mut nonz = 0;
        for (i, c) in n.chars().enumerate() {
            if c != '0' {
                nonz = i + 1;
            }
        }
        let postfix: String = n.chars().take(nonz + 1).collect();
        println!("{}", postfix);
        
    } else {
        let zeros = m.len() - 1;
        let prefix: String = n.chars().take(n.len() - zeros).collect();
        if prefix.len() == 0 {
            print!("0");
        } else {
            print!("{}", prefix);
        }
        
        //Find zeros
        let mut nonz = 0;
        for (i, c) in n.chars().skip(n.len() - zeros).enumerate() {
            if c != '0' {
                nonz = i + 1;
            }
        }
        let postfix: String = n.chars().skip(n.len() - zeros).take(nonz).collect();
        if postfix.len() > 0 {
            println!(".{}", postfix);
        } else {
            println!("{}", postfix);
        }
    }
    Ok(())
}
